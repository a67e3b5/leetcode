#
# @lc app=leetcode id=167 lang=python3
#
# [167] Two Sum II - Input Array Is Sorted
#


# @lc code=start
from collections import deque


class Solution:
    def twoSum(self, numbers: list[int], target: int) -> list[int]:
        l = len(numbers)
        even = list(filter(lambda x: x[1] % 2 == 0, enumerate(numbers)))
        odd = list(filter(lambda x: x[1] % 2 == 1, enumerate(numbers)))
        le = len(even)
        lo = len(odd)
        if target % 2 == 1:
            stack = deque([(0, 0, le - 1, lo - 1)])
            while True:
                i, j, di, dj = stack.pop()
                s = even[i][1] + odd[j][1]
                if s < target:
                    if j + dj < lo and i != j + dj:
                        stack.append((i, j + dj, di, dj // 2))
                    if i + di < le and i + di != j:
                        stack.append((i + di, j, di // 2, dj))
                elif s > target:
                    if j - dj >= 0 and i != j - dj:
                        stack.append((i, j + dj, di, dj // 2))
                    if i - di >= 0 and i - di != j:
                        stack.append((i + di, j, di // 2, dj))
                else:
                    i, j = even[i][0] + 1, odd[j][0] + 1
                    return [min(i, j), max(i, j)]
        else:
            stack = deque([(0, le - 1, le - 1 // 2, le - 1 // 2)])
            while True:
                try:
                    i, j, di, dj = stack.pop()
                except IndexError:
                    break
                s = even[i][1] + even[j][1]
                if s < target:
                    if j + dj < le and i != j + dj:
                        stack.append((i, j + dj, di, dj // 2))
                    if i + di < le and i + di != j:
                        stack.append((i + di, j, di // 2, dj))
                elif s > target:
                    if j - dj >= 0 and i != j - dj:
                        stack.append((i, j + dj, di, dj // 2))
                    if i - di >= 0 and i - di != j:
                        stack.append((i + di, j, di // 2, dj))
                else:
                    i, j = even[i][0] + 1, even[j][0] + 1
                    return [min(i, j), max(i, j)]

            stack = deque([(0, lo - 1, lo - 1 // 2, lo - 1 // 2)])
            while True:
                try:
                    i, j, di, dj = stack.pop()
                except IndexError:
                    break
                s = odd[i][1] + odd[j][1]
                if s < target:
                    if j + dj < lo and i != j + dj:
                        stack.append((i, j + dj, di, dj // 2))
                    if i + di < lo and i + di != j:
                        stack.append((i + di, j, di // 2, dj))
                elif s > target:
                    if j - dj >= 0 and i != j - dj:
                        stack.append((i, j + dj, di, dj // 2))
                    if i - di >= 0 and i - di != j:
                        stack.append((i + di, j, di // 2, dj))
                else:
                    i, j = odd[i][0] + 1, odd[j][0] + 1
                    return [min(i, j), max(i, j)]

# @lc code=end

s = Solution()
# assert s.twoSum(numbers=[2, 7, 11, 15], target=9) == [1, 2]
# assert s.twoSum(numbers=[2, 3, 4], target=6) == [1, 3]
# assert s.twoSum(numbers=[-1, 0], target=-1) == [1, 2]
assert s.twoSum([0, 0, 3, 4], 0) == [1, 2]
