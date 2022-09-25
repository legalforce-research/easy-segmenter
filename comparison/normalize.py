#!/usr/bin/env python3

import sys
import time

from ja_sentence_segmenter.normalize.neologd_normalizer import normalize

text = sys.stdin.read()
for t in normalize(text):
    print(t)
