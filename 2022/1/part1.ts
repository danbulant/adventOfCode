
const text = await Deno.readTextFile("./input");
const lines = text.split("\n");

let acc = 0;
let max = 0;
for(const line of lines) {
    if(line === "") {
        if(acc > max) {
            max = acc;
        }
        console.log(acc);
        acc = 0;
        continue;
    }
    acc += parseInt(line);
    if(isNaN(acc)) {
        console.log("'" + line + "'");
        break;
    }
}

console.log();
console.log("max", max);