#!/usr/bin/env python3

a_file = "./fizzbuzz_python.txt"
b_file = "./fizzbuzz_rust.txt"

with open(a_file, "r") as f:
    a_lines = f.read()

with open(b_file, "r") as f:
    b_lines = f.read()

a_lines = a_lines.split()
b_lines = b_lines.split()

if a_lines == b_lines:
    print("OK")
else:
    print("NG")
