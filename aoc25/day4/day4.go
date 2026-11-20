
package main

import( 
	"fmt"
	"log"
    "os"
	"bufio"
	//"strconv"
)

func main() {

    f, err := os.Open("input.txt")
    if err != nil {
        log.Fatal(err)
    }
    defer f.Close()
    scanner := bufio.NewScanner(f)
	var a []string
	var height = 0
	var width = 0
	var determineWidth = true
	var solution = 0
	var solution2 = 0
	var removedOne = true
    for scanner.Scan() {
		var inputLine string = scanner.Text()
		for _, c := range inputLine {
			a = append(a, string(c))
			if determineWidth {
				width++
			}
		}
		determineWidth = false
		height++
	}
	for i := 0; i < height; i++ {
		for j:= 0; j < width; j++ {
			var counter = 0
			if a[i*height + j] == "@" {
				for m := -1; m < 2 ; m++ {
					for n := -1; n < 2; n++ {
						if !(n == 0 && m == 0) {
							var newI = i+m
							var newJ = j+n
							if newI >= 0 && newI < height && newJ >= 0 && newJ < width {
								if a[(i+m)*height + (j+n)] == "@" {
									counter++
								}
							}
						}
					}
				}
			if counter < 4 {
				solution++
			}
			}
		}
	}
	for removedOne {
		removedOne = false
		for i := 0; i < height; i++ {
			for j:= 0; j < width; j++ {
				var counter = 0
				if a[i*height + j] == "@" {
					for m := -1; m < 2 ; m++ {
						for n := -1; n < 2; n++ {
							if !(n == 0 && m == 0) {
								var newI = i+m
								var newJ = j+n
								if newI >= 0 && newI < height && newJ >= 0 && newJ < width {
									if a[(i+m)*height + (j+n)] == "@" {
										counter++
									}
								}
							}
						}
					}
				if counter < 4 {
					solution2++
					a[i*height+j] = "."
					removedOne = true
				}
				}
			}
		}
	}
	fmt.Printf("Part 1: %v \n", solution)
	fmt.Printf("Part 2: %v \n", solution2)
}