package main

import (
    "fmt"
    "log"
    "os"
	"bufio"
	"strconv"
)

func main() {
// open file
    f, err := os.Open("input.txt")
    if err != nil {
        log.Fatal(err)
    }
    // remember to close the file at the end of the program
    defer f.Close()

    // read the file line by line using scanner
    scanner := bufio.NewScanner(f)
	var counter = 50
	var pass = 0
	var pass2 = 0
    for scanner.Scan() {
        // do something with a line
		var inputLine string = scanner.Text()
		var dir string = string(inputLine[0])
		var amountString = string(inputLine[1:])
		var amount, err = strconv.Atoi(amountString)
		if err != nil {
			fmt.Printf("error")
		}
		for i:=0; i < amount; i++ {
			if dir == "L" {
				counter--;
			}
			if dir == "R" {
				counter++;
			}
			if counter > 99 {
				counter = 0
			}
			if counter < 0 {
				counter = 99
			}
			if counter == 0 {
				pass2++
			}
		}
		if counter == 0 {
				pass++
		}
    	//fmt.Printf("%d \n", counter)

    }
    fmt.Printf("%d \n", pass)
    fmt.Printf("%d \n", pass2)

    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }

}