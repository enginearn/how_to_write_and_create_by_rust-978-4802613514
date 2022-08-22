#!/bin/bash/env python3

import time

SEED = 0

def rand(start, end):
    global SEED
    if SEED == 0:
        SEED = int(time.time() * 1000)

    SEED ^= (SEED << 13) & 0xffffffff
    SEED ^= (SEED >> 17) & 0xffffffff
    SEED ^= (SEED << 5) & 0xffffffff
    return (SEED % (end - start + 1)) + start

for _ in range(100):
    print(rand(1, 6))
