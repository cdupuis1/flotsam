//
// Basic shell written in Rust
//
use std::io::{self, Write};
use std::process;
use std::process::Command;
use execute::Execute;
use std::env;

// Remove newline from end of string if it exists
//
// str_to_chomp - Input string
//
// Returns string with newline removed (or same string if none exists)
fn chomp(mut str_to_chomp: String) -> String
{
    match str_to_chomp.chars().last() {
        // last() returns what is known as an option.  It may return data or
        // it may return nothing which is a value option of "None"
        Some(last_char) => {
            if last_char == '\n' {
                str_to_chomp.truncate(str_to_chomp.len() - 1);
            }
        },
        None => {}
    }

    str_to_chomp
}

// Processes an internal shell command; i.e. not an executable
//
// internal_cmd: Vector of the command and any arguments. internal_cmd[0] is
//               the command to process
//
// Returns true if a command was executes, false otherwise
fn process_internal_cmd(internal_cmd: Vec<&str>) -> bool
{
    let mut retval:bool = false;

    match chomp(internal_cmd[0].to_string()).as_str() {
        // Exit the shell
        "exit" | "quit" => {
            process::exit(0);
        },
        // Change the current working directory in the shell
        "chdir" => {
            let dir: String = chomp(internal_cmd[1].to_string());

            match env::set_current_dir(dir) {
                // set_current_dir returns a Result enum which can either be
                // Ok() which means that the process succeeded or Err() which
                // contains the error string
                Ok(_) => {},
                Err(e) => {
                    println!("{}", e);
                }
            }
            println!("");
            retval = true;
        },
        _ => ()
    }

    retval
}

fn main()
{
    loop
    {
        let mut line = String::new(); 
        print!("> ");

        // Oddity of Rust; need to explicitly flush the io buffer to the tty
        // before issuing the read_line else our prompt won't print beforehand
        io::stdout().flush().unwrap();

        // Read the command
        std::io::stdin().read_line(&mut line).unwrap();

        // Split the string into a vector using a space
        let str_split: Vec<&str> = line.split(' ').collect();

        // Extract command
        let mut path: String = str_split[0].to_string();
        path = chomp(path);

        // If just Enter is pressed, proces the next command 
        if path.is_empty() {
            println!("");
            continue;
        }

        // Check for built in commands.  Note have to clone this since not
        // doing so would move ownership of str_split to the function which
        // means that no subsequent code in this scope could use it then
        if process_internal_cmd(str_split.clone()) {
            continue;
        }

        // Create the Command object
        let mut the_command = Command::new(path);

        // Create a slice of the line that read in but without the first string
        let args = &str_split[1..];
        for arg in args {
            let mut tmp_str: String = arg.to_string();
            tmp_str = chomp(tmp_str);
            // Add argument to the command to execute
            the_command.arg(tmp_str);
        }

        // Execute the command and print the output to the current tty
        match the_command.execute_output() {
            Ok(_) => {},
            Err(error) => {
                println!("Error: {}", error);
            }
        }

        println!("");
    }
}
