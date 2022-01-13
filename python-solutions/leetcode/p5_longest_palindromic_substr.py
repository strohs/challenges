# # Problem 5 - longest palindromic substring
# Given a string `s`, return the longest palindromic substring in `s`
#
# ## Example 1
#
# Input: s = "babad"
# Output: "bab"
# Note: "aba" is also a valid answer.
#


# returns True if s is a palindrome
def is_palindrome(s: str) -> bool:
    if len(s) <= 1:
        return True
    for (left, right) in zip(s, reversed(s)):
        if left != right:
            return False
    return True


# returns a sliding window of size equal to length
def windows(string, length):
    return (string[0+i:length+i] for i in range(0, len(string), 1) if len(string[0+i:length+i]) == length)


def longest_palindrome(s: str) -> str:
    longest = ""
    if len(s) <= 1:
        return s

    for size in range(len(s), 0, -1):
        for window in windows(s, size):
            if is_palindrome(window) and len(window) > len(longest):
                longest = window

    return longest

