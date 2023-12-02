
const input = Deno.readTextFileSync("input");

let sum = 0;
for(const line of input.split("\n")) {
    const [first1, first2, second1, second2] = line.split(/[^\d]/).map(t => parseInt(t));
    console.log(".".repeat(first1) + "X".repeat(first2 - first1 + 1) + ".".repeat(100-first2) + " " + first1 + "-" + first2);
    console.log(".".repeat(second1) + "X".repeat(second2 - second1 + 1) + ".".repeat(100-second2) + " " + second1 + "-" + second2);
    if(first1 >= second1 && first1 <= second2 || first2 >= second1 && first2 <= second2 || second1 >= first1 && second1 <= first2 || second2 >= first1 && second2 <= first2) {
        sum++;
        console.log(`${first1}-${first2},${second1}-${second2} yes`);
    } else {
        console.log(`${first1}-${first2},${second1}-${second2} no`);
    }
    console.log();
}

console.log(sum);