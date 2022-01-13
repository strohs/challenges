/// # 29. Divide Two Integers
///
/// Given two integers `dividend` and `divisor`, divide two integers without using multiplication,
/// division and mod operator.
/// Return the `quotient` after dividing `dividend` by `divisor`.
/// The integer division should truncate toward zero, which means losing its fractional part.
/// For example, `truncate(8.345) = 8` and `truncate(-2.7335) = -2`.
///
/// ## Note
/// - Both dividend and divisor will be 32-bit signed integers.
/// - The divisor will never be 0.
/// - Assume we are dealing with an environment which could only store integers within the 32-bit
///   signed integer range: `[−2^31,  2^31 − 1]`. For the purpose of this problem, assume that your
///   function returns `2^31 − 1` when the division result overflows.
///
/// ## Example 1
/// Input: dividend = 10, divisor = 3
/// Output: 3
/// Explanation: 10/3 = truncate(3.33333..) = 3.


var divide = function (dividend, divisor) {

  const MAX_POS_INT =  2147483647;
  const MIN_NEG_INT = -2147483648;

  let finalSign = (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < 0) ? -1 : 1;

  // Check for overflow
  if (divisor === 0 || (dividend === MIN_NEG_INT && divisor === -1)) {
    return MAX_POS_INT;
  }
  if (divisor === dividend) {
    return 1;
  }

  let quot = 0;
  let dend = Math.abs(dividend);
  let dsor = Math.abs(divisor);

  while (dend >= dsor) {
    let shift = 0;
    let shiftedDivisor = dsor;
    while (dend >= shiftedDivisor) {
      shift += 1;
      shiftedDivisor = dsor << shift;
      // check for any overflow caused by JS numerics
      if (shiftedDivisor < 0) {
        break;
      }
    }
    quot += (1 << (shift - 1));
    dend -= dsor << (shift - 1);
  }

  return finalSign === -1 ? -quot : quot;
};


// function divide(dividend, divisor) {
//   if (dividend === 0) return 0;
//
//   // compute the sign of the final quotient
//   let sign = -1;
//   if ((dividend > 0 && divisor > 0) || (dividend < 0 && divisor < 0)) {
//     sign = 1;
//   }
//
//   let dend = Math.abs(dividend);
//   let dsor = Math.abs(divisor);
//
//   let quotient = 0;
//   while (dend >= dsor) {
//     quotient++;
//     dend -= dsor;
//   }
//
//   return quotient * sign;
// }

// const assert = require('assert');
// assert(divide(10, 3) === 3);
//
// assert(divide(3, 10) === 0);
//
// assert(divide(-10, 3) === -3);
//
// assert(divide(-10, -3) === 3);
//
// assert(divide(0, 33) === 0);
//
// assert(divide(-2147483648, -1) === 2147483647);

console.log(divide(10, 3));
console.log(divide(0, 33));
console.log(divide(-10, -3));
console.log(divide(1, -1));
console.log(divide(-3, 2));
console.log(divide(-2147483648, -1)); // 2147483647
console.log(divide(-2147483648, 1)); // -2147483648
