//
// Small program to align a file to certain size with zeroes
//
use std::fs;
use clap::{Parser};
use std::process;

// Used clap crate to parse command line arguments
#[derive(Parser)]
struct Args {
    /// Value to align binary to
    #[arg(short='a')]
    alignment: usize,

    /// Name of file
    #[arg(short='n')]
    filename: String,
}

fn main() {
    // Call clap to do the parsing
    let args = Args::parse();

    // Read file contents into a u8 vector.  We use unwrap_or_else() to handle
    // and errors with a closure function
    let mut buffer: Vec<u8> = fs::read(args.filename.as_str()).unwrap_or_else(|err| {
        eprintln!("File open failed: {}", err);
        process::exit(1);
    });

    let remainder = buffer.len() % args.alignment;

    if remainder != 0 {
        // The get how many bytes we need for alignment, subject the remainder
        // from the alignment number
        let bytes_to_add = args.alignment - remainder;
        println!("Add {} bytes to align", bytes_to_add);

        // Use buffer extend to add a vector of 0s to the end of the buffer
        buffer.extend(vec![0u8; bytes_to_add]);

        // Actually write the results back out
        let result = fs::write(args.filename.as_str(), buffer);

        // Use match to handle any results
        match result {
            Err(e) => eprintln!("Error was {}", e),
            Ok(()) => println!("File successfully written")
        }
    } else {
        println!("File already {} byte aligned", args.alignment);
    }
}
