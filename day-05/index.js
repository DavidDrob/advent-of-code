const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf8").trimEnd();
const data = input.split("\n").map((line) => line.replace("\r", "").split(","));

function getCrates() {
  let arr = [];
  for (let i = 0; i < data.length; i++) {
    if (data[i][0].split(" ").includes("1")) break;
    const cratesObject = data[i][0]
      .split(/\[(.*?)\]|(    )/)
      .filter((item) => item != "" && item != " " && item != undefined);
    arr.push(Object.keys(cratesObject).map((key) => [cratesObject[key]]));
  }
  return arr;
}

function getInstructions() {
  let arr = [];

  for (let i = data.length; i > 0; i--) {
    if (data[i] == "") break;
    data[i] != undefined ? arr.push(data[i]) : null;
  }
  return arr.reverse();
}

let crates = getCrates();
const instructions = getInstructions();

let formattedCrates = {};
crates.forEach((row) => {
  for (let i = 0; i < crates[0].length; i++) {
    if (formattedCrates[i] == undefined) formattedCrates[i] = [];
    if (row[i][0] != "    ") formattedCrates[i].push(row[i][0]);
  }
});

instructions.forEach((instruction) => {
  const amount = instruction[0].split(" ")[1];
  const from = instruction[0].split(" ")[3] - 1;
  const to = instruction[0].split(" ")[5] - 1;

  for (let i = 0; i < amount; i++) {
    const crate = formattedCrates[from][i];
    formattedCrates[to] = [crate.toString()].concat(formattedCrates[to]);
  }
  formattedCrates[from].splice(0, amount);
});
let result = "";

for (let i = 0; i < Object.keys(formattedCrates).length; i++) {
  formattedCrates[i][0] ? (result += formattedCrates[i][0]) : null;
}
console.log("Part one:", result);

// --- Part Two --- //

formattedCrates = {};
crates.forEach((row) => {
  for (let i = 0; i < crates[0].length; i++) {
    if (formattedCrates[i] == undefined) formattedCrates[i] = [];
    if (row[i][0] != "    ") formattedCrates[i].push(row[i][0]);
  }
});

let resultTwo = "";

instructions.forEach((instruction) => {
  const amount = instruction[0].split(" ")[1];
  const from = instruction[0].split(" ")[3] - 1;
  const to = instruction[0].split(" ")[5] - 1;

  let currentCrates = [];

  for (let i = 0; i < amount; i++) {
    const crate = formattedCrates[from][i];
    currentCrates.push(crate);
  }
  currentCrates.reverse();
  formattedCrates[to] = currentCrates.reverse().concat(formattedCrates[to]);
  formattedCrates[from].splice(0, amount);
});

for (let i = 0; i < Object.keys(formattedCrates).length; i++) {
  formattedCrates[i][0] ? (resultTwo += formattedCrates[i][0]) : null;
}

console.log("Part two:", resultTwo);

// --- BAD SOLUTION --- //

// formattedCrates.forEach((stack) => {
//   console.log(stack[0]);
// });
// let idx = 0;

// instructions.forEach((element) => {
//   const amount = element[0].split(" ")[1];
//   const from = element[0].split(" ")[3];
//   const to = element[0].split(" ")[5];
//   if (idx < 3) {
//     if (amount >= crates.length) crates = [[["    "], ["    "], ["    "]]].concat(crates);
//     for (let i = 0; i < amount; i++) {
//       // check if line exists -> replace

//       // console.log(from);
//       //   console.log(crates[i]);
//       //   console.log(crates[i][to - 1]);
//       if (crates[i][from - 1][0] == "    ") {
//         for (let j = i; j < amount.length; j++) {
//           console.log(crates[i]);
//           console.log(crates[i][to - 1]);
//           crates[j][to - 1][0] = crates[i + 1][from - 1][0];
//           crates[j + 1][from - 1][0] = "    ";
//         }
//       } else {
//         crates[i][to - 1][0] = crates[i][from - 1][0];
//         crates[i][from - 1][0] = "    ";
//       }
//       // else add new line on top
//     }
//     console.log(crates);
//     console.log("--");
//     idx += 1;
//   }

//   //   console.log(d);

//   //   crates[amount - 1][to - 1][0] = crates[amount - 1][from - 1][0];
//   //   crates[amount - 1][from - 1][0] = "    ";

//   //   console.log(crates);
// });

// for (const j in crates) {
//   for (const k in crates[j]) {
//     console.log(crates[j][k]);
//   }
// }
