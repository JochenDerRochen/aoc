
package main

import( 
	"fmt"
	"log"
    "os"
	"bufio"
	"strings"
	"strconv"
	"slices"
	"sort"
)
type freshRange struct {
	start int
	end  int
	modified bool
}
func main() {

    f, err := os.Open("input.txt")
    if err != nil {
        log.Fatal(err)
    }
    defer f.Close()
	var counter = 0
	var counter2 = 0
    scanner := bufio.NewScanner(f)
	var allRanges []freshRange
    for scanner.Scan() {
		var inputLine string = scanner.Text()
		if strings.Contains(inputLine,"-") {
			var ranges = strings.Split(inputLine,"-")
			var start,_ = strconv.Atoi(ranges[0])
			var end,_ = strconv.Atoi(ranges[1])
			var oneRange = freshRange{start, end, false}
			allRanges = append(allRanges, oneRange)
		} else {
			if len(inputLine) >= 1 {
				var toTest,_ = strconv.Atoi(inputLine)
				var found = false
				for _, r := range allRanges {
					if !found && r.start <= toTest && r.end >= toTest{
						found = true
						counter++
						break
					}
				}
			}
		}
	}
	var toDelete []int
	var newRanges []freshRange
	for i:= 0; i < len(allRanges); i++ {
		for j:= 0; j < len(allRanges); j++ {
			if j != i && !slices.Contains(toDelete, i){
				if allRanges[j].start >= allRanges[i].start && allRanges[j].end <= allRanges[i].end {
					toDelete = append(toDelete, j)
				}
			}
		}
	}
	
	for i := 0; i < len(allRanges); i++ {
		if !slices.Contains(toDelete, i) {
			newRanges = append(newRanges, allRanges[i])
		}
	}
	sort.Slice(newRanges, func(i, j int) bool {
		return newRanges[i].start < newRanges[j].start
	})

	for i:= 0; i < len(newRanges); i++ {
		for j:= 0; j < len(newRanges); j++ {
			if j != i {
				if newRanges[j].start <= newRanges[i].start && newRanges[j].end >= newRanges[i].start && newRanges[j].end <= newRanges[j].end && (!newRanges[j].modified){
					/*if newRanges[j].start == newRanges[j].end {
						newRanges[i].end = newRanges[j].end
					}*/
					newRanges[i].start = newRanges[j].start
					newRanges[j].start = 0
					newRanges[j].end = 0
					newRanges[j].modified = true
				}
				if newRanges[j].end >= newRanges[i].end && newRanges[j].start >= newRanges[i].start && newRanges[j].start <= newRanges[i].end && (!newRanges[j].modified) {
					newRanges[i].end = newRanges[j].end
					newRanges[j].start = 0
					newRanges[j].end = 0
					newRanges[j].modified = true
				}
			}
		}
	}
	for i:= 0; i < len(newRanges); i++ {
		if !(newRanges[i].start == 0 && newRanges[i].end == 0) {
			counter2 += (newRanges[i].end+1) - newRanges[i].start
		}
	}
	fmt.Printf("%v \n", newRanges)
	fmt.Printf("Part1: %v \n", counter)
	fmt.Printf("Part1: %v \n", counter2)
}