
const input = Deno.readTextFileSync("input");

const priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".split("");

let sum = 0;
const lines = input.split("\n");
for(let i = 0; i < lines.length; i += 3) {
    const first = lines[i];
    const second = lines[i + 1];
    const third = lines[i + 2];

    const sharedLetter = first.split("").find((letter) => second.includes(letter) && third.includes(letter));
    sum += priority.indexOf(sharedLetter!) + 1;
}

console.log(sum)