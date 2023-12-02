import math
limits = {
    "red": 12,
    "green": 13,
    "blue": 14
}

print(sum([
    math.prod([
        max([type[0] for round in game["rounds"] for type in round if type[1] == key]) for key in limits.keys()
    ])
    for game in map(lambda game: {
        "id": int(game.split(" ")[1][0:-1]),
        "rounds": [[(int(type.strip().split(" ")[0]), type.strip().split(" ")[1]) for type in round.split(", ")] for round in game.split(":")[1].split(";")]
    }, open("./input2"))
]))