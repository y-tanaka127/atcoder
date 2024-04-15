#!/bin/bash

tmp=/tmp/tmp_$$

echo /home/y-tanaka/atcoder/??????/testcases |
tarr - |
while read file
do
	ls ${file} |
	grep '{' |
	ls ${file} |
	sed 's/.yml//g' |
	cat > $tmp-tmp

	#mkdir $(cat $tmp-tmp)

	cat $tmp-tmp |
	joinx - <(echo in out | tarr -) |
	awk '{print $1"/"$2}' |
	awk '{print "'${file}'""/"$1}' |
	yarr - |
	cat > $tmp-hoge

	mkdir -p $(cat $tmp-hoge)
done

rm -rf $tmp-*
exit 0
