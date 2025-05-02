//
// Test driver program to run calls from ollama-lib
//
use ollama_lib::send_ollama_query;
use ollama_lib::OllamaData;

fn main() {
    let ollama_data = OllamaData {
        model: String::from("llama3"), // Replace with appropriate model name
        prompt: String::from("Why does gold have the chemical symbol Ag?"), // Replace with which ever you'd like
        stream: false // Send reply in one shot
    };

    println!("Calling send_ollama_query()");
    let result = send_ollama_query(ollama_data, "10.0.0.22");
    println!("{}", result.unwrap());
}
