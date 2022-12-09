const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf8").trimEnd();
const data = input
  .split("\n")
  .map((line) => line.replace("\r", ""))
  .map((line) => line.split(""));

const startX = 1;
const maxX = data[0].length;
let startY = 1;
const maxY = data.length;
let sum = (data[0].length + data.length) * 2 - 4;

function checkRow(x, y) {
  const size = data[x][y];

  let left = false;
  let right = false;

  let stoppedLeft = false;
  let treesLeft = 0;
  let stoppedRight = false;
  let treesRight = 0;

  for (let i = y - 1; i >= 0; i--) {
    if (data[x][i] >= size) left = true;
    if (!stoppedLeft) {
      if (data[x][i] <= size) {
        treesLeft += 1;
        if (data[x][i] == size) {
          stoppedLeft = true;
        }
      } else if (i == 0) treesLeft += 1;
      else {
        stoppedLeft = true;
        treesLeft += 1;
      }
    }
  }
  for (let i = y + 1; i < maxX; i++) {
    if (data[x][i] >= size) right = true;
    if (!stoppedRight) {
      if (data[x][i] <= size) {
        treesRight += 1;
        if (data[x][i] == size) {
          stoppedRight = true;
        }
      } else if (i == maxX - 1) treesRight += 1;
      else {
        stoppedRight = true;
        treesRight += 1;
      }
    }
  }
  return [left && right, treesLeft, treesRight];
}

function checkCol(x, y) {
  const size = data[x][y];

  let top = false;
  let bottom = false;

  let stoppedUp = false;
  let treesUp = 0;
  let stoppedDown = false;
  let treesDown = 0;

  for (let i = x - 1; i >= 0; i--) {
    if (data[i][y] >= size) top = true;
    if (!stoppedUp) {
      if (data[i][y] <= size) {
        treesUp += 1;
        if (data[i][y] == size) {
          stoppedUp = true;
        }
      } else if (i == 0) treesUp += 1;
      else {
        stoppedUp = true;
        treesUp += 1;
      }
    }
  }
  for (let i = x + 1; i <= maxY - 1; i++) {
    if (data[i][y] >= size) bottom = true;
    if (!stoppedDown) {
      if (data[i][y] <= size) {
        treesDown += 1;
        if (data[i][y] == size) {
          stoppedDown = true;
        }
      } else if (i == maxY - 1) treesDown += 1;
      else {
        stoppedDown = true;
        treesDown += 1;
      }
    }
  }
  return [top && bottom, treesUp, treesDown];
}

for (let i = startX; i < maxX - 1; i++) {
  for (let j = startY; j < maxY - 1; j++) {
    if (!checkRow(i, j)[0] || !checkCol(i, j)[0]) sum += 1;
  }
}
console.log("Part one:", sum);

// --- PART TWO --- //

let bestView = 0;

for (let i = 0; i < data[0].length; i++) {
  for (let j = 0; j < data.length; j++) {
    const [_, left, right] = checkRow(i, j);
    const [__, top, bottom] = checkCol(i, j);
    const trees = left * right * top * bottom;
    if (trees > bestView) bestView = trees;
  }
}

console.log("Part two:", bestView);
