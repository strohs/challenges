from enum import Enum


class Comparison(Enum):
    Equal = "equal",
    Sublist = "sublist",
    Superlist = "superlist",
    Unequal = "unequal"


def contains_sublist(als: list, bls: list) -> bool:
    if len(als) == 0:
        return False

    # find all indices of als[0] within bls
    indices = list(map(lambda tup: tup[0], filter(lambda tup: tup[1] == als[0], enumerate(bls))))

    for bi in indices:
        bend = bi + len(als)
        if bend > len(bls):
            continue
        bslice = bls[bi:bend]
        if als == bslice:
            return True

    return False


def sublist(first_list: list, second_list: list) -> Comparison:
    first_sub = contains_sublist(first_list, second_list)
    second_sub = contains_sublist(second_list, first_list)

    if first_sub and second_sub:
        return Comparison.Equal
    elif first_sub and not second_sub:
        return Comparison.Sublist
    elif not first_sub and second_sub:
        return Comparison.Superlist
    else:
        return Comparison.Unequal


l1 = [1, 2, 3]
l2 = [4, 5, 6, 1, 2, 9, 1, 2, 3]
l3 = [1, 2, 3]
l4 = [1]
l5 = [2]
r1 = sublist(l1, l2)
r3 = sublist(l1, l3)
r4 = sublist(l2, l4)
r5 = sublist(l4, l5)
print(r1)
print(r3)
print(r4)
print(r5)