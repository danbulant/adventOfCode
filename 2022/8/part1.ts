
const input = Deno.readTextFileSync("./input");

const lines = input.split("\n").map(t => t.split("").map(t => parseInt(t)));

let hiddenCount = 0;
let visibleCount = 0;
let edge = 0;
let interior = 0;

for(let x = 0; x < lines.length; x++) {
    for(let y = 0; y < lines[x].length; y++) {
        const num = lines[x][y];

        // get all numbers in row;
        const rowBefore = lines[x].slice(0, y);
        const rowAfter = lines[x].slice(y + 1);
        const columnBefore = lines.map(t => t[y]).slice(0, x);
        const columnAfter = lines.map(t => t[y]).slice(x + 1);
        const isEdge = rowBefore.length == 0 || rowAfter.length == 0 || columnBefore.length == 0 || columnAfter.length == 0

        if(isEdge || (rowBefore.every(t => t < num) || rowAfter.every(t => t < num) || columnAfter.every(t => t < num) || columnBefore.every(t => t < num))) {
            visibleCount++;
            if(isEdge) {
                edge++;
            } else {
                console.log(num, rowBefore, rowAfter, columnBefore, columnAfter);
                console.log("visible interior");
                interior++;
            }
        } else {
            console.log(num, rowBefore, rowAfter, columnBefore, columnAfter);
            hiddenCount++;
            console.log("hidden");
        }
    }
    console.log("ROW", x);
}

console.log({ visibleCount, hiddenCount, edge, interior });