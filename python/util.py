import unittest
from typing import List, Tuple, Generator


def chunks(input_list: List, n: int) -> Generator:
    """Generator; Yields the list in n-sized chunks.
    """
    for i in range(0, len(input_list), n):
        yield input_list[i:i + n]


def traverse(p1: Tuple[int, int], p2: Tuple[int, int]) -> Generator:
    """
    Generates the steps between p1 and p2, which are two-element int tuples
    """
    x1, y1 = p1[0], p1[1]
    x2, y2 = p2[0], p2[1]

    step_x = -1 if x1 > x2 else 1
    step_y = -1 if y1 > y2 else 1

    if y1 == y2:
        for x in range(x1, x2 + step_x, step_x):
            yield x, y1
    elif x1 == x2:
        for y in range(y1, y2 + step_y, step_y):
            yield x1, y
    else:
        step_count = abs(x1 - x2)
        for s in range(step_count + 1):
            yield x1 + s * step_x, y1 + s * step_y


def point2idx(x: int, y: int, width: int) -> int:
    """
    Returns a 2d point to a 1d index.
    """

    return x + y * width


class UtilTests(unittest.TestCase):

    def test_chunk(self):
        res = list(chunks([1, 2, 3, 4, 5, 6, 7, 8], 3))
        exp = [[1, 2, 3], [4, 5, 6], [7, 8]]

        self.assertEqual(exp, res)

    def test_traverse_x_inc(self):
        p1, p2 = (0, 9), (3, 9)
        exp = [(0, 9), (1, 9), (2, 9), (3, 9)]
        self.assertEqual(exp, list(traverse(p1, p2)))

    def test_traverse_x_dec(self):
        p1, p2 = (3, 9), (0, 9)
        exp = [(3, 9), (2, 9), (1, 9), (0, 9)]
        self.assertEqual(exp, list(traverse(p1, p2)))

    def test_traverse_y_inc(self):
        p1, p2 = (0, 3), (0, 5)
        exp = [(0, 3), (0, 4), (0, 5)]
        self.assertEqual(exp, list(traverse(p1, p2)))

    def test_traverse_y_dec(self):
        p1, p2 = (0, 5), (0, 3)
        exp = [(0, 5), (0, 4), (0, 3), ]
        self.assertEqual(exp, list(traverse(p1, p2)))

    def test_point2idx(self):
        cases = [
            (3, 5, 4, 23),
            (3, 0, 4, 3),
            (0, 2, 5, 10),
            (0, 0, 10, 0)
        ]

        for (x, y, w, exp) in cases:
            self.assertEqual(exp, point2idx(x, y, w))
