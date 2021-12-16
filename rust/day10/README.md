## 🛠️ Task 1

Given a set of parentheses, find invalid ones that contain sequences that are invalid due to incorrect closing braces.

### ❄️ Solution

Use a stack to keep track of the parentheses. Push if opening, pop if closing. Compare the popped opening with the current closing one. If match, continue, otherwise line is corrupt.

#### 🚀 Performance

```
day10 task_1            time:   [75.017 us 75.318 us 75.646 us]
```

## 🛠️ Task 2

Find the incomplete lines, figure out what it'd take to complete them, then get a score based on the missing characters. Return the score of the middle value.

### ❄️ Solution

Same as task 1, but only care about inputs where there were still elements in the stack after having done parsing the line. That means that there were more open braces than close ones. then map the stack to the closing braces, reverse the list, then apply the scoring formula.

#### 🚀 Performance

```
day10 task_2            time:   [77.153 us 77.554 us 77.950 us]
```
