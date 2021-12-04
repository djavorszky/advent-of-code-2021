# ☀️ Day 4

## 🛠️ Task 1

Play Bingo

### ❄️ Solution

On each instruction update all of the boards.

#### 🚀 Performance

```
using Vec to store marked fields:
day04 task_1            time:   [564.67 us 566.85 us 569.29 us]

using HashSet to store marked fields:
day04 task_1            time:   [909.68 us 911.70 us 913.82 us]

```


## 🛠️ Task 2

Play Bingo until the last board wins

### ❄️ Solution

Same as Task 1, but keep track if a board has already won. If it did, skip it. Otherwise go until the last board won.

#### 🚀 Performance

```
using Vec to store marked fields
day04 task_2            time:   [2.0388 ms 2.0446 ms 2.0508 ms]

Using HashSet to store marked fields
day04 task_2            time:   [2.2377 ms 2.2427 ms 2.2481 ms]
```