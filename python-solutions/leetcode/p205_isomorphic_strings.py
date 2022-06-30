class Solution:

    def isIsomorphic(self, s: str, t: str) -> bool:
        char_map = dict()
        used_values = set()

        for s_char, t_char in zip(s, t):
            if s_char not in char_map and t_char not in used_values:
                char_map[s_char] = t_char
                used_values.add(t_char)
            elif s_char not in char_map and t_char in used_values:
                # t_char is already mapped but no two chars may map to the same char
                return False
            elif s_char in char_map:
                # char_map must continue to map to the same char
                if char_map[s_char] != t_char:
                    return False

        return True


