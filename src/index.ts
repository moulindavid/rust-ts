import fs from "fs";

const randomList = [1, 2, 3].map( x => x + 1);

console.log(randomList);


fs.readFileSync("lines").
    toString().
    split("\n").
    forEach(line => console.log(line));
