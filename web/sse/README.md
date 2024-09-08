# SSE examples

This set of code shows how to use server side events (SSE) in both Python using a flask application and rust using the rocket framework. My original goal was to just use rust but python is easier to implement in this case so I implemented in python first to see how SSE works and then moved on to the rust example.

The major files are:

- sse_generator.py - Flash application definition
- data_python.html - Accompanying HTML/Javascript that works with sse_generator.py
- main.rs - Rocket application implementation of SSE
- data_rust.html - Accompanying HTML/Javascript that works with main.rs
- boot_python_sse.sh - Starts the virtual environment with flask and gunicorn packages


