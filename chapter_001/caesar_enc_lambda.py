#!/bin/bash/env python3

def caesar_enc(text, key):
    a = ord('a')
    conv = lambda x: chr(((ord(x) - a + key) % 26) + a)
    enc1 = lambda x: conv(x) if 'a' <= x <= 'z' else x
    return ''.join([enc1(x) for x in text])

enc = caesar_enc('hello', 1)
dec = caesar_enc(enc, -1)
print(f"{enc} -> {dec}")