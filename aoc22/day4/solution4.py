with open('input') as file:
    lines = file.readlines()
counter = 0
counter2 = 0
for line in lines:
    minus = line.find("-")
    komma = line.find(",")
    first1 = int(line[0:minus])
    second1 = int(line[minus+1:komma])
    secondLine = line[komma+1: len(line)]
    minus = secondLine.find("-")
    first2 = int(secondLine[0:minus])
    second2 = int(secondLine[minus+1:len(secondLine)])
    array1 = set(range(first1, second1+1))
    array2 = set(range(first2, second2+1))
    if(array1.issubset(array2) or array2.issubset(array1)):
        counter += 1
    if(bool(array1 & array2)):
        counter2 += 1
print(counter)
print(counter2)