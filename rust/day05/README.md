# ☀️ Day 5

## 🛠️ Task 1

Given a set of from-to coordinates that denote lines (start-end points), return the number of times where vertical and horizontal lines overlap.

### ❄️ Solution

To simplify processing the data, assume that all lines have a direction (and, therefore, are vectors). All such vectors are strictly increasing - so either their X, Y, or both coordinates are increasing when going from the `start` to the `end` `Point`.

For this, a couple of things needed to be implemented, most importantly the `Ord` trait for `Point`, which states that a `Point` is "greater than" another `Point` if it's higher in the `y` axis, otherwise higher in the `x` axis. This way we can order the the points in the instruction, and ensure that we can iterate over the coordinates of a line by only increasing counters.

Other than that, it's a naive approach: Have an array of length `n*n`, initialized to all `0`s, where `n` is the largest coordinate in any instructions (`991`), and for each point on a (horizontal + vertical) `Line`, increment the corresponding grid point's value by one.

Return the count of all items where the value is higher than 1.


#### 🚀 Performance

```
day05 task_1            time:   [892.65 us 895.57 us 898.87 us]
```
## 🛠️ Task 2

Same as Task 1, but also consider diagonals.

### ❄️ Solution

Same as Task 1, but also consider diagonals.

#### 🚀 Performance

```
day05 task_2            time:   [886.28 us 888.01 us 889.75 us]
```
