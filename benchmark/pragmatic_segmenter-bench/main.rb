#!/usr/bin/env ruby

require 'benchmark'
require "pragmatic_segmenter"

text = $stdin.read

num_sents = 0
elapsed_sec = Benchmark.realtime do
    for _ in 1..100
        ps = PragmaticSegmenter::Segmenter.new(text: text, language: 'ja')
        num_sents += ps.segment.size
    end    
end

num_sents /= 100
elapsed_sec /= 100

puts "pragmatic_segmenter: #{elapsed_sec * 1000} ms, #{num_sents} sentences"
