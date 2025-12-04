package main

import( 
	"fmt"
	"log"
    "os"
	"bufio"
	"strconv"
)

func main() {

    f, err := os.Open("input.txt")
    if err != nil {
        log.Fatal(err)
    }
    defer f.Close()
    scanner := bufio.NewScanner(f)
	var overallJoltage = 0
	var overallJoltage2 = 0
    for scanner.Scan() {
		var highestJoltage = 0
		var currentJoltage = ""
		var currentIndex = 0
		var inputLine string = scanner.Text()
		for i := 0; i < len(inputLine); i++ {
			var val = inputLine[i]
			for j := i; j < len(inputLine); j++ {
				if j != i {
					var val1 = inputLine[j]
					var joltage = string(val) + string(val1)
					var joltageInt, _ = strconv.Atoi(joltage)
					if joltageInt > highestJoltage {
						highestJoltage = joltageInt
					}
				}
			}
		}
		for len(currentJoltage) < 12 {
			var bestIndex = currentIndex
			var bestJoltage = 0
			for len(inputLine)-currentIndex >= 12-len(currentJoltage) {
				var currentVal, _ = strconv.Atoi(string(inputLine[currentIndex]))
				if currentVal > bestJoltage {
					bestJoltage = currentVal
					bestIndex = currentIndex
				}
				currentIndex++
			}
			currentJoltage = currentJoltage + string(inputLine[bestIndex])
			currentIndex = bestIndex+1
		}
		overallJoltage += highestJoltage
		var currJoltageI, _ = strconv.Atoi(currentJoltage) 
		overallJoltage2 += currJoltageI
	}
	fmt.Printf("Part1: %v \n", overallJoltage)
	fmt.Printf("Part2: %v \n", overallJoltage2)
}
