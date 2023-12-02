
const input = Deno.readTextFileSync("./input");

const lines = input.split("\n").map(t => t.split("").map(t => parseInt(t)));


let highestCount = 0;

function countLength(row: number[], num: number) {
    let count = 0;
    for(let i = 0; i < row.length; i++) {
        count++;
        if(row[i] >= num) {
            break;
        }
    }
    return count;
}

for(let x = 0; x < lines.length; x++) {
    for(let y = 0; y < lines[x].length; y++) {
        const num = lines[x][y];

        // get all numbers in row;
        const rowBefore = lines[x].slice(0, y).reverse();
        const rowAfter = lines[x].slice(y + 1);
        const columnBefore = lines.map(t => t[y]).slice(0, x).reverse();
        const columnAfter = lines.map(t => t[y]).slice(x + 1);
        const isEdge = rowBefore.length == 0 || rowAfter.length == 0 || columnBefore.length == 0 || columnAfter.length == 0;
        if(isEdge) continue;
        const rowBeforeCount = countLength(rowBefore, num);
        const rowAfterCount = countLength(rowAfter, num);
        const columnBeforeCount = countLength(columnBefore, num);
        const columnAfterCount = countLength(columnAfter, num);
        const count = rowBeforeCount * rowAfterCount * columnBeforeCount * columnAfterCount;
        console.log(`[${x} ${y}]`, num.toString(), count, rowBeforeCount, rowAfterCount, columnBeforeCount, columnAfterCount);
        if(count > highestCount) {
            highestCount = count;
        }
    }
}

console.log(highestCount);