## 🛠️ Task 1

In a consortium of 10x10 octopi, each one flashes once every step if their energy level is greater than 9. Their energy level increases automatically once every step, as well as if any neighbour flashes. They only flash once.

Figure out how many flashes happened after 100 steps.

### ❄️ Solution

Simulate them.

#### 🚀 Performance

```
initial:
day11 task_1            time:   [284.79 us 286.62 us 288.85 us]

refactoring to get rid of a .collect():
day11 task_1            time:   [215.43 us 215.82 us 216.27 us]
                        change: [-20.665% -20.135% -19.483%] (p = 0.00 < 0.05)
```

## 🛠️ Task 2

Same as before, but figure out which step they all flash at the same time.

### ❄️ Solution

Simulate until the flash_count is 100

#### 🚀 Performance

```
initial:
day11 task_2            time:   [802.53 us 804.91 us 807.39 us]

refactoring to get rid of a .collect():
day11 task_2            time:   [602.19 us 603.53 us 605.11 us]
                        change: [-22.953% -21.757% -20.512%] (p = 0.00 < 0.05)
```
