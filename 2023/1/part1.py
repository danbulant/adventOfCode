import re
numbers = [(int(re.match(r".*?(\d)", x)[1]) * 10 + int(re.match(r"(?:.*)(\d)", x)[1])) for x in open("input3", "r") if re.match(r".*?\d+", x) is not None]
print(sum(numbers))
print(numbers)