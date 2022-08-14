#!/bin/bash/env python3

a = b = 1
print(a)
for _ in range(10):
    a, b = b, a + b
    print(a)