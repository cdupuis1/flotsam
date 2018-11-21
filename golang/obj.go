package main

import (
	"fmt"
	"strconv"
)

type Person struct {
	First_name string
	Last_name  string
}

type Clown struct {
	Person
	is_funny bool
}

func (c Clown) IsFunny() {
	fmt.Println(c.First_name + " " + c.Last_name + " funny=" + strconv.FormatBool(c.is_funny))
}

func main() {
	chad := Clown{Person{"Chad", "Dupuis"}, false}
	chad.IsFunny()
}
