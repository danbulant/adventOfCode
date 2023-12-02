const input = await Deno.readTextFile('./input.txt');

const playerValues = {
    A: 1,
    B: 2,
    C: 3,
};
const userMap = {
    A: "R",
    B: "P",
    C: "S",

    X: "Lose",
    Y: "Draw",
    Z: "Win"
};

const resultValues = {
    X: 0,
    Y: 3,
    Z: 6
}

function mapFromValue(value: number) {
    return userMap[Object.entries(playerValues).find(([_, val]) => val === value)![0] as keyof typeof playerValues];
}
function mapResultFromValue(value: number) {
    return userMap[Object.entries(resultValues).find(([_, val]) => val === value)![0] as keyof typeof resultValues];
}

function log(...args: any[]) {
    console.log(args.join("\t"));
}

function logResult(pv: number, ov: number, rv: number, score: number) {
    log(mapResultFromValue(rv), pv + " " + mapFromValue(pv), ov + " " + mapFromValue(ov), rv, rv+pv, score);
}

log("state", "pv", "ov", "result", "score", "totalScore");

let score = 0;
for(const line of input.split("\n")) {
    const opponent = line[0] as (keyof typeof playerValues);
    const result = line[2] as (keyof typeof resultValues);
    const ov = playerValues[opponent];

    score += resultValues[result];

    // simple rock paper scissors
    if(result === "Y") {
        // need to draw
        score += playerValues[opponent];
        logResult(ov, ov, resultValues[result], score);
    } else if(result === "X") {
        // need to win
        if(ov === 1) {
            score += 3;
            logResult(ov, 3, resultValues[result], score);
        } else {
            score += ov - 1;
            logResult(ov, ov - 1, resultValues[result], score);
        }
    } else {
        // need to lose
        if(ov === 3) {
            score += 1;
            logResult(ov, 1, resultValues[result], score);
        } else {
            score += ov + 1;
            logResult(ov, ov + 1, resultValues[result], score);
        }
    }
}

console.log(score);