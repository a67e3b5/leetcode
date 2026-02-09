#
# @lc app=leetcode id=83 lang=python3
#
# [83] Remove Duplicates from Sorted List
#


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# @lc code=start
class Solution:
    def deleteDuplicates(self, head: ListNode | None) -> ListNode | None:
        dummy = ListNode()
        cur = dummy
        while head:
            while head.next and head.val == head.next.val:
                head = head.next
            cur.next = head
            cur = cur.next
            head = head.next
        return dummy.next


# @lc code=end
