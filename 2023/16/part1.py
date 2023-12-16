
map = []

for line in open('input1'):
    map.append([x for x in line.strip()])

energised = [[False for _ in x] for x in map]

def print_energised():
    for line in energised:
        print(''.join(['#' if x else '.' for x in line]))

# List of tuples of (x, y) coordinates and vec direction vectors
beams = []

VEC_RIGHT = (0, 1)
VEC_LEFT = (0, -1)
VEC_UP = (-1, 0)
VEC_DOWN = (1, 0)

beams.append(((0, -1), VEC_RIGHT))

mirrorMap = {
    ('/', VEC_RIGHT): VEC_UP,
    ('/', VEC_LEFT): VEC_DOWN,
    ('/', VEC_UP): VEC_RIGHT,
    ('/', VEC_DOWN): VEC_LEFT,
    ('\\', VEC_RIGHT): VEC_DOWN,
    ('\\', VEC_LEFT): VEC_UP,
    ('\\', VEC_UP): VEC_LEFT,
    ('\\', VEC_DOWN): VEC_RIGHT
}

def add(pos, vec):
    return (pos[0] + vec[0], pos[1] + vec[1])

def append_if_not_in(list, item):
    if item not in list:
        list.append(item)

offset = 0
while len(beams) > offset:
    beam = beams[offset]
    offset += 1
    pos, vec = beam
    x, y = beam[0]
    dx, dy = beam[1]
    # print("Beam at (%d, %d) with vector (%d, %d)" % (x, y, dx, dy))

    if x + dx < 0 or x + dx >= len(map[0]) or y + dy < 0 or y + dy >= len(map):
        continue

    new_pos = (x + dx, y + dy)
    point = map[new_pos[0]][new_pos[1]]
    # print('point: ' + point)
    energised[new_pos[0]][new_pos[1]] = True

    if point == '.':
        append_if_not_in(beams, (new_pos, vec))
    if point == '/' or point == '\\':
        new_vec = mirrorMap[(point, vec)];
        append_if_not_in(beams, (new_pos, new_vec))
    if point == '-' or point == '|':
        if (point == '-' and (vec == VEC_RIGHT or vec == VEC_LEFT)) or (point == '|' and (vec == VEC_UP or vec == VEC_DOWN)):
            append_if_not_in(beams, (new_pos, vec))
        elif point == '-':
            append_if_not_in(beams, (new_pos, VEC_LEFT))
            append_if_not_in(beams, (new_pos, VEC_RIGHT))
        elif point == '|':
            append_if_not_in(beams, (new_pos, VEC_UP))
            append_if_not_in(beams, (new_pos, VEC_DOWN))

print_energised()
print(sum([sum([1 if x else 0 for x in y]) for y in energised]))