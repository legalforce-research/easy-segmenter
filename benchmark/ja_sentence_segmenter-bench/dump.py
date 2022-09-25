#!/usr/bin/env python3

import sys
import time

from ja_sentence_segmenter.common.pipeline import make_pipeline
from ja_sentence_segmenter.split.simple_splitter import split_newline, split_punctuation

text = sys.stdin.read()
segmenter = make_pipeline(split_newline, split_punctuation)
for sent in segmenter(text):
    print(sent)
