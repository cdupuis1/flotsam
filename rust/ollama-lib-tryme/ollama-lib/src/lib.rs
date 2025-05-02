//
// Send a ollama query in one function
//
use std::fmt;
use serde::{Serialize, Deserialize};
use serde_json;
use std::io::Read;
use serde_json::Value;

// List of errors the library can return
//
// derive(Debug) is needed to implement unwrap
#[derive(Debug)]
pub enum OllamaLibError {
    PostError,
    HttpStatusError,
    ResponseNotFoundError,
    NoDataReadError,
}

// We need to implement fmt::Display if we want unwrap on the data type to work
impl fmt::Display for OllamaLibError {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OllamaLibError::PostError => write!(f, "Post failed"),
            OllamaLibError::HttpStatusError => write!(f, "HTTP status error"),
            OllamaLibError::ResponseNotFoundError => write!(f, "Response data not found"),
            OllamaLibError::NoDataReadError => write!(f, "No data read from network socket")
        }
    }
}

// Result type to return either a string or the error
pub type OllamaLibResult = Result<String, OllamaLibError>;

// This is the format of an ollama request.  Note that all members need to be
// set to "pub" for an including program to access
#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaData {
    pub model: String,
    pub prompt: String,
    pub stream: bool
}

// Main function to send the query to the ollama server
//
// ollama_data - Struct to form the ollama query
// ip_addr - IP address where ollama server is being run from
//
// Returns either the result string or an error of the type OllamaLibResult
pub fn send_ollama_query(mut ollama_data: OllamaData, ip_addr: &str) -> OllamaLibResult {
    // Always set the stream to false so we get the response in one string
    ollama_data.stream = false;

    // Convert struct to json string
    let json_string = serde_json::to_string(&ollama_data).unwrap();
    let rsp;

    let ollama_api_url = format!("http://{}:11434/api/generate", ip_addr);
    
    println!("Ollama request data={} url={}\n", json_string, ollama_api_url);

    // Send request.  Note request is block so process will block while waiting for
    // replay
    let client = reqwest::blocking::Client::new();
    let mut response = client.post(ollama_api_url)
        .header("Content-Type", "application/json")
        .body(json_string)
        .send();

    // Use & here to take a reference to response so it is not moved and causes
    // an issue when trying to access response in the match statement
    match &response {
        Ok(resp) => {
            // Check response status
            if resp.status().is_success() {

                let mut buffer: Vec<u8> = vec![0; 16553];
                let bytes_read = response.as_mut().expect("Network read failed").read(&mut buffer).unwrap();
                    
                if bytes_read == 0 {
                    rsp = Err(OllamaLibError::NoDataReadError);
                }
                else {
                    // Convert u8 buffer to string for printing
                    let rsp_string = String::from_utf8_lossy(&buffer[..bytes_read]);

                    // Extract 'response' field from the JSON and print it
                    let json_obj: Value = serde_json::from_str(&rsp_string).expect("Failed to parse JSON");
                    if let Some(ollama_rsp) = json_obj.get("response").and_then(|v| v.as_str()) {
                        rsp = Ok(ollama_rsp.to_string());
                    } else {
                        println!("Field 'response' not found or not a string");
                        rsp = Err(OllamaLibError::ResponseNotFoundError);
                    }
                }
            } else {
                eprintln!("Failed response status {}", resp.status());
                rsp = Err(OllamaLibError::HttpStatusError);
               
            }
        }
        Err(err) => {
            eprintln!("Response error: {}", err);
            rsp = Err(OllamaLibError::PostError);
            
        }
    }

    rsp
}
