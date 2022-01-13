# Problem 1979 - Greatest Common Denominator of an Array
# Given an integer array `nums`, return the greatest common divisor of the smallest
# number and largest number in nums.
#
# The greatest common divisor of two numbers is the largest positive integer that
# evenly divides both numbers.

def gcd(a: int, b: int) -> int:
    # if b > a swap the values so that a is always larger than b
    a, b = (b, a) if b > a else (a, b)

    if b == 0:
        return a
    elif a == 0:
        return b
    else:
        return gcd(b, a % b)


def find_gcd(ns: list[int]) -> int:
    ns.sort()
    smallest = ns[0]
    largest = ns[len(ns) - 1]
    return gcd(largest, smallest)


nums = [7, 5, 6, 8, 3, 21, 48, 18]
res = find_gcd(nums)
print(res)
