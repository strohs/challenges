import unittest
from leetcode.p205_isomorphic_strings import Solution


class Test205Solution(unittest.TestCase):
    def test_is_isomorphic_ex1(self):
        sol = Solution()
        self.assertTrue(sol.isIsomorphic("foo", "add"))

    def test_is_isomorphic_ex2(self):
        sol = Solution()
        self.assertFalse(sol.isIsomorphic("foo", "bar"))

    def test_is_isomorphic_ex3(self):
        sol = Solution()
        self.assertTrue(sol.isIsomorphic("paper", "title"))


if __name__ == '__main__':
    unittest.main()
