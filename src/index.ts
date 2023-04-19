import fs from "fs";

const randomList = [1, 2, 3].map( x => x + 1);

console.log(randomList);


fs.readFileSync("lines").
    toString().
    split("\n").
    forEach(line => console.log(line));


fs.readFileSync("lines_odd").
    toString().
    split("\n").
    filter((_, i) => i % 2 === 0).
    forEach(line => console.log(line));

fs.readFileSync("lines").
    toString().
    split("\n").
    filter((_, i) => i % 2 === 0).
    filter((_, i) => i >= 2 && i < 4).
    forEach(line => console.log(line));
