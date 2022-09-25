# easy-segmenter/comparison

Here provides speed comparision of several segmenters.

## Preparation

You need to install other tools.

```
$ python -m pip install ja_sentence_segmenter
$ python -m pip install bunkai
$ gem install pragmatic_segmenter
```

## How to bench

To measure segmentation times of all methods,

```
$ ./run.sh ../data/gakumonno_susume.txt
```

To output segmented results,

```
$ ./dump.sh ../data/gakumonno_susume.txt gakumon
$ diff --strip-trailing-cr gakumon.easy-segmenter.txt gakumon.ja_sentence_segmenter.txt
# no diff
```
