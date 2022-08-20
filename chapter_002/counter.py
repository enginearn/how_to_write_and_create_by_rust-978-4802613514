#!/bin/bash/env python3

import random
import string

alphabet = string.ascii_uppercase
print(f"alphabet: {type(alphabet[:3])}")
print(f"alphabet: {alphabet[:3]}")
abc = alphabet[:3]
randlist = [random.choice(string.ascii_uppercase) for i in range(10)]
randlist = [random.choice(abc) for i in range(30)]
randstr = ','.join(randlist)
print(randlist)
print(randstr)
abc = ','.join(random.sample(abc, len(abc)))
print(f"abc: {abc}")

def random_name(n) -> str:
    return ''.join(random.choices(string.ascii_letters + string.digits, k=n))

V_DATA = randstr
# print(f"V_DATA: {V_DATA}")
c_dict = {}
for w in V_DATA.split(","):
    if w in c_dict:
        c_dict[w] += 1
    else:
        c_dict[w] = 1
print(c_dict)

for key, value in c_dict.items():
    print(f"{key}:{value}")
