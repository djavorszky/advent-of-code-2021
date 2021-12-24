import unittest
from typing import List, Generator


def chunks(input_list: List, n: int) -> Generator:
    """Generator; Yields the list in n-sized chunks.
    """
    for i in range(0, len(input_list), n):
        yield input_list[i:i + n]


class UtilTests(unittest.TestCase):

    def test_chunk(self):
        res = list(chunks([1, 2, 3, 4, 5, 6, 7, 8], 3))
        exp = [[1, 2, 3], [4, 5, 6], [7, 8]]

        self.assertEqual(exp, res)
