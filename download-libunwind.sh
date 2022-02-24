#!/usr/bin/env sh

VERSION=1.6.2

curl -L https://github.com/libunwind/libunwind/releases/download/v$VERSION/libunwind-$VERSION.tar.gz > ./libunwind.tar.gz
tar xvf libunwind.tar.gz
rm ./libunwind.tar.gz
mv libunwind-$VERSION libunwind