const path = require("path");
const readline = require('readline');
const fs = require("fs");

/// in this challenge morse code dots and dashes will be 'smooshed' together. Write a function:
/// `snorse` that takes a string of letters as input, and outputs morse code dots and dashes
/// smooshed together
///
/// # Example
///
/// ```
/// smorse("sos") => "...---..."
/// smorse("daily") => "-...-...-..-.--"
/// smorse("programmer") => ".--..-.-----..-..-----..-."
/// smorse("bits") => "-.....-..."
/// smorse("three") => "-.....-..."
/// ```
///
/// An obvious problem with this system is that decoding is ambiguous. For instance,
/// both `bits` and `three` encode to the same string, so you can't tell which one you would
/// decode to without more information.
///
/// #  Bonus Challenges
/// 1. The sequence `-...-....-.--.` is the code for four different words
///   `(needing, nervate, niding, tiling)`. Find the only sequence that's the code
///   for 13 different words.

const wordFilePath = path.join(__dirname, "./enable1.txt");
const letterMap = buildLetterMap();

// returns a map from the letters a-z,  to morse code dots and dashes
function buildLetterMap() {
    // morse code array for letters a-z
    const LETTERS = "abcdefghijklmnopqrstuvwxyz".split("");
    const CODES = ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. --- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --..".split(" ");

    let map = new Map();
    for (let i = 0; i < LETTERS.length; i++) {
        map.set(LETTERS[i], CODES[i]);
    }
    return map;
}

// convert a string into a "smooshed" morse code string
function smorse(str) {
    return str.split("").map(ch => letterMap.get(ch)).join("");
}

function mapCodesToWords(filePath) {
    return new Promise((resolve, reject) => {
        const codeMap = new Map();
        const readInterface = readline.createInterface({
            input: fs.createReadStream(filePath),
            crlfDelay: Infinity
        });

        readInterface
            .on("line", function(line) {
                const code = smorse(line);
                if (codeMap.has(code)) {
                    codeMap.get(code).push(line);
                } else {
                    codeMap.set(code, [line]);
                }
            })
            .on("close", () => {
                resolve(codeMap);
            });
    });
}

mapCodesToWords(wordFilePath)
    .then(mapping => {
        // find the morse code that maps the same 13 words
        let res;
        for (let [k,v] of mapping.entries()) {
            if (v.length === 13) {
                res = [k, v];
                break;
            }
        }
        console.log(res);
    })
    .catch(err => console.error(err));