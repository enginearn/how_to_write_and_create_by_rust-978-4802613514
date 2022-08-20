#!/bin/bash/env python3

import sys

dict_file = "./ejdict-hand-utf8.txt"

if len(sys.argv) != 2:
    print("Usage: {} <word>".format(sys.argv[0]))
    sys.exit(1)
word = sys.argv[1]

with open(dict_file, "r") as f:
    while True:
        line = f.readline()
        if not line:
            break
        if word in line:
            print(line.strip())
