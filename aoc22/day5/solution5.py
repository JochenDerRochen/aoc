with open('input') as file:
    lines = file.readlines()

arrays = [[], [], [], [], [], [], [], [], []]
arrays2 = [[], [], [], [], [], [], [], [], []]

def moveCargo(start, to, count):
    movingCargos = arrays[start][-count:]
    arrays[start] = arrays[start][:len(arrays[start]) - count]
    movingCargos.reverse()
    for cargo in movingCargos:
        arrays[to].append(cargo)

def moveCargoRight(start, to, count):
    movingCargos = arrays2[start][-count:]
    arrays2[start] = arrays2[start][:len(arrays2[start]) - count]
    for cargo in movingCargos:
        arrays2[to].append(cargo)

def buildStacks():
    for i in range(7, -1, -1):
        counter = 0
        for j in range(1, 36, 4):
            if lines[i][j] != " ":
                arrays[counter].append(lines[i][j])
                arrays2[counter].append(lines[i][j])
            counter += 1
buildStacks()

for i in range(10, len(lines)):
    line = lines[i]
    firstSpace = line.find(" ")
    if line[firstSpace+2] != " ":
        count = int(line[firstSpace+1: firstSpace+3])
    else:
        count = int(line[firstSpace+1])
    start = int(line[line.find("m ")+2])-1
    end = int(line[line.find("o ")+2])-1
    moveCargo(start, end, count)
    moveCargoRight(start, end, count)

solution1 = ""
solution2 =""
for i in arrays:
    solution1 += i[-1]
for i in arrays2:
    solution2 += i[-1]
print(solution1)
print(solution2)