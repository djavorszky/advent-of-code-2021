from util import chunks


def task1(boards):
    """
    Find the first bingo board that wins.
    >>> with open("inputs/test/day04.txt") as file:
    ...     task1([line.rstrip() for line in file.readlines()])
    4512
    """

    numbers = [int(n) for n in boards[0].split(",")]
    boards = list(build_boards(boards[1:]))

    for number in numbers:
        for board in boards:
            mark_number(board, number)

            if has_won(board):
                return calculate_win(board, number)

    pass


def calculate_win(board, number):
    return sum([n for n in board['nums'] if n not in board['called_nums']]) * number


def has_won(board):
    if len(board['called_nums']) < 5:
        return False

    return has_h_bingo(board) or has_v_bingo(board)


def has_h_bingo(board):
    for line in chunks(board['nums'], 5):
        if all([n in board['called_nums'] for n in line]):
            return True

    return False


def has_v_bingo(board):
    for start in range(0, 5):
        if all([n in board['called_nums'] for n in board['nums'][start::5]]):
            return True

    return False


def mark_number(board, number):
    board['called_nums'].add(number)


def build_boards(boards):
    for board in chunks(boards, 6):
        yield dict(
            called_nums=set(),
            nums=[int(n) for line in board if line != ""
                  for n in line.split(" ") if n != ""],
            won=False
        )


def task2(boards):
    """
    Find the last bingo board that wins.
    >>> with open("inputs/test/day04.txt") as file:
    ...     task2([line.rstrip() for line in file.readlines()])
    1924
    """

    numbers = [int(n) for n in boards[0].split(",")]
    boards = list(build_boards(boards[1:]))

    for number in numbers:
        boards = list(filter(lambda b: b['won'] is False, boards))
        for board in boards:
            mark_number(board, number)

            if has_won(board):
                board['won'] = True
                if len(boards) == 1:
                    return calculate_win(board, number)

    pass


if __name__ == "__main__":
    with open("inputs/day04.txt") as file:
        lines = [line.rstrip() for line in file.readlines()]

        print("task1:", task1(lines))
        print("task2:", task2(lines))
