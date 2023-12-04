import re
wordNumbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
parse = lambda x: int(x) if x.isdigit() else wordNumbers.index(x)
numbers = [
    (parse(re.match(r".*?(\d|one|two|three|four|five|six|seven|eight|nine)", x)[1]) * 10 + parse(re.match(r"(?:.*)(\d|one|two|three|four|five|six|seven|eight|nine)", x)[1]))
    for x in open("input2", "r")
    if re.match(r".*?(\d|one|two|three|four|five|six|seven|eight|nine)", x) is not None
]
print(sum(numbers))