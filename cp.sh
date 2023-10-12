#!/bin/bash
#

homd=/home/y-tanaka
atcd=$homd/atcoder
tmp=/tmp/tmp_$$

base_d=$1
copy_d=$2

mkdir -p ${copy_d}

fd -H -d 1 -E 'target' . ${base_d}                  |
tee >(sed "s;${base_d};${copy_d};g" > $tmp-cp_list) |
cat > $tmp-list

paste $tmp-list $tmp-cp_list |
xargs -n2 cp -pr

ln -s $atcd/$base_d/target $atcd/$copy_d/target

rm -rf $tmp-*
