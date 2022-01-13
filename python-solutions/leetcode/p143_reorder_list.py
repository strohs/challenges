### https://leetcode.com/problems/reorder-list/


# Definition for singly-linked list.
class ListNode(object):
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution(object):
    def reorderList(self, head):
        """
        :type head: ListNode
        :rtype: None Do not return anything, modify head in-place instead.
        """
        # compute the midpoint of the list using slow/fast indices
        slow, fast = head, head
        while fast.next and fast.next.next:
            slow = slow.next
            fast = fast.next.next
        # mid will point to the start of the list to be reversed
        mid = slow.next
        slow.next = None

        # reverse the second list
        p, c, n = None, mid, mid
        while c:
            n = c.next
            c.next = p
            p = c
            c = n

        # merge the two lists, head and p
        x, y, xx, yy = head, p, None, None
        while x and y:
            xx = x.next
            yy = y.next
            x.next = y
            y.next = xx
            x = xx
            y = yy


    def print_list(self, ls: ListNode):
        while ls is not None:
            print(ls.val)
            ls = ls.next


list5 = ListNode(val=1, next=ListNode(val=2, next=ListNode(val=3, next=ListNode(val=4, next=ListNode(val=5)))))
list4 = ListNode(val=1, next=ListNode(val=2, next=ListNode(val=3, next=ListNode(val=4))))
list3 = ListNode(val=1, next=ListNode(val=2, next=ListNode(val=3)))
list2 = ListNode(val=1, next=ListNode(val=2))
list1 = ListNode(val=1)

sol = Solution()
sol.reorderList(list5)
sol.print_list(list5)
print("---------------")
sol.reorderList(list4)
sol.print_list(list4)
print("---------------")
sol.reorderList(list3)
sol.print_list(list3)
print("---------------")
sol.reorderList(list2)
sol.print_list(list2)
print("---------------")
sol.reorderList(list1)
sol.print_list(list1)



## Brute Force Solution
# class Solution(object):
#     def reorderList(self, head):
#         """
#         :type head: ListNode
#         :rtype: None Do not return anything, modify head in-place instead.
#         """
#         while self.three_or_more_nodes(head):
#             n2l = self.next_to_last(head)
#             last = n2l.next
#             n2l.next = last.next
#             self.insert_after(head, last)
#             head = head.next.next
#
#
#     def print_list(self, ls: ListNode):
#         while ls is not None:
#             print(ls.val)
#             ls = ls.next
#
#
#     def three_or_more_nodes(self, head: ListNode) -> bool:
#         return head is not None and head.next is not None and head.next.next is not None
#
#
#     def next_to_last(self, head: ListNode) -> ListNode | None:
#         """
#         returns the next to last node of a linked list. if list is empty or only has one element
#         None is returned
#         :param head:
#         """
#         if head is not None and head.next is not None:
#             while head.next is not None and head.next.next is not None:
#                 head = head.next
#             return head
#         else:
#             return None
#
#
#     def insert_after(self, node: ListNode, target: ListNode):
#         """ inserts target node after node """
#         target.next = node.next
#         node.next = target
