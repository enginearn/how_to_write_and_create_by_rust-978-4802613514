#!/usr/bin/env python3

import numpy as np
import sys
import matplotlib.pyplot as plt

if len(sys.argv) < 2: quit()

x, y = ([], [])
lines = open(sys.argv[1], 'r').read().split('\n')

for i, v in enumerate(lines):
    if v != '':
        x.append(i)
        y.append(float(v))
    if i > 1000: break

plt.plot(x, y)
plt.show()
