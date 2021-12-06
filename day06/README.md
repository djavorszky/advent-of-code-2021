# ☀️ Day 1

## 🛠️ Task 1

Fish be reproducing after 6 days, 8 if they are just born. Given set of existing fish, how many fish would there be after 80 days?

### ❄️ Solution

First, the naive, brute-force approach - simulate every iteration by keeping track of all the fishies for each simulation. It's slow, but it gets the work done.

Second, the cleverer one: use a `HashMap` to keep track of how many fish are alive in any given reproductive cycle.

On each simulation step, move the fishies one closer to day 0. The fishes in day0 get special treatment:

1. Their number gets set for the day8 crowd, as offspring
2. They also get added to the day6 crowd (along with day7), as they restart their reproductive cycles.

At the end, sum the number of fishies in the `HashMap` and return that as the result.

#### 🚀 Performance

```
using brute-force vec:
day06 task_1            time:   [1.5374 ms 1.5422 ms 1.5479 ms]

using HashMap:
day06 task_1            time:   [19.582 us 19.619 us 19.660 us]
```

## 🛠️ Task 2

Same as Task 1, but 256 iterations

### ❄️ Solution

Brute force way runs forever, so clever way was utilized, see above.

#### 🚀 Performance

```
day06 task_2            time:   [54.514 us 54.643 us 54.805 us]
```