#!/bin/bash/env python3

height_cm = float(input("身長(cm): "))
weight_kg = float(input("体重(kg): "))

height_m = height_cm / 100.0
bmi = weight_kg / (height_m ** 2)

bmi_list = [
    {"min": 0, "max": 18.5, "level": "痩せ型"},
    {"min": 18.5, "max": 25, "level": "標準型"},
    {"min": 25, "max": 30, "level": "肥満（1度）"},
    {"min": 30, "max": 35, "level": "肥満（2度）"},
    {"min": 35, "max": 40, "level": "肥満（3度）"},
    {"min": 40, "max": "", "level": "肥満（4度）"},
]

result = ""
for range in bmi_list:
    if range["min"] <= bmi < range["max"]:
        result = range["level"]

print(f"BMI: {bmi:.2f} {result}")
