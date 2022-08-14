#!/bin/bash/env python3

for i in range(1, 10):
    for j in range(1, 10):
        # print(f"{i} * {j} = {i * j}", end="")
        print(f"{i * j :3}", end="")
    print("")

print("===================================")

for i in range(1, 10):
    a = [f"{i * j :3}" for j in range(1, 10)]
    print(",".join(a))