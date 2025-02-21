//
// Program to send a request to ollama in "serve" mode and then print the
// result
//
use serde::{Serialize, Deserialize};
use serde_json;
use std::io::Read;
use serde_json::Value;

// This is the format of an ollama request
#[derive(Serialize, Deserialize, Debug)]
struct OllamaData {
    model: String,
    prompt: String,
    stream: bool
}

fn main() {
    let ollama_data = OllamaData {
        model: String::from("llama3"), // Replace with appropriate model name
        prompt: String::from("who shot mckinley?"), // Replace with which ever you'd like
        stream: false // Send reply in one shot
    };

    // Convert struct to json string
    let json_string = serde_json::to_string(&ollama_data).unwrap();  
    println!("Ollama request data={}\n", json_string);

    // Send request.  Note request is block so process will block while waiting for
    // replay
    let client = reqwest::blocking::Client::new();
    let mut response = client.post("http://10.0.0.57:11434/api/generate")
        .header("Content-Type", "application/json")
        .body(json_string)
        .send();

    // Use & here to take a reference to response so it is not moved and causes
    // an issue when trying to access response in the match statement
    match &response {
        Ok(resp) => {
            // Check response status
            if resp.status().is_success() {
                loop {
                    let mut buffer: Vec<u8> = vec![0; 4096];

                    //bytes_read = response.as_mut().expect("Network read failed").read(&mut buffer).unwrap();
                    let bytes_read = response.as_mut().expect("Network read failed").read(&mut buffer).unwrap();
                    
                    if bytes_read == 0
                    {
                        break;
                    }

                    // Convert u8 buffer to string for printing
                    let rsp_string = String::from_utf8_lossy(&buffer[..bytes_read]);

                    // Extract 'response' field from the JSON and print it
                    let json_obj: Value = serde_json::from_str(&rsp_string).expect("Failed to parse JSON");
                    if let Some(ollama_rsp) = json_obj.get("response").and_then(|v| v.as_str()) {
                        println!("{}", ollama_rsp);
                    } else {
                        println!("Field 'response' not found or not a string");
                    }
                }
            }
            else {
                eprintln!("Failed response status {}", resp.status());
            }
        }
        Err(err) => {
            eprintln!("Response error: {}", err);
        }
    }

    ()
}
