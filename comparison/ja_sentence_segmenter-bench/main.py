#!/usr/bin/env python3

import sys
import time

from ja_sentence_segmenter.common.pipeline import make_pipeline
from ja_sentence_segmenter.split.simple_splitter import split_newline, split_punctuation

runs = 10
text = sys.stdin.read()

num_sents = 0
time_start = time.perf_counter()
for _ in range(0, runs):
    segmenter = make_pipeline(split_newline, split_punctuation)
    num_sents += len(list(segmenter(text)))
elapsed_sec = time.perf_counter() - time_start

num_sents //= runs
elapsed_sec /= runs

print(f'ja_sentence_segmenter: {elapsed_sec * 1000} ms, {num_sents} sentences')
