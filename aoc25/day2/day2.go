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
	var counter = 0
	var counter2 = 0
    for scanner.Scan() {
		var inputLine string = scanner.Text()
		var parts = strings.Split(inputLine,",")
		for _, part := range parts {
			var ranges = strings.Split(part, "-")
			var leftBound, _ = strconv.Atoi(ranges[0])
			var rightBound, _ = strconv.Atoi(ranges[1])
			for i:= leftBound; i <= rightBound; i++ {
				var testValue = strconv.Itoa(i)
				var arr []string
				for j := 2; j <= len(testValue); j++ {
					arr = divideString(testValue, j)
					if(allSameStrings(arr)) {
						counter2 += i
						break
					}
				}
				if len(testValue) % 2 == 0{
					var leftPart = testValue[0:len(testValue)/2]
					var rightPart = testValue[len(testValue)/2:]
					if leftPart == rightPart {
						counter += i
					}
				}
				
			}
		}
	}
	fmt.Printf("Part 1: %v \n", counter)
	fmt.Printf("Part 2: %v \n", counter2)
}

func allSameStrings(a []string) bool {
    for _, v := range(a) {
        if v != a[0] {
            return false
        }
    }
    return true
}

func divideString(mystr string, size int) []string {
   var parts []string
   partSize := len(mystr) / size
   for i := 0; i < size; i++ {
      start := i * partSize
      end := start + partSize
      if i == size-1 {
         end = len(mystr)
      }
      parts = append(parts, mystr[start:end])
   }
   return parts
}