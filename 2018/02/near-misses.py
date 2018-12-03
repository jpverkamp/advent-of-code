#!/usr/bin/env python3

import fileinput

def differences(word1, word2):
    count = 0
    for letter1, letter2 in zip(word1, word2):
        if letter1 != letter2:
            count += 1
    return count

def shared_letters(word1, word2):
    output = []
    for letter1, letter2 in zip(word1, word2):
        if letter1 == letter2:
            output.append(letter1)
    return ''.join(output)

words = list(fileinput.input())

for word1 in words:
    for word2 in words:
        if differences(word1, word2) == 1:
            print(shared_letters(word1, word2))
            exit()
