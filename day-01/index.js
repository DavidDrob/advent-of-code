const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf8").trimEnd();
const data = input.split("\n").map(Number);
data.push(0);

let elfs = [];
let currentElf = 0;

for (let i = 0; i < data.length; i++) {
  if (data[i] !== 0) currentElf += data[i];
  else {
    elfs.push(currentElf);
    currentElf = 0;
  }
}
elfs.sort((a, b) => b - a);
console.log("Part one:", elfs[0]);
console.log("Part two:", elfs[0] + elfs[1] + elfs[2]);
