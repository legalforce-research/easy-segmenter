#!/bin/bash

set -eux

type cargo
type ruby
type python3

text_file=$1
out_base=$2

pushd easy-segmenter-bench
    cargo run --release --bin dump < ../${text_file} > ../${out_base}.easy-segmenter.txt
popd

pushd ja_sentence_segmenter-bench
    python3 dump.py < ../${text_file} > ../${out_base}.ja_sentence_segmenter.txt
popd
