import unittest
import p5_longest_palindromic_substr as ps


class TestP5(unittest.TestCase):
    def test_example1(self):
        s = "babad"
        self.assertEqual(ps.longest_palindrome(s), 'bab')


if __name__ == '__main__':
    unittest.main()
