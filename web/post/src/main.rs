//
// Sample rocket application to handle a POST request
//
use rocket::fs::FileServer;
use rocket::launch;
use rocket::form::Form;
use rocket::post;
use rocket::FromForm;
use rocket::routes;

// Add a dervie FromForm so the data can be marshalled from the form
// into a structure
#[derive(FromForm)]
struct FormData {
    query: String
}

// The post macro decorates the function so that it will accept a POST
// requests from the submit target where the data structure is named
// query_from
#[post("/submit", data="<query_form>")]
async fn handle_submit(query_form: Form<FormData>) -> String {
    let query_data = query_form.into_inner();
    format!("Submitted: {}", query_data.query).to_string()
}

// Main entry point for a rocket application
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static")) // Serve files from the "static" directory
        .mount("/", routes![handle_submit]) // Route queries to "submit" to the handle_submit function
}
