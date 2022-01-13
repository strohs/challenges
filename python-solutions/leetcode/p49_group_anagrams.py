# Given an array of strings strs, group the anagrams together. You can return the answer in any order.
#
# An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically
# using all the original letters exactly once.
#
# Example 1
# Input: strs = ["eat","tea","tan","ate","nat","bat"]
# Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
#
from typing import List, Dict


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        anagram_groups: Dict[str, List[str]] = {}
        for word in strs:
            sword = "".join(sorted(word))
            if sword in anagram_groups:
                anagram_groups[sword].append(word)
            else:
                anagram_groups[sword] = [word]
        return list(anagram_groups.values())


strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
strs2 = ["", ""]
sol = Solution()
print(sol.groupAnagrams(strs2))
