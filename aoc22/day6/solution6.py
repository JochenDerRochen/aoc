with open('input') as file:
    lines = file.readlines()

charCounter = 0
charStack = []
fourIsOpen = True
for c in lines[0]:
    charCounter += 1
    if c not in charStack:
        charStack.append(c)
    elif c in charStack:
        index = charStack.index(c)
        charStack = charStack[index+1:]
        charStack.append(c)
    if len(charStack) == 4 and fourIsOpen:
        print(charCounter)
        fourIsOpen = False
    if len(charStack) == 14:
        print(charCounter)
        break
