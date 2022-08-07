class Solution:
    def simplifyPath(self, s: str) -> str:
        stack = []
        splits = s.split("/")

        for segment in splits:
            if segment == "" or segment == ".":
                pass
            elif segment == "..":
                if len(stack) > 0:
                    stack.pop()
            else:
                stack.append(segment)

        return "/" + "/".join(stack)


if __name__ == "__main__":
    sol = Solution()
    s = "/abc/123/././xyz"
    res = sol.simplifyPath(s)
    assert res == "/abc/123/xyz"

    s = "/../../../.."
    res = sol.simplifyPath(s)
    assert res == "/"

    s = "////"
    res = sol.simplifyPath(s)
    assert res == "/"

    s = "/"
    res = sol.simplifyPath(s)
    assert res == "/"

    s = "/../"
    res = sol.simplifyPath(s)
    assert res == "/"
