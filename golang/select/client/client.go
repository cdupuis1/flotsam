//
// client.go
//
// Connects to unix socket and sends a short message

package main

import (
	"fmt"
	"net"
	"os"
)

const sockAddr = "/tmp/random.sock"

func main() {
	c, err := net.Dial("unix", sockAddr)
	if err != nil {
		fmt.Println("Could not connect to socket " + sockAddr)
		os.Exit(1)
	}

	_, err = c.Write([]byte("Hello from client!"))
	if err != nil {
		fmt.Println("Write failed")
	}

	c.Close()
}
