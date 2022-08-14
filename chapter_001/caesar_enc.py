#!/bin/bash/env python3


def caesar_enc(text, key):
    """
    Caesar cipher encryption
    """
    result = ""
    for c in text:
        if c.isalpha():
            result += chr((ord(c) - ord('a') + key) % 26 + ord('a'))
        else:
            result += c
    return result

def caesar_dec(text, key):
    """
    Caesar cipher decryption
    """
    return caesar_enc(text, -key)

text = input("Enter text: ")
enc = caesar_enc(text.lower(), 3)
dec = caesar_dec(enc, 3)
print(f"Encrypted: {enc}")
print(f"Decrypted: {dec}")