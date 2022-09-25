#!/bin/bash

set -eux

text_file="gakumon-no-susume.txt"

pushd easy-segmenter-bench
    cargo run --release < ../${text_file}
popd