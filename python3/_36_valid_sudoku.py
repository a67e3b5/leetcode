#
# @lc app=leetcode id=36 lang=python3
#
# [36] Valid Sudoku
#


# @lc code=start
class Solution:
    def isValidSudoku(self, board: list[list[str]]) -> bool:
        row = {i: set([]) for i in range(9)}
        column = {j: set([]) for j in range(9)}
        box = {(x, y): set([]) for x in range(3) for y in range(3)}
        for i in range(9):
            for j in range(9):
                v = board[i][j]
                if v == ".":
                    continue
                try:
                    row[i].remove(v)
                    return False
                except KeyError:
                    row[i].add(v)
                try:
                    column[j].remove(v)
                    return False
                except KeyError:
                    column[j].add(v)
                x = j // 3
                y = i // 3
                try:
                    box[(x, y)].remove(v)
                    return False
                except KeyError:
                    box[(x, y)].add(v)
        return True


# @lc code=end

s = Solution()
assert s.isValidSudoku(
    board=[
        ["5", "3", ".", ".", "7", ".", ".", ".", "."],
        ["6", ".", ".", "1", "9", "5", ".", ".", "."],
        [".", "9", "8", ".", ".", ".", ".", "6", "."],
        ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
        ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
        ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
        [".", "6", ".", ".", ".", ".", "2", "8", "."],
        [".", ".", ".", "4", "1", "9", ".", ".", "5"],
        [".", ".", ".", ".", "8", ".", ".", "7", "9"],
    ]
)
assert not s.isValidSudoku(
    board=[
        ["8", "3", ".", ".", "7", ".", ".", ".", "."],
        ["6", ".", ".", "1", "9", "5", ".", ".", "."],
        [".", "9", "8", ".", ".", ".", ".", "6", "."],
        ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
        ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
        ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
        [".", "6", ".", ".", ".", ".", "2", "8", "."],
        [".", ".", ".", "4", "1", "9", ".", ".", "5"],
        [".", ".", ".", ".", "8", ".", ".", "7", "9"],
    ]
)
