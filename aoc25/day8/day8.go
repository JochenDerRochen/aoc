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

type box struct {
	x int
	y int
	z int
	inCircut bool
}
func main() {

    f, err := os.Open("test.txt")
    if err != nil {
        log.Fatal(err)
    }
    defer f.Close()
	var boxes []box
	var circuts [][]box
    scanner := bufio.NewScanner(f)
    for scanner.Scan() {
		var inputLine string = scanner.Text()
		var coords = strings.Split(inputLine, ",")
		var x, _ = strconv.Atoi(coords[0]) 
		var y, _ = strconv.Atoi(coords[1]) 
		var z, _ = strconv.Atoi(coords[2]) 
		var newBox = box{x,y,z, false}
		boxes = append(boxes, newBox)
	}
	var counter = 0
	for counter < 10 {
		var closest = 1000000
		var closestJ = 0
		var closestI = 0
		for i:=0; i < len(boxes); i++ {
			for j:=0; j < len(boxes); j++ {
				if i != j && !boxes[i].inCircut{
					var dist = distance(boxes[i], boxes[j])
					if dist < closest {
						closest = dist
						closestJ = j
						closestI = i
					}
				}
			}
		}
		var added = false
		boxes[closestI].inCircut = false
		boxes[closestJ].inCircut = false
		for m:= 0; m < len(circuts); m++{
			var circut = circuts[m]
			if slices.Contains(circut, boxes[closestI]) {
				fmt.Printf("%v \n", circut)
				circut = append(circut, boxes[closestJ])
				boxes[closestJ].inCircut = true
				boxes[closestI].inCircut = true
				added = true
				break
			}
			if slices.Contains(circut, boxes[closestJ]) {
				circut = append(circut, boxes[closestI])
				boxes[closestI].inCircut = true
				boxes[closestJ].inCircut = true
				added = true
				break
			}
		}
		if !added { 
				var cir = []box{boxes[closestJ], boxes[closestI]}
				boxes[closestI].inCircut = true
				boxes[closestJ].inCircut = true
				circuts = append(circuts, cir)
		}
		counter++
	}
	fmt.Printf("%v \n", circuts)
}

func distance(box1 box, box2 box) int {
	var overall = 0
	overall += (box1.x - box2.x) * (box1.x - box2.x)
	overall += (box1.y - box2.y) * (box1.y - box2.y)
	overall += (box1.z - box2.z) * (box1.z - box2.z)
	return overall
}