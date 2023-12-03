import re
import math

symbols = []
partnumbers = []

lineNumber = 0
for line in open("./input2"):
    lineNumber += 1
    # Find all numbers and their positions
    foundNumbers = [(m.group(), m.start(), m.end()) for m in re.finditer(r'\d+', line)]
    partnumbers += [(lineNumber, number[0], number[1], number[2]) for number in foundNumbers]
    # Find all symbols and their positions
    foundSymbols = [(m.group(), m.start(), m.end()) for m in re.finditer(r'[^\d.\n]', line)]
    symbols += [(lineNumber, symbol[0], symbol[1], symbol[2]) for symbol in foundSymbols]

print(symbols)
foundGears = []

for symbol in symbols:
    if symbol[1] != '*': continue
    foundNumbers = [partnumber for partnumber in partnumbers if abs(symbol[0] - partnumber[0]) <= 1 and partnumber[2] - 1 <= symbol[2] <= partnumber[3]]
    if len(foundNumbers) != 2: continue
    gearRation = math.prod([int(number[1]) for number in foundNumbers])
    foundGears.append(gearRation)

print(sum(foundGears))