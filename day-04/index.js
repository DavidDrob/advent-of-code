const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf8").trimEnd();
const data = input.split("\n").map((line) => line.replace("\r", "").split(","));

let sum = 0;
let sumTwo = 0;

for (let index = 0; index < data.length; index++) {
  const first = createRange(data[index][0].split("-")[0], data[index][0].split("-")[1]);
  const second = createRange(data[index][1].split("-")[0], data[index][1].split("-")[1]);
  let smaller;
  let larger;

  if (first.length - second.length >= 0) {
    smaller = second;
    larger = first;
  } else {
    smaller = first;
    larger = second;
  }

  for (let i = 0; i < smaller.length; i++) {
    if (!larger.includes(smaller[i])) break;
    else if (i >= smaller.length - 1) sum += 1;
  }

  // --- PART TWO --- //

  for (let i = 0; i < smaller.length; i++) {
    if (larger.includes(smaller[i])) {
      sumTwo += 1;
      break;
    }
  }
}

console.log("Part one:", sum);
console.log("Part two:", sumTwo);

function createRange(low, high) {
  let arr = [];
  for (let i = 0; i < high - low + 1; i++) {
    arr.push(high - i);
  }
  return arr;
}
