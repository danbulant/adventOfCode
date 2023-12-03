import re

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

matchingNumbers = []

for partnumber in partnumbers:
    foundSymbols = [symbol for symbol in symbols if abs(symbol[0] - partnumber[0]) <= 1 and partnumber[2] - 1 <= symbol[2] <= partnumber[3]]
    if len(foundSymbols) > 0:
        print(partnumber[1])
        matchingNumbers.append(partnumber[1])

print(sum([int(number) for number in matchingNumbers]))