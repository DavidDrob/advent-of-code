const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf8").trimEnd();
const data = input.split("\n").map((l) => l.split(" "));

const rules = {
  X: {
    beats: "C",
    equivalent: "A",
    points: 1,
    pointsForResult: 0,
  },
  Y: {
    beats: "A",
    equivalent: "B",
    points: 2,
    pointsForResult: 3,
  },
  Z: {
    beats: "B",
    equivalent: "C",
    points: 3,
    pointsForResult: 6,
  },
};

let sum = 0;

data.forEach((round) => {
  let oponent = round[0];
  let me = round[1].replace("\r", "");

  if (rules[me].beats == oponent) sum += rules[me].points + 6;
  else if (oponent == rules[me].equivalent) sum += rules[me].points + 3;
  else sum += rules[me].points;
});

console.log("Part one: ", sum);

// --- PART TWO --- //

let sumTwo = 0;

data.forEach((round) => {
  let oponent = round[0];
  let me = round[1].replace("\r", "");
  let key;

  if (me == "X") {
    const eq = Object.keys(rules).find((key) => rules[key].equivalent === oponent);
    key = Object.keys(rules).find((key) => rules[key].equivalent === rules[eq].beats);
  } else if (me == "Y") {
    key = Object.keys(rules).find((key) => rules[key].equivalent === oponent);
  } else {
    key = Object.keys(rules).find((key) => rules[key].beats === oponent);
  }

  sumTwo += rules[me].pointsForResult + rules[key].points;
});

console.log("Part two: ", sumTwo);
