with open('input') as file:
    lines = file.readlines()

def checkForSameType(first, second):
    for i in first:
        if i in second:
            num = ord(i)
            if num < 97:
                return num - 38
            else:
                return num - 96

def checkForSameType2(first, second, third):
    for i in first:
        if i in second and i in third:
            print(first, second, third)
            num = ord(i)
            if num < 97:
                return num - 38
            else:
                return num - 96

sum = 0
for line in lines:
    length = len(line) - 1
    half = int(length / 2)
    first = line[0:half]
    second = line[half:length]
    sum += checkForSameType(first, second)

threeLines = []
sum2 = 0
for line in lines:
    if len(threeLines) < 3:
        threeLines.append(line)
    else:
        sum2 += checkForSameType2(threeLines[0], threeLines[1], threeLines[2])
        threeLines = []
        threeLines.append(line)

sum2 += checkForSameType2(lines[-1], lines[-2], lines[-3])
print(sum)
print(sum2)