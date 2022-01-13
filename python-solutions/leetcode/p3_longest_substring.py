# Problem 3: length of longest substring without repeating characters
#
# Given a string `s`, find the length of the longest substring without repeating characters.
# `s` consists of English letters, digits, symbols and spaces.
#
# Example 1
# Input: s = "abcabcbb"
# Output: 3
# Explanation: The answer is "abc", with the length of 3.
#
# Example 3
# Input: s = "pwwkew"
# Output: 3
# Explanation: The answer is "wke", with the length of 3.
# Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        cur_longest = 0
        char_count = 0
        i = 0
        # holds most recent index of char within the input string
        char_map = {}
        while i < len(s):
            ch = s[i]
            if ch not in char_map:
                char_map[ch] = i
                char_count += 1
                i += 1
            else:
                if char_count > cur_longest:
                    cur_longest = char_count
                i = char_map[ch] + 1
                char_map.clear()
                char_count = 0


        if char_count > cur_longest:
            cur_longest = char_count

        return cur_longest


s = "dvdf"
sol = Solution()
print("longest non repeating length for '{}' is {}".format(s, sol.lengthOfLongestSubstring(s)))
