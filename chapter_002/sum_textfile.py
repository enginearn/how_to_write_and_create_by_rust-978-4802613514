#!/bin/bash/env python3

import sys

total = 0

for i, v in enumerate(sys.argv):
    if i == 0:
        continue
    with open(v, "rt") as fp:
        text = fp.read()
        for line in text.split("\n"):
            try:
                if not line.startswith(";"):
                    total += int(line)
                    continue
            except ValueError:
                pass
print(total)
