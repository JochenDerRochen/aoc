from enum import Enum
class Choice(Enum):
    ROCK = 1
    PAPER = 2
    SCISSOR = 3

def winOrLose(elv, player):
    castPlayer = castPlayerChoice(player)
    castElv = castElvChoice(elv)
    if castElv == castPlayer:
        return 3
    elif (castElv == 1 and castPlayer == 3) or (castElv == 2 and castPlayer == 1) or (castElv == 3 and castPlayer == 2):
        return 0
    else:
        return 6

def castPlayerChoice(char):
    if char == "X":
        return Choice.ROCK.value
    elif char == "Y":
        return Choice.PAPER.value
    elif char == "Z":
        return Choice.SCISSOR.value

def castElvChoice(char):
    if char == "A":
        return Choice.ROCK.value
    elif char == "B":
        return Choice.PAPER.value
    elif char == "C":
        return Choice.SCISSOR.value

def winOrLoseSecond(elv, player):
    playerChoice = castPlayerChoice(player)
    elvChoice = castElvChoice(elv)
    sum = 0
    if playerChoice == 1:
        sum += 0
        if elvChoice == 1:
            sum += 3
        elif elvChoice == 2:
            sum += 1
        else:
            sum += 2
    elif playerChoice == 2:
        sum += 3
        sum += elvChoice
    elif playerChoice == 3:
        sum += 6
        if elvChoice == 1:
            sum += 2
        elif elvChoice == 2:
            sum += 3
        else:
            sum += 1
    return sum

with open('input') as file:
    lines = file.readlines()

sum = 0
stratSum = 0
for line in lines:
    elvChoice = line[0]
    ownChoice = line[2]
    if ownChoice == "X":
        sum += Choice.ROCK.value
    elif ownChoice == "Y":
        sum += Choice.PAPER.value
    elif ownChoice == "Z":
        sum += Choice.SCISSOR.value
    sum += winOrLose(elvChoice, ownChoice)
    stratSum += winOrLoseSecond(elvChoice, ownChoice)

print(sum)
print(stratSum)
