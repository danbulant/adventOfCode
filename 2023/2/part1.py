limits = {
    "red": 12,
    "green": 13,
    "blue": 14
}

print(sum([
    game["id"] for game in map(lambda game: {
        "id": int(game.split(" ")[1][0:-1]),
        "rounds": [[(int(type.strip().split(" ")[0]), type.strip().split(" ")[1]) for type in round.split(", ")] for round in game.split(":")[1].split(";")]
    }, open("./input2"))
    if all(type[0] <= limits[type[1]] for round in game["rounds"] for type in round)
]))