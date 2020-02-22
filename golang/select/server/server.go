//
// Server that listens on a unix socket and sends a random number back to the
// caller in a go routine
//
package main

import (
	"fmt"
	"log"
	"net"
	"os"
	"strconv"
	"sync"
	"time"
)

const sockAddr = "/tmp/random.sock"

// Handles an incoming connection from accept
func handleConnection(conn net.Conn) {
	// Create buffer to read from socket
	buf := make([]byte, 1024)

	// Read the message
	numbytes, err := conn.Read(buf)
	if err != nil {
		log.Println(err)
		return
	}

	// Set "data" equal to the number of bytes read
	data := buf[0:numbytes]

	fmt.Println("Read: \"" + string(data) + "\"")

	// Sleep intentionally here to demonstrate using up all the worker
	// goroutines
	time.Sleep(1 * time.Second)
}

func main() {
	var wg sync.WaitGroup

	// Create buffered channel where it accepts an anonymous function to
	// create a set of worker functions
	workers := 10
	pool := make(chan func(net.Conn), workers)

	// Prime the pool channel with functions
	for i := 0; i < workers; i++ {
		pool <- func(conn net.Conn) {
			handleConnection(conn)
		}
	}

	// Remove any previous socket
	err := os.RemoveAll(sockAddr)
	if err != nil {
		log.Fatal(err)
	}

	// Recreate the socket
	sock, err := net.Listen("unix", sockAddr)
	if err != nil {
		log.Fatal(err)
	}

	numRun := 0

	for {
		// Wait for an incoming message
		conn, err := sock.Accept()
		if err != nil {
			log.Fatal(err)
			break
		}

		select {
		// Take one of the function handlers from the pool
		case f := <-pool:
			wg.Add(1)
			// Handle reading from socket in go routine
			go func(conn net.Conn) {
				// Call the handler which will handle the message
				f(conn)

				// Return the function to the pool
				pool <- f

				// Tell the wait group we're done so it will proceed
				wg.Done()
			}(conn)
			numRun++
		// If there are no more handlers print an error
		default:
			fmt.Println("Cannot handle request")
		}

		fmt.Println("Number of goroutines spun: " + strconv.Itoa(numRun))
	}

	//
	// Most likely won't get here unless Accept fails
	//

	// Wait for all goroutines to end
	wg.Wait()
	sock.Close()
}
