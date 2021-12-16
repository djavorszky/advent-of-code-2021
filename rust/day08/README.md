## 🛠️ Task 1

7 segment display got messed up, shows random values. Figure out based on input what the correct values are;
https://adventofcode.com/2021/day/8.

For part 1, only figure out how many times the numbers 1, 3, 4, 7 occur in the inputs.

### ❄️ Solution

Just.. count the jumpled inputs that have that many letters.


#### 🚀 Performance

```
day08 task_1            time:   [22.987 us 23.360 us 23.783 us]
```

## 🛠️ Task 2

Figure out which numbers there are, then calculate their sum.


### ❄️ Solution

In all seriousness, knowing 4 values just by their lengths, I can use them as masks to apply to different length numbers.

For example, when looking at numbers that have 5 length (2, 3, and 5), I can apply 1 an 4 as masks to get the three unknown ones:

```
Masks:

# #   #
###   #
  #   #

Numbers:

###  ###  ###
#      #    #
###  ###  ###
  #  #      #
###  ###  ###

```

In the above scenario, we can figure out which of the numbers we are looking at by checking the following:

1. Does 1 overlap completely with the number? If so, it's a 3. Otherwise:
2. Does 4 only have 2 overlapping faces with the number? If so, it's a 2. Otherwise, a 5.

This can also be applied to numbers that have 6 lenghts.

Once we're done with building the numbers, we hash the input values (from which we realized what the numbers are) via replacing the letters with prime numbers and getting their products. We then do the same with numbers we should calculate and compare.

#### 🚀 Performance

```
day08 task_2            time:   [263.01 us 266.50 us 270.57 us]
```
