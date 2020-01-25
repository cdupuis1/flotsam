//
// goroutine.go
//
// Demonstrates basic concurrent execution using goroutines by printing 10
// random numbers
//
package main

import (
	"fmt"
	"math/rand"
	"strconv"
	"sync"
	"time"
)

func printNum(num int) {
	fmt.Println("The number is " + strconv.Itoa(num))
}

func main() {
	var wg sync.WaitGroup

	// Add 10 go routines to our wait group since we'll loop 10 times calling
	// printNum
	wg.Add(10)

	// Create a new random source using the current Unix time
	s1 := rand.NewSource(time.Now().UnixNano())

	// Seed new random source
	r1 := rand.New(s1)

	// Set off 10 independent goroutines.  Note we put the argument to the
	// closure outside the function definition
	for i := -0; i < 10; i++ {
		go func(num int) {
			printNum(num)
			wg.Done()
		}(r1.Intn(1000))
	}

	// Wait for all goroutines to end
	wg.Wait()
}
