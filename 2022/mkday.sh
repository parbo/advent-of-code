#!/bin/bash

zpd=$(printf "%02d" $1)
cp -R template $zpd
sed -i "s/template/day$zpd/g" $zpd/Cargo.toml

# Download input
# Put this in .cookie.txt
#  # Netscape HTTP Cookie File
#  .adventofcode.com	TRUE	/	FALSE	0	session	<token-copied-from-browser-devtools>
curl -o $zpd/input.txt --cookie .cookie.txt -A "mkday.sh by github.com/parbo via cURL" https://adventofcode.com/2022/day/$1/input

cat $zpd/input.txt
