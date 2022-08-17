#!/bin/bash/env python3

import random

nums = list(range(1, 76))

random.shuffle(nums)

nums[12] = "★"

for x in range(0, 5):
    for y in range(0, 5):
        print(f"{nums[x*5+y]: >3} ", end="")
    print("")