# ☀️ Day 4

## 🛠️ Task 1

Play Bingo

### ❄️ Solution

On each instruction update all of the boards.

#### 🚀 Performance

```
using Vec to store marked fields:
day04 task_1            time:   [5.4346 us 5.4833 us 5.5388 us]

using HashSet to store marked fields:
day04 task_1            time:   [11.422 us 11.449 us 11.481 us]


```


## 🛠️ Task 2

Play Bingo until the last board wins

### ❄️ Solution

Same as Task 1, but keep track if a board has already won. If it did, skip it. Otherwise go until the last board won.

#### 🚀 Performance

```
day04 task_2            time:   [5.6001 us 5.6300 us 5.6607 us]
```