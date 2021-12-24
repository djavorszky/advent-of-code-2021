from math import prod


def task1(displays):
    """
    Figure out how many 1, 4, 7, and 8 displays are active.
    >>> with open("inputs/test/day08.txt") as file:
    ...     task1([line.rstrip() for line in file.readlines()])
    26
    """

    return len([c for display in displays for _, c in parse_numbers(display) if c in [1, 4, 7, 8]])


primes = {
    'a': 2,
    'b': 3,
    'c': 5,
    'd': 7,
    'e': 11,
    'f': 13,
    'g': 17,
}


def parse_numbers(display):
    def hash(s):
        return prod([primes[c] for c in s])

    def decipher_signal(signal):
        result = dict()

        one = set(next(x for x in signal.split(" ") if len(x) == 2))
        four = set(next(x for x in signal.split(" ") if len(x) == 4))

        for s in signal.split(" "):
            val = set(s)
            index = 0
            match len(s):
                case 2:
                    index = 1
                case 3:
                    index = 7
                case 4:
                    index = 4
                case 7:
                    index = 8
                case 5 if len(one & val) == len(one):
                    index = 3
                case 5 if len(four & val) == 2:
                    index = 2
                case 5:
                    index = 5
                case 6 if len(four & val) == len(four):
                    index = 9
                case 6 if len(one & val) == len(one):
                    index = 0
                case 6:
                    index = 6

            result[hash(s)] = index

        return result

    def get_number(segment, signal):
        return signal[hash(segment)]

    (tmpl, numbers) = display.split(" | ")

    signal = decipher_signal(tmpl)

    for idx, n in enumerate(numbers.split(" ")):
        yield idx, get_number(n, signal)


def task2(displays):
    """
    Figure out the sum of the displays.
    >>> with open("inputs/test/day08.txt") as file:
    ...     task2([line.rstrip() for line in file.readlines()])
    61229
    """

    result = 0
    for display in displays:
        for idx, c in parse_numbers(display):
            result += c * pow(10, 3 - idx)

    return result


if __name__ == "__main__":
    with open("inputs/day08.txt") as file:
        lines = [line.rstrip() for line in file.readlines()]

        print("task1:", task1(lines))
        print("task2:", task2(lines))
