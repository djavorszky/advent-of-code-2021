## 🛠️ Task 1

Find all the local minimums on a 2d grid

### ❄️ Solution

Parse input data into a helper `Map` struct, then iterate over all the fields and check if any of its neighbours are lower. If so, it's not a local minimum, otherwise it is.

#### 🚀 Performance

```
day09 task_1            time:   [172.66 us 176.93 us 181.45 us]
```

## 🛠️ Task 2

Get the basin sizes

### ❄️ Solution

Recursive function that walks through the basin. Uses a `HashSet` to keep track of which indexes it has already seen.

#### 🚀 Performance

```
day09 task_2            time:   [933.36 us 939.39 us 945.88 us]
```