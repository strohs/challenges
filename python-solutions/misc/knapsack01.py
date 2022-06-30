from dataclasses import dataclass
from functools import reduce


@dataclass
class Item:
    """
    Keeps track of the details of an item.
    index is the items index within the weights/values list
    weight is the items weight
    value is the items value
    """
    index: int
    weight: int
    value: int


def solve(weights: [int], values: [int], max_weight: int) -> [(int, int, int)]:
    """
    solves the 01-knapsack problem by finding the items with the maximum value
    that do not exceed max-weight
    :param weights: weights of the items
    :param values: values of the items
    :param max_weight: the maximum weight that should not be exceeded
    :return: a list of Item containing the items with the optimal values
    """
    length = len(values)
    # dp is the dynamic programming matrix that will be used to compute optimal value and items
    dp = [[0 for w in range(0, max_weight + 1)] for item in range(0, length + 1)]

    for i in range(1, length + 1):
        for w in range(0, max_weight + 1):
            if weights[i-1] <= w:
                # curr item fits, so best value is curr items value plus the value of the the previous items
                # at index: dp[ii-1][wi - curr_weight]
                curr_weight = weights[i - 1]
                dp[i][w] = values[i - 1] + dp[i - 1][w - curr_weight]
            else:
                # curr item won't fit in curr weight so best value is the item value in the row above
                dp[i][w] = dp[i-1][w]

    # best possible value is in the bottom right corner of the dp matrix

    # now find the actual items, by iterating thru the rows in reverse order. Stop iterating when the item index == 0
    # Include an item if the curr value and the value in the row above it differ.
    # If the item is included, subtract its weight from wi to get the next wi index to look at (in the row above)
    # If the curr value and the value in the row above are the same, the curr item is not included and we simply
    # move up one row to examine the previous item

    items: [Item] = []
    wi = max_weight
    for i in reversed(range(1, length+1)):
        curr_value = dp[i][wi]
        if curr_value != dp[i-1][wi]:
            items.append(Item(i-1, weights[i-1], values[i-1]))
            wi -= weights[i-1]

    return items


if __name__ == "__main__":
    weights = [3, 1, 3, 4, 2]
    values = [2, 2, 4, 5, 3]
    items = solve(weights, values, 7)
    print("optimal items:")
    for item in items:
        print(f"   {item}")

    # reducing each items weight and value into a temporary Item() class that holds the max values
    max_item = reduce(lambda acc,i: Item(0, acc.weight + i.weight, acc.value + i.value), items, Item(0, 0, 0))
    print(f"  with max weight: {max_item.weight}  max_value:{max_item.value}")


