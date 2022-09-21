#!/bin/bash/env python3

stack = []

s = input('RPN > ')
tokens = s.split()
for token in tokens:
    token = token.strip()
    try:
        stack.append(float(token))
    except ValueError:
        arg2 = stack.pop()
        arg1 = stack.pop()
        if token == '+':
            stack.append(arg1 + arg2)
        elif token == '-':
            stack.append(arg1 - arg2)
        elif token == '*':
            stack.append(arg1 * arg2)
        elif token == '/':
            stack.append(arg1 / arg2)
        else:
            print('Illegal operator: {}'.format(token))
            exit(1)

print(stack.pop())
