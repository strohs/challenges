# Suppose you dial keys on the keypad using only hops a knight can make. Every time the knight lands on a key, we dial
# that key and make another hop. The starting position counts as being dialed.
#
# How many distinct numbers can you dial in N hops from a particular starting position?
# See https://hackernoon.com/google-interview-questions-deconstructed-the-knights-dialer-f780d516f029
# 6 -> 0,1,7
#   0 -> 4,6
#       4 -> 0,3,9
#       6 -> 0,1,7
#   1 -> 6,8
#       6 -> 0,1,7
#       8 -> 1,3
#   7 -> 2,6
#       2 -> 7,9
#       6 -> 0,1,7
#
# 6,0,4
# 6,0,6
# 6,1,6
# 6,1,8
# 6,7,2
# 6,7,6
#

key_map = {
    1: (6, 8),
    2: (7, 9),
    3: (4, 8),
    4: (0, 3, 9),
    5: tuple(),
    6: (0, 1, 7),
    7: (2, 6),
    8: (1, 3),
    9: (2, 4),
    0: (4, 6)
}


def neighbors(pos):
    return key_map[pos]


def yield_sequences(start_pos, n, cur_seq=None):
    if cur_seq is None:
        cur_seq = [start_pos]

    if n == 0:
        yield cur_seq
        return

    for neighbor in neighbors(start_pos):
        yield from yield_sequences(neighbor, n - 1, cur_seq + [neighbor])


def yield_seq_iter(start_pos, n):
    results = [[start_pos]]

    while n > 0:
        cur_list = []

        for num_list in results:
            # get the next neighbors for each list of numbers
            nn = neighbors(num_list[-1])

            for next_num in nn:
                cur_seq = num_list.copy()
                cur_seq.append(next_num)
                cur_list.append(cur_seq)

        results = cur_list
        n -= 1
    return results


def count_sequences_recursive(start_position, n):
    if n == 0:
        return 1

    num_sequences = 0
    for position in neighbors(start_position):
        num_sequences += count_sequences_recursive(position, n - 1)
    return num_sequences


def count_sequences_iter(start_position, n):
    num_sequences = 0
    sequences = [start_position]
    while n > 0:
        next_seq = []

        for pos in sequences:
            nn = neighbors(pos)
            next_seq += nn

        num_sequences = len(next_seq)
        sequences = next_seq
        n -= 1
    return num_sequences


# for seq in yield_seq_iter(6, 3):
#     print(seq)

# print("----------------")
# for seq in yield_sequences(6, 2):
#     print(seq)
print(count_sequences_iter(6, 20))
# print(count_sequences_recursive(6, 20))
