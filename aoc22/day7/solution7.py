from anytree import Node, RenderTree

class FileNode(Node):
    size = 0

with open('input') as file:
    lines = file.readlines()

currentNode = None
command = False
for line in lines:
    line = line.replace("\n", "")
    if line[0] == "$":
        if line[2] == "c":
            #go into dir
            if line.find("..") == -1:
                dirName = line[5:]
                if currentNode != None:
                    parentNode = currentNode
                    currentNode = FileNode(dirName, parent=parentNode)
                    print(currentNode.size)
                else:
                    currentNode = FileNode(dirName)
            #go out of dir
            else:
                currentNode = currentNode.parent
        elif line[2] == "l":
            continue
    else:
        #we know it is a dir
        if line[0] == "d":
            continue
        elif line[0] != "d":
            sizeIndex = line.find(" ")
            size = int(line[:sizeIndex])
            name = line[sizeIndex+1:]
            file = FileNode(name, parent=currentNode)
            file.size = size

while currentNode.parent is not None:
    currentNode = currentNode.parent

for pre, fill, node in RenderTree(currentNode):
    print("%s%s" % (pre, node.name))

for i in [x.split()for x in open("input")]:
    if i[0]=="dir"or i[1]=="ls":pass
    elif i[0]!='$':d[-1]+=int(i[0])
    elif i[2]=='..':
        s+=[d.pop()]
        d[-1]+=s[-1]
    elif i[2]=='/':s,d=[],[0]
    elif i[0]=="$"and i[1]=="cd":d.append(0)
print(sum(i for i in s+d[-1:]if i<=100000))
print(min(i for i in s+d[-1:]if i>(sum(d)-40000000)))   