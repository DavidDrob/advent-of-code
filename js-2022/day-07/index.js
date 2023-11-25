const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf8").trimEnd();
const data = input.split("\n").map((line) => line.replace("\r", ""));
data.shift();

const numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
let files = {
  "/": 0,
};

var currentDirectory = "/";
let listIdxs = [];

for (const line in data) data[line] == "$ ls" ? listIdxs.push(parseInt(line)) : null;
// listIdxs[4] = 24;
listIdxs[4] = 22;

for (let j = 0; j < listIdxs.length; j++) {
  for (let i = listIdxs[j] + 1; i < listIdxs[j + 1]; i++) {
    if (numbers.includes(data[i].charAt(0))) {
      if (files[currentDirectory] == undefined) files[currentDirectory] = 0;
      files[currentDirectory] += parseInt(data[i].split(" ")[0]);

      let splitDir = currentDirectory.split("/").map((item) => item.replace("", "/"));

      let amount = 1;
      for (let k = splitDir.length; k > 1; k -= 2) {
        splitDir.splice(k - 2, amount);

        let joined = splitDir.join("");
        if (joined.substring(0, 2) == "//") joined = joined.substring(1);
        amount += 1;

        if (files[joined] == undefined) files[joined] = 0;
        files[joined] += parseInt(data[i].split(" ")[0]);
      }
    } else if (data[i].substring(0, 4) == "$ cd") {
      const destination = data[i].split("cd ")[1];

      if (destination == "..") {
        const splitDir = currentDirectory.split("/");
        splitDir.pop();
        splitDir.pop();
        if (splitDir == "") currentDirectory = "/" + splitDir.join("/");
        else currentDirectory = splitDir.join("/");
      } else {
        currentDirectory += destination + "/";
      }
    }
  }
}

console.log(files);

const newArr = Object.values(files).filter((element) => 100000 >= element);
console.log(
  "Part one:",
  newArr.reduce((a, b) => a + b, 0)
);
