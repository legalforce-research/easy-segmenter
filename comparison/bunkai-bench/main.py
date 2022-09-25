#!/usr/bin/env python3

import sys
import time

from bunkai import Bunkai

runs = 10
text = sys.stdin.read()

num_sents = 0
time_start = time.perf_counter()
for _ in range(0, runs):
    bunkai = Bunkai()
    num_sents += len(list(bunkai(text)))
elapsed_sec = time.perf_counter() - time_start

num_sents //= runs
elapsed_sec /= runs

print(f'bunkai: {elapsed_sec * 1000} ms, {num_sents} sentences')
