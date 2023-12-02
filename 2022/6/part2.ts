
const input: string = Deno.readTextFileSync("input");

for(let i = 14; i < input.length; i++) {
    let last = input.slice(i - 14, i);
    if(last.split("").find((c, i, a) => a.indexOf(c) !== i)) continue;
    console.log(last, i);
    break;
}