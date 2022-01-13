/// # 28. Implement strStr()
/// Return the index of the first occurrence of `needle` in `haystack`, or -1 if `needle` is not
/// part of `haystack`
///
/// if `needle` is the empty string, return 0.
/// if `needle` is not in haystack, return -1.
///


/**
 * @param {string} haystack
 * @param {string} needle
 * @return {number}
 */
const strStr = function(haystack, needle) {
    if (needle.length === 0) return 0;

    let i = 0;
    while (i < haystack.length) {
        if (haystack.charAt(i) === needle.charAt(0) && allCharsMatch(haystack, i, needle)) {
            return i;
        }
        i++;
    }

    return -1;
};

/// returns true if all chars of `subStr` match the chars of `str`, starting at `fromIndex` of `str`, else false.
function allCharsMatch(str, fromIndex, subStr) {
    const sliceLength = str.length - fromIndex;
    if (subStr.length > sliceLength || sliceLength <= 0 ) return false

    let i = fromIndex;
    let j = 0;

    while (i < str.length && j < subStr.length) {

        if (str.charAt(i) !== subStr.charAt(j)) {
            return false;
        }
        i++;
        j++;
    }

    return true;
}

console.log( strStr("hello", "ll") );
console.log( strStr("hello", "llz") );
console.log( strStr("hello", "") );
console.log( strStr("hello", "hello") );


