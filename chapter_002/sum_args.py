#!/bin/bash/env python3

import sys
total = 0

for i, v in enumerate(sys.argv):
    if i == 0:
        continue

    try:
        total += float(v)
    except ValueError:
        pass
print(total)
