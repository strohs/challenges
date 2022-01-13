/// # Problem 13 - Roman Numerals to Integer
/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M
/// For example, two is written as II in Roman numeral, just two one's added together. Twelve is
/// written as, XII, which is simply X + II. The number twenty seven is written as XXVII,
/// which is XX + V + II.
//
// Roman numerals are usually written largest to smallest from left to right. However, the numeral
// for four is not IIII. Instead, the number four is written as IV. Because the one is before the
// five we subtract it making four. The same principle applies to the number nine, which is written
// as IX. There are six instances where subtraction is used:
//
// * I can be placed before V (5) and X (10) to make 4 and 9.
// * X can be placed before L (50) and C (100) to make 40 and 90.
// * C can be placed before D (500) and M (1000) to make 400 and 900.
//
// Given a Roman Numeral, convert it to an Integer. Input is guaranteed to be within the
// range from 1 to 3999


// build a Map for roman numerals to integer
const mapper = new Map();
mapper.set("I", 1);
mapper.set("V", 5);
mapper.set("X", 10);
mapper.set("L", 50);
mapper.set("C", 100);
mapper.set("D", 500);
mapper.set("M", 1000);

// converts a string of roman numeral characters to an integer.
function romanToInt(romanStr) {
    return romanStr.split("").reduce((acc, c, idx, chars) => {
        const preChar = mapper.get(chars[idx - 1]);
        const curChar = mapper.get(c);

        if (preChar === undefined) {
            acc += curChar;
        } else if (preChar < curChar) {
            acc -= preChar;
            acc += curChar - preChar;
        } else {
            acc += curChar;
        }
        return acc;
    }, 0);
}

const res = romanToInt("MCMIV");
console.log(res);