
const text = await Deno.readTextFile("./input");
const lines = text.split("\n");

let values: number[] = [];
let acc = 0;
for(const line of lines) {
    if(line === "") {
        values.push(acc);
        acc = 0;
        continue;
    }
    acc += parseInt(line);
    if(isNaN(acc)) {
        console.log("'" + line + "'");
        break;
    }
}

values.sort((a, b) => b - a);
console.log(values.slice(0, 3));
console.log(values.slice(0, 3).reduce((a,b) => a+b,0));