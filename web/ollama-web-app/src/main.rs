//
// Application that gets a post form from a web page, sends the request to the
// ollama service and then pushes the response back to the webpage using sse.
//
use rocket::launch;
use rocket::post;
use rocket::routes;
use rocket::get;
use rocket::response::stream::{Event, EventStream};
use rocket::tokio;
use rocket::serde::Serialize;
use rocket::serde::Deserialize;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fs::FileServer;
use rocket::form::Form;
use rocket::FromForm;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc;
use ollama_lib::{OllamaData, send_ollama_query};

// Add Serialize/Deserialize decorators so the struct can be converted to JSON
#[derive(Serialize, Deserialize)]

// Add a dervie FromForm so the data can be marshalled from the form
// into a structure
#[derive(FromForm)]
struct FormData {
    query: String
}

// Format of the data to send back to the web page
#[derive(Serialize, Deserialize)]
struct ResponseData {
    rsp_data: String
}

//
// Global mutexes to allow the /sse and /submit handlers to communicate since
// we cannot pass a value to either
//
static GLOBAL_SENDER: Lazy<Mutex<Option<Sender<String>>>> = Lazy::new(|| {
    Mutex::new(None)
});

// Store the receiver separately to avoid mutex deadlock
static GLOBAL_RECEIVER: Lazy<Mutex<Option<Receiver<String>>>> = Lazy::new(|| {
    Mutex::new(None)
});

//
// Define a CORS (cross origin resource sharing) headers so that the web page can
// get responses from a differnet origin that where the web page came from.  In this
// case the web page is a file so getting a SSE response from the rocket application
// will cause the web browser to block the request unless we add these headers
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

//#[get] bindes the sse function to GET requests for /sse
#[get("/sse")]
async fn sse() -> EventStream![] {
    // Define the SSE stream event
    EventStream! {
        loop {
            let mut ollama_data = OllamaData {
                model: String::from("llama3"), // Replace with appropriate model name
                prompt: String::from(""), // Replace with which ever you'd like
                stream: false // Send reply in one shot
            };

            let rx = {
                let mut rx_guard = GLOBAL_RECEIVER.lock().await;
                rx_guard.take()
            };

            if let Some(mut receiver) = rx {
                if let Some(message) = receiver.recv().await {
                    ollama_data.prompt = message;
                } else {
                    println!("Channel closed");
                    break;
                }

                // Put the receiver back when done outside the loop
                let mut rx_guard = GLOBAL_RECEIVER.lock().await;
                *rx_guard = Some(receiver);
            }

            // Send the request to the ollama server
            //
            // FIXME: For some reason if the web page reloads, we will continually
            //        send some type of query to the ollama server until the web page
            //        submits a query
            let result = send_ollama_query(ollama_data.clone(), "10.0.0.22").await;

            // Now that we have the response format the datat to send back to
            // client
            let the_rsp_data = ResponseData {
                rsp_data: result.expect("Error sending request").clone()
            };

            let json_string = serde_json::to_string(&the_rsp_data).unwrap();
            
            // Format and send the SSE event.  yield is used here to pause
            // the event generator so that the event can be sent
            yield Event::data(json_string);
        }
    }
}

// The post macro decorates the function so that it will accept a POST
// requests from the submit target where the data structure is named
// query_from
#[post("/submit", data="<query_form>")]
async fn handle_submit(query_form: Form<FormData>) -> String {
    let query_data = query_form.into_inner();

    // Clone the sender so we don't need to remove it from GLOBAL_CHANNEL
    let tx_clone = {
        let channel_guard = GLOBAL_SENDER.lock().await;
        match &*channel_guard {
            Some(tx) => Some(tx.clone()),
            None => None,
        }
    };

    // The if let here is a concise form of a match against tx_clone:
    //
    // match (tx_clone) {
    //     Some(tx) => {do stuff with tx},
    //     None => {print an error or something}
    //
    // Instead of the match, we use the if Some(tx) to extract tx from
    // tx_clone() and then use it
    if let Some(tx) = tx_clone {
        match tx.send(query_data.query).await {
            Ok(_) => {},
            Err(e) => println!("Failed to send: {}", e),
        }
    } else {
        println!("Channel not initialized!");
    }

    "Submitted!".to_string()
}

// Initialize the global sender and receiver
async fn init_async() {
    // Declare the shared sender/receiver
    let (tx, rx) = mpsc::channel(100);
    
    // Store sender and receiver separately
    let mut channel_guard = GLOBAL_SENDER.lock().await;
    
    *channel_guard = Some(tx);
    drop(channel_guard);
    
    let mut rx_guard = GLOBAL_RECEIVER.lock().await;
    *rx_guard = Some(rx);
}

#[launch]
async fn rocket() -> _ {
    // Initial the global channel and mutexes to protect them so the /sse and
    // /submit callbacks can communicate.  Note that the function that uses this
    // must be declared async so we can call "await" on init_async().  If we do
    // not cal "await" the function does nothing
    init_async().await;

    // Create the rocket application.  Note that we attach a CORS fairing.  A
    // fairing is a middleware that executes at certain points in the rocket
    // lifecycle.  In our case, we want CORS headers to be added to each SSE
    // response
    rocket::build()
        .attach(CORS)
        .mount("/", routes![sse])
        .mount("/", FileServer::new("static")) // Serve files from the "static" directory
        .mount("/", routes![handle_submit]) // Route queries to "submit" to the handle_submit function
}
