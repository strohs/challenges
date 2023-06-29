def permute(s):
    perm_rec("", s)


def perm_rec(perm, rest):
    if len(rest) == 0:
        print(perm)
    else:
        for i in range(0, len(rest)):
            ch = rest[i]
            new_rest = rest[0:i] + rest[i+1:]
            perm_rec(perm + ch, new_rest)


permute("abc")
