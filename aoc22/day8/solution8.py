treeGrid = []
with open('input') as file:
    lines = file.readlines()

for line in lines:
    line = line.replace("\n", "")
    array = list(line)
    array = list(map(int, array))
    treeGrid.append(array)

 
counter = 0
maxScenic = 0
for i in range(len(treeGrid)):
    for j in range(len(treeGrid[0])):
        #edges
        if i == 0 or j == 0 or i == len(treeGrid)-1 or j == len(treeGrid[0])-1:
            counter += 1
        else:
            tree = treeGrid[i][j]
            rightSide = treeGrid[i][j+1:]
            leftSide = treeGrid[i][:j]
            bottomSide = [column[j] for column in treeGrid][i+1:]
            topSide = [column[j] for column in treeGrid][:i]
            leftSide.reverse()
            topSide.reverse()
            rCounter = 0
            lCounter = 0
            tCounter = 0
            bCounter = 0
            for r in rightSide:
                if r < tree:
                    rCounter += 1
                else:
                    rCounter += 1
                    break
            for l in leftSide:
                if l < tree:
                    lCounter += 1
                else:
                    lCounter += 1
                    break
            for t in topSide:
                if t < tree:
                    tCounter += 1
                else:
                    tCounter += 1
                    break
            for b in bottomSide:
                if b < tree:
                    bCounter += 1
                else:
                    bCounter += 1
                    break
            #print("for i, j", i, j, treeGrid[i][j], rCounter, lCounter, tCounter, bCounter)
            scenicForTree = rCounter * lCounter * tCounter * bCounter
            #print(scenicForTree)
            if scenicForTree > maxScenic:
                maxScenic = scenicForTree
            if tree > max(rightSide) or tree > max(leftSide) or tree > max(topSide) or tree > max(bottomSide):
                counter += 1

print(counter)
print(maxScenic)