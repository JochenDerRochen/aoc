package main

import( 
	"fmt"
	"log"
    "os"
	"bufio"
	"strings"
	"strconv"
	"slices"
)

func main() {

    f, err := os.Open("input.txt")
    if err != nil {
        log.Fatal(err)
    }
    defer f.Close()
	var solution = 0
	var solution2 = 0
    scanner := bufio.NewScanner(f)
	var fullMatrix [][]string
	var fullMatrix2 [][]string
    for scanner.Scan() {
		var inputLine string = scanner.Text()
		var toDelete []int
		var toHold []string
		var lines = strings.Split(inputLine, " ")
		var lines2 = strings.Split(inputLine, "")
		fullMatrix2 = append(fullMatrix2, lines2)
		for i:= 0; i < len(lines); i++ {
			lines[i] = strings.ReplaceAll(lines[i], " ", "")
			if lines[i] == "" {
				toDelete = append(toDelete, i)
			}
		}
		for i:=0; i < len(lines); i++ {
			if !slices.Contains(toDelete, i) {
				toHold = append(toHold, lines[i])
			}
		}
		fullMatrix = append(fullMatrix, toHold)
	}



	for i:= 0; i < len(fullMatrix[0]); i++ {
		var operation = fullMatrix[len(fullMatrix)-1][i]
		var overall = 0
		if operation == "*" {
			overall = 1
		}
		for j:= 0;  j< len(fullMatrix)-1; j++ {
			var val, _ = strconv.Atoi(fullMatrix[j][i])
			if operation == "+" {
				overall += val
			} else {
				overall *= val
			}
		}
		solution += overall
	}
	var strings []string
	var numString = ""
	var overall = 0
	for i:= len(fullMatrix2[0])-1; i >= 0; i-- {
		if numString != "" {
			strings = append(strings, numString)
		}
		numString = ""
		for j:= 0; j < len(fullMatrix2); j++ {
			if fullMatrix2[j][i] != " " && fullMatrix2[j][i] != "+" &&  fullMatrix2[j][i] != "*" {
				numString += string(fullMatrix2[j][i])
			} else {
				if fullMatrix2[j][i] == "+" {
					overall = 0
					strings = append(strings, numString)
					for m:= 0; m < len(strings); m++ {
						var val,_ = strconv.Atoi(strings[m])
						overall += val
					}
					solution2 +=overall
					numString = ""
					fmt.Printf("%v \n", strings)
					strings = []string{}
				} 
				if fullMatrix2[j][i] == "*" {
					overall = 1
					strings = append(strings, numString)
					numString = ""
					for m:= 0; m < len(strings); m++ {
						var val,_ = strconv.Atoi(strings[m])
						overall *= val
					}
					solution2 += overall
					fmt.Printf("%v \n", strings)
					strings = []string{}
				}
			}
		}
	}
	fmt.Printf("Part1: %v \n", solution)
	fmt.Printf("Part2: %v \n", solution2)
}

func getBiggestLength(arr []string) int {
	var biggest = 0
	for _, val := range arr {
		if len(val) > biggest {
			biggest = len(val)
		}
	}
	return biggest
}