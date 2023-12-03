import re
import math

symbols = []
partnumbers = []

lineNumber = 0
for line in open("./input2"):
    lineNumber += 1
    partnumbers += [(lineNumber, m.group(), m.start(), m.end()) for m in re.finditer(r'\d+', line)]
    symbols += [(lineNumber, m.group(), m.start(), m.end()) for m in re.finditer(r'[^\d.\n]', line)]

print(sum(
    [
        math.prod([int(number[1]) for number in foundNumbers])
        for symbol, foundNumbers in [
            (symbol, [partnumber for partnumber in partnumbers if abs(symbol[0] - partnumber[0]) <= 1 and partnumber[2] - 1 <= symbol[2] <= partnumber[3]])
            for symbol in symbols if symbol[1] == '*'
        ]
        if len(foundNumbers) == 2
    ]
))
