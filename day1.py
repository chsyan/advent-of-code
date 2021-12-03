with open("day1.txt","r") as f:
    lines = f.read().splitlines()

count = 0
for i in range(1, len(lines)):
    if int(lines[i]) > int(lines[i-1]):
        count+=1
print('Part 1: ', count)

count = 0
for i in range(len(lines)-3):
    if int(lines[i+3]) > int(lines[i]):
        count+=1
print('Part 2: ', count)
