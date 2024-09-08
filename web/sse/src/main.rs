//
// Sample app to serve up simple data over a SSE connection
//
use rocket::launch;
use rocket::routes;
use rocket::get;
use rocket::response::stream::{Event, EventStream};
use tokio::time;
use std::time::Duration;
use rocket::tokio;
use rocket::serde::Serialize;
use rocket::serde::Deserialize;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

// Add Serialize/Deserialize decorators so the struct can be converted to JSON
#[derive(Serialize, Deserialize)]
struct Data<'a> {
    message: &'a str,
    id: i32,
}

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
fn sse() -> EventStream![] {
    // Define the struct to derive the JSON to send
    let mut the_data = Data {
        message: "The id is",
        id: 0
    };

    // Define the SSE stream event
    EventStream! {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            let json_string = serde_json::to_string(&the_data).unwrap();

            // Format and send the SSE event.  yield is used here to pause
            // the event generator so that the event can be sent
            yield Event::data(json_string);
            the_data.id += 1;
            interval.tick().await;
        }
    }
}

#[launch]
fn rocket() -> _ {
    // Create the rocket application.  Note that we attach a CORS fairing.  A
    // fairing is a middleware that executes at certain points in the rocket
    // lifecycle.  In our case, we want CORS headers to be added to each SSE
    // response
    rocket::build()
        .attach(CORS)
        .mount("/", routes![sse])
}
