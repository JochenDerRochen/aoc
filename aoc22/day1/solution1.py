with open('input') as file:
    lines = file.readlines()
array = []
sum = 0
for line in lines:
    if(line != "\n"):
        castable = line.replace("\n", "")
        sum += int(line)
    if(line == "\n"):
        array.append(sum)
        sum = 0
#solution for first star
print(max(array))

#solution for second star
array.sort(reverse=True)
totalThree = array[0] + array[1] + array[2]
print(totalThree)