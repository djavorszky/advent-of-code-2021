## 🛠️ Task 1

Given a set of parentheses, find invalid ones that contain sequences that are invalid due to incorrect closing braces.

### ❄️ Solution

Use a stack to keep track of the parentheses. Push if opening, pop if closing. Compare the popped opening with the current closing one. If match, continue, otherwise line is corrupt.

#### 🚀 Performance

```
day10 task_1            time:   [75.017 us 75.318 us 75.646 us]
```

## 🛠️ Task 2

Explanation

### ❄️ Solution

Solution

#### 🚀 Performance

Perf
