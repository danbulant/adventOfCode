const input = await Deno.readTextFile('./input.txt');

const values = {
    A: 1,
    B: 2,
    C: 3,
    X: 1,
    Y: 2,
    Z: 3
};
const userMap = {
    A: "R",
    B: "P",
    C: "S",
    X: "R",
    Y: "P",
    Z: "S"
};

let score = 0;
for(const line of input.split("\n")) {
    const opponent = line[0] as (keyof typeof values);
    const player = line[2] as (keyof typeof values);
    const ov = values[opponent];
    const pv = values[player];

    score += values[player];

    // simple rock paper scissors
    if(ov === pv) {
        score += 3;
    } else if(pv - 1 === ov) {
        score += 6;
    } else if(pv + 2 === ov) {
        score += 6;
    }
}

console.log(score);