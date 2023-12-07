import functools
cards = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"]
cards.reverse()

hands = [(line.split(" ")[0], int(line.split(" ")[1])) for line in open("./input2")]

def type_value(hand):
    hand_count = {}
    for card in hand:
        if card[0] in hand_count:
            hand_count[card[0]] += 1
        else:
            hand_count[card[0]] = 1
    hand_max = max(hand_count.values())
    if hand_max >= 4: return hand_max + 2
    if hand_max == 3 and 2 in hand_count.values(): return 5
    if hand_max == 3: return 4
    if hand_max == 2 and list(hand_count.values()).count(2) == 2: return 3
    return hand_max


def compare(hand1_src, hand2_src):
    hand1 = hand1_src[0]
    hand2 = hand2_src[0]
    hand1_type = type_value(hand1)
    hand2_type = type_value(hand2)
    if hand1_type != hand2_type: return hand2_type - hand1_type
    # if there's a tie, check each card in sequence
    # print(hand1, hand2)
    for i in range(len(hand1)):
        if hand1[i][0] != hand2[i][0]:
            return cards.index(hand2[i][0]) - cards.index(hand1[i][0])

# sort the array using the compare function
hands.sort(key=functools.cmp_to_key(compare))
hands.reverse()

# print(hands)

valued_hands = [(hand[0], hand[1], hand[1] * (i + 1), type_value(hand[0])) for i, hand in enumerate(hands)]

print(valued_hands)
value = 0
for i in range(len(hands)):
    value += hands[i][1] * (i + 1)

print(value)