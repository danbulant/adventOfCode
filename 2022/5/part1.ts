
const file = Deno.readTextFileSync("./input");

const [arrangement, moves] = file.split("\n\n");

const list: string[][] = [];
function visualize() {
    console.log(
        list.map((t,i) => (i+1) + " " + t.slice().reverse().join(" ")).join("\n")
    )
}

for(const line of arrangement.split("\n")) {
    if(line[1] == "1") break;
    for(let i = 0; i < line.length; i+=4) {
        const letter = line[i + 1];
        if(letter == " ") continue;
        const num = i/4;
        if(!list[num]) list[num] = [];
        list[num].push(letter);
    }
}

visualize();

for(const move of moves.split("\n")) {
    if(!move.startsWith("move")) break;
    const [, num, , src, , dest ] = move.split(" ").map(t => parseInt(t));
    
    console.log();
    console.log(`Move ${num} from ${src} to ${dest}`);
    let moved = list[src - 1].splice(0, num).reverse();
    list[dest - 1].unshift(...moved);
    console.log(moved);
    visualize();
}

console.log(list)
visualize();