const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf8").trimEnd();
const data = input.split("");

latest = [];

for (let i = 0; i < data.length; i++) {
  const char = data[i];
  latest.push(char);
  if (latest.length > 3) {
    if (latest.length > 4) latest.shift();
    const local = new Set(latest);
    if (local.size === 4) {
      console.log("Part one:", i + 1);
      break;
    }
  }
}

// --- PART TWO --- //

for (let i = 0; i < data.length; i++) {
  const char = data[i];
  latest.push(char);
  if (latest.length > 13) {
    if (latest.length > 14) latest.shift();
    const local = new Set(latest);
    if (local.size === 14) {
      console.log("Part two:", i + 1);
      break;
    }
  }
}
