package main

import( 
	"fmt"
	"log"
    "os"
	"bufio"
	"strings"
	"strconv"
)

func main() {

    f, err := os.Open("input.txt")
    if err != nil {
        log.Fatal(err)
    }
    defer f.Close()
    scanner := bufio.NewScanner(f)
	var fullMatrix [][]string
	var splitCounter = 0
	var fullMatrix2 [][]string
	var wayCounter = 0
    for scanner.Scan() {
		var inputLine string = scanner.Text()
		var lines = strings.Split(inputLine, "")
		fullMatrix = append(fullMatrix, lines)
		fullMatrix2 = append(fullMatrix2, lines)
	}
	/*for i:= 1; i < len(fullMatrix); i++ {
		for j:= 0; j < len(fullMatrix[0]); j++ {
			if fullMatrix[i-1][j] == "S" {
				fullMatrix[i][j] = "|"
			}
			if fullMatrix[i-1][j] == "|" && fullMatrix[i][j] != "^"{
				fullMatrix[i][j] = "|"
			}
			if fullMatrix[i-1][j] == "|" && fullMatrix[i][j] == "^"{
				fullMatrix[i][j-1] = "|"
				fullMatrix[i][j+1] = "|"
				splitCounter++
			}
		}
	}*/
	for i:= 1; i < len(fullMatrix2); i++ {
		for j:= 0; j < len(fullMatrix2[0]); j++ {
			if fullMatrix2[i-1][j] == "S" {
				fullMatrix2[i][j] = "1"
			}
			if (fullMatrix2[i-1][j] != "." && fullMatrix2[i-1][j] != "^" && fullMatrix2[i-1][j] != "S") && fullMatrix2[i][j] != "^"{
				var val, _ = strconv.Atoi(fullMatrix2[i][j])
				var valA, _ = strconv.Atoi(fullMatrix2[i-1][j])
				fullMatrix2[i][j] = strconv.Itoa(val + valA)
			}
			if fullMatrix2[i][j] == "^" {
				var val, _ = strconv.Atoi(fullMatrix2[i-1][j])
				var valL, _ = strconv.Atoi(fullMatrix2[i][j-1])
				var valR, _ = strconv.Atoi(fullMatrix2[i][j+1])
				fullMatrix2[i][j-1] = strconv.Itoa(val+valL)
				fullMatrix2[i][j+1] = strconv.Itoa(val+valR)
			}
		}
	}
	for i:= 0; i < len(fullMatrix2)-1; i++ {
		if fullMatrix2[len(fullMatrix2)-1][i] != "." {
			var val, _ =strconv.Atoi(fullMatrix2[len(fullMatrix2)-1][i])
			wayCounter += val
		}
	}
	fmt.Printf("Part1: %v \n", splitCounter)
	fmt.Printf("Part1: %v \n", wayCounter)

}