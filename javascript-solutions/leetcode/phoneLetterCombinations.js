/// # Letter Combinations of a phone number
/// Given a string containing digits from `2-9` inclusive, return all possible letter combinations
/// that the number could represent.
/// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that
/// 1 does not map to any letters.
/// ```
/// 2 -> a,b
/// 3 -> d,e,f
/// 4 -> g,h,i
/// 5 -> j,k,l
/// 6 -> m,n,o
/// 7 -> p,q,r,s
/// 8 -> t,u,v
/// 9 -> w,x,y,z
/// ```
/// ## Example
/// input: "23"
/// output: `["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]`
///
/// FYI permutation - you care about the order of the elements
///     n! / (n - k)!
/// combinations - you don't care about the order of the elements
///    combination formula, you have n objects and want to choose k    = n! / k!(n - k!)

// build a mapping from digits to their corresponding letters
const digitMap = {
    2: ["a", "b", "c"],
    3: ["d", "e", "f"],
    4: ["g", "h", "i"],
    5: ["j", "k", "l"],
    6: ["m", "n", "o"],
    7: ["p", "q", "r", "s"],
    8: ["t", "u", "v"],
    9: ["w", "x", "y", "z"],
}

// return a new array that is the cartesian product of the elements in collections s1 and s2
// s1 and s2 should be Iterables containing strings
function cartesianProduct(s1, s2) {
    const products = [];
    for (const l1 of s1) {
        for (const l2 of s2) {
            products.push(l1 + l2);
        }
    }
    return products;
}

var letterCombinations = function(digits) {
    if (!digits) return [];

    return digits.split("")
        .reduce((acc, currDigit) => {
            const letters = digitMap[currDigit];
            return cartesianProduct(acc, letters);
        }, [""]);
};


// iterate through each digit's and combine the currDigit's letters with the next digit's letters
const combinations = letterCombinations("23");
console.log(combinations);