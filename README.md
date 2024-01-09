Solved parts 1 & 2

The main effort was building the logic to shift the grid. Part 1 could be solved just by running that shifting function n times, part 2 required observing when a certain cycle (excluding the first few rotations, i.e. before it gets into a cycle) was achieved and just running the shift function n % the length of the cycles.