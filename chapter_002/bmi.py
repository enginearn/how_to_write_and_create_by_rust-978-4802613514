#!/bin/bash/env python3

height_cm = float(input("身長(cm): "))
weight = float(input("体重(kg): "))

bmi = weight / (height_cm / 100) ** 2
print(f"BMI: {bmi:.2f}")

if bmi < 18.5:
    print("低体重")
elif bmi < 25:
    print("標準")
elif bmi < 30:
    print("肥満(1度)")
elif bmi < 35:
    print("肥満(2度)")
elif bmi < 40:
    print("肥満(3度)")
else:
    print("肥満(4度)")
