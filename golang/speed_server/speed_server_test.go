//
// speed_server_test.go
//
// Unit test to make http request to grab data from server
package main

import (
	"fmt"
	"io/ioutil"
	"net"
	"net/http"
	"strconv"
	"testing"
)

const test_port string = "8000"

// Main test toutine
func TestSpeedServer(t *testing.T) {
	var err error
	var resp *http.Response

	// First test that someone is listening on test_port 8000
	_, err = net.Listen("tcp", ":"+test_port)
	if err == nil {
		t.Error("Could bind to port " + test_port + " so server not running")
		return
	}

	// Request 2MB from server
	resp, err = http.Get("http://localhost:" + test_port + "/?size_in_kb=2048")

	// 200 is HTTP status OK
	if err != nil && resp.Status == "200" {
		t.Error("HTTP Get Failed")
		return
	}

	// Print HTTP headers
	for k, v := range resp.Header {
		fmt.Print(k)
		fmt.Print(": ")
		fmt.Println(v)
	}

	// Check for Access-Control-Allow-Origin header
	if _, ok := resp.Header["Access-Control-Allow-Origin"]; !ok {
		t.Error("Access-Control-Allow-Origin header not found")
		return
	}

	// Read the reply into a byte slice
	var tmpBuf []byte
	tmpBuf, err = ioutil.ReadAll(resp.Body)

	if err != nil {
		t.Error("Error reading response data")
		return
	}

	// Remove the header
	tmpBuf = tmpBuf[9:]
	fmt.Println("Returned buffer len: " + strconv.Itoa(len(tmpBuf)))
}
