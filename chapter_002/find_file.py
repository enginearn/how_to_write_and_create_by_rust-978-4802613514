#!/bin/bash/env python3

import os
import sys
import platform

if len(sys.argv) < 3:
    print("find_file.py <dir> <filename>")
    sys.exit(1)

target_dir = sys.argv[1]
if platform.system() == "Windows" and target_dir[0] != "\\":
    target_dir = target_dir.replace("/", "\\")
else:
    target_dir

keyword = sys.argv[2]

for root, dirs, files in os.walk(target_dir):
    # print(f"root: {root}, dirs: {dirs}, files: {files}")
    for file in files:
        if keyword in file:
            full_path = os.path.join(root, file)
            print(full_path)
