#!/bin/bash 
#
# abcxxx以下の.gitディレクトリ削除


find /home/y-tanaka/atcoder -type d -name .git -mindepth 2 |
xargs rm -rf 
