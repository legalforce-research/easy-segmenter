#!/bin/bash

set -eux

type cargo
type ruby

text_file="gakumon-no-susume.txt"

pushd easy-segmenter-bench
    cargo run --release < ../${text_file}
popd

pushd pragmatic_segmenter-bench
    ruby main.rb < ../${text_file}
popd
