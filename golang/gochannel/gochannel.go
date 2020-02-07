//
// goroutines.go
//
// Program to show example of how to use gochannels

package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"sync"
	"time"
)

// Function for first goroutine which reads a string from the terminal
func waitForInput(pipe chan string) {
	var line string
	var err error

	buf := bufio.NewReader(os.Stdin)

	for {
		fmt.Print("Enter a string (or \"exit\" to quit): ")
		line, err = buf.ReadString('\n')
		if err != nil {
			fmt.Println(err)
			return
		}

		// Send the string that was entered to the other goroutine
		pipe <- line

		if line == "exit\n" {
			// Close the pipe on exit to avoid a deadlock
			close(pipe)
			break
		}

		// Sleep for a 500ms to allow the other go routine print the string
		time.Sleep(500 + time.Millisecond)
	}
}

// Second goroutine to get a string from the channel and print it
func printTheInput(pipe chan string) {
	var lineFromChannel string

	for {
		// Block and then wait for the string
		lineFromChannel = <-pipe
		if lineFromChannel == "exit\n" {
			break
		}
		// TrimSuffix chomps the newline off the end of the string
		fmt.Println("Got the string \"" + strings.TrimSuffix(lineFromChannel, "\n") + "\" from the channel")
	}
}

func main() {
	var wg sync.WaitGroup

	// Create the channel which will be used to send a string from one
	// goroutine to another
	pipe := make(chan string)

	// Wait for both go routines
	wg.Add(2)

	fmt.Println("Starting printTheInput()")
	go func(thechan chan string) {
		printTheInput(thechan)
		wg.Done()
	}(pipe)
	fmt.Println("Starting waitForInput()")
	go func(thechan chan string) {
		waitForInput(thechan)
		wg.Done()
	}(pipe)

	// Wait for goroutines to be done
	wg.Wait()
}
