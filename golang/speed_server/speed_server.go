//
// speed_server
//
// Web service to return binary data to test network
// speed
//
package main

import (
	"fmt"
	"log"
	"net/http"
	"strconv"
)

const port string = "8000"

// Handles HTTP request received from client
func myHandler(w http.ResponseWriter, r *http.Request) {
	var size int
	var tmp []string
	var sizeStr string
	var ok bool
	var err error

	tmp, ok = r.URL.Query()["size_in_kb"]

	// Note that tmp is a string slice so to check
	// the first argument we need to reference element 0
	if !ok || len(tmp[0]) < 1 {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprintf(w, "Bad argument, use size_in_kb")
		return
	}

	sizeStr = tmp[0]
	size, err = strconv.Atoi(sizeStr)

	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprint(w, "Bad number")
		return
	}

	// Make size argument number of bytes
	size = size * 1024

	// Create zeroized data to send back
	buf := make([]byte, size)

	// Write back the data as a binary stream.  Note that
	// the header must be set before the write
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Write(buf)
}

func main() {
	// Set the http handler function for the root to the
	// myHandler function
	http.HandleFunc("/", myHandler)

	fmt.Println("Listening on port " + port)

	// Note that program will exit abruptly if tcp port is
	// taken up already
	log.Fatal(http.ListenAndServe(":"+port, nil))
}
