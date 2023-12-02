
const input = Deno.readTextFileSync("input");

const priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".split("");

let sum = 0;
for(const line of input.split("\n")) {
    const firstHalf = line.slice(0, line.length / 2);
    const secondHalf = line.slice(line.length / 2);
    console.log(firstHalf, secondHalf, firstHalf.length, secondHalf.length);

    const sharedLetter = firstHalf.split("").find((letter) => secondHalf.includes(letter));
    sum += priority.indexOf(sharedLetter!) + 1;
    console.log(sharedLetter, priority.indexOf(sharedLetter!) + 1);
}

console.log(sum)