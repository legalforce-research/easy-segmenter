# comparison

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
