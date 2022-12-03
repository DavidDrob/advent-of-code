const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf8").trimEnd();
const data = input.split("\n").map((line) => line.replace("\r", ""));

function getPoints(str) {
  if (str == str.toLowerCase()) return str.charCodeAt() - 96;
  else return str.charCodeAt() - 38;
}

let sum = 0;

data.forEach((bag) => {
  const compOne = bag.substring(0, bag.length / 2).split("", bag.length / 2);
  const compTwo = bag.substring(bag.length / 2, bag.length).split("", bag.length / 2);

  for (let i = 0; i < compOne.length; i++) {
    const item = compOne[i];
    if (compTwo.includes(item)) {
      sum += getPoints(item);
      break;
    }
  }
});

console.log("Part one: ", sum);

// --- PART TWO --- //

sum = 0;

for (let i = 0; i < data.length; i += 3) {
  const bagOne = data[i].split("", data[i].length);
  const bagTwo = data[i + 1].split("", data[i + 1].length);
  const bagThree = data[i + 2].split("", data[i + 2].length);

  for (let j = 0; j < bagOne.length; j++) {
    const item = bagOne[j];
    if (bagTwo.includes(item) && bagThree.includes(item)) {
      sum += getPoints(item);
      break;
    }
  }
}

console.log("Part two: ", sum);
