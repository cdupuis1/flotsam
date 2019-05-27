//
// simple_http
//
// Simplest web server in golang
package main

import (
	"fmt"
	"log"
	"net/http"
)

// Handles HTTP request received from client
func myHandler(w http.ResponseWriter, r *http.Request) {

	// Client will receive hello world message
	fmt.Fprintf(w, "Hello web server!")
}

func main() {
	// Set the http handler function for the root to the
	// myHandler function
	http.HandleFunc("/", myHandler)

	// Note that program will exit abruptly if tcp port is
	// taken up already
	log.Fatal(http.ListenAndServe(":8000", nil))
}
