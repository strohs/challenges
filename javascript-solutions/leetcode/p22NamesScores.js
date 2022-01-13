/**
 * # Problem 22 - Names Scores
 * Using `names.txt`, a 46K text file containing over five-thousand first names, begin by
 * sorting it into alphabetical order. Then working out the alphabetical value for each name,
 * multiply this value by its alphabetical position in the list to obtain a name score.
 * For example, when the list is sorted into alphabetical order, COLIN, which is worth:
 * 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score
 * of 938 × 53 = 49714.
 * What is the total of all the name scores in the file?
 */
const fs = require('fs');

const filePath = "../euler-java/src/p022_names.txt";

function parseFile() {
    const line = fs.readFileSync(filePath, 'UTF-8');
    return line.split(",")
        .map((name, idx) => name.replace(/"/gi, ''));
}

function nameScore(name) {
    return Array.from(name)
        .map(ch => ch.charCodeAt(0) - 65 + 1)
        .reduce((acc, cc) => acc + cc, 0)
}

let names = parseFile();
names = names.sort();
let totalScore = names
    .map((name, idx) => nameScore(name) * (idx + 1))
    .reduce((acc, score) => acc + score);

// 871198282
console.log("total name score =", totalScore);