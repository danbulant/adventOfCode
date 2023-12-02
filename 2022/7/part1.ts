
const input = Deno.readTextFileSync("./input");

class File {
    constructor(public name: string, public size: number) {}
}

class Directory {
    constructor(public name: string, public parent: Directory | null, public files: File[], public folders: Directory[]) {}

    getSize(): number {
        return this.files.reduce((acc, file) => acc + file.size, 0) + this.folders.reduce((acc, folder) => acc + folder.getSize(), 0);
    }

    print(level = 0) {
        console.log(" ".repeat(level * 2) + this.name + " (" + this.getSize() + ")");
        for(const file of this.files) {
            console.log(" ".repeat((level + 1) * 2) + file.name + " (" + file.size + ")");
        }
        for(const folder of this.folders) {
            folder.print(level + 1);
        }
    }
}

let root: Directory = new Directory("", null, [], []);
let current = root;

for(const line of input.split("\n")) {
    if(line[0] === "$") {
        if(line[2] === "c") {
            let name = line.slice(5);
            if(name === "/") current = root;
            else if(name === "..") current = current.parent!;
            else {
                let folder = current.folders.find(folder => folder.name === name);
                if(!folder) {
                    folder = new Directory(name, current, [], []);
                    current.folders.push(folder);
                }
                current = folder;
            }
        }
    } else {
        let [size, name] = line.split(" ");
        if(size === "dir") {
            const folder = new Directory(name, current, [], []);
            current.folders.push(folder);
        } else {
            current.files.push(new File(name, parseInt(size)));
        }
    }
}

root.print();

var acc = 0;

function traverse(directory: Directory) {
    let size = directory.getSize();
    if(size < 100000) {
        acc += size;
    }
    for(const folder of directory.folders) {
        traverse(folder);
    }
}

traverse(root);