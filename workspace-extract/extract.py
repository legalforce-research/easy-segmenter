#!/usr/bin/env python3

import sys

for line in sys.stdin:
    word = line.split(',')[0]
    if '。' in word or '．' in word:
        print(word)
