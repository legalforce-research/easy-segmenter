#!/bin/bash

set -eux

type cargo
type ruby
type python3

text_file="gakumon-no-susume.txt"

pushd easy-segmenter-bench
    cargo run --release < ../${text_file}
popd

pushd pragmatic_segmenter-bench
    ruby main.rb < ../${text_file}
popd

pushd ja_sentence_segmenter-bench
    python3 main.py < ../${text_file}
popd
