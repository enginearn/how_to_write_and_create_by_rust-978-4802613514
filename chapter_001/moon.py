#!/usr/bin/env python3

import math

moon = 384_400
car = 80
train = 300
hour = 24

print(f"take {moon / car / hour} days to go around the moon by car.")
print(f"take {moon / train / hour} days to go around the moon by train.")
print(f"take {math.floor(moon / car / hour):.2f} days to go around the moon by car.")
print(f"take {math.floor(moon / train / hour):2f} days to go around the moon by train.")
print(f"take {round(moon / car / hour, 2)} days to go around the moon by car.")
print(f"take {round(moon / train / hour, 2)} days to go around the moon by train.")