#!/bin/bash/env python3

import random

MAP_N = 25

maze = []
for _ in range(0, MAP_N):
    maze.append([0 for x in range(0, MAP_N)])

for i in range(0, MAP_N):
    maze[i][0] = maze[i][MAP_N - 1] = 1
    maze[0][i] = maze[MAP_N - 1][i] = 1

for i in range(2, MAP_N - 2):
    for j in range(2, MAP_N - 2):
        if j % 2 == 1 or i % 2 == 1:
            continue
        maze[i][j] = 1

        r = random.randint(0, 3)
        if r == 0: maze[j - 1][i] = 1
        if r == 1: maze[j + 1][i] = 1
        if r == 2: maze[j][i - 1] = 1
        if r == 3: maze[j][i + 1] = 1

tiles = ["  ", "##"]
for i in range(0, MAP_N):
    for j in range(0, MAP_N):
        print(tiles[maze[i][j]], end="")
    print("")
