#!/bin/bash

year=$1
zpd=$(printf "%02d" $2)
mkdir -p inputs/$year/$zpd

# Download input
# Put this in .cookie.txt
#  # Netscape HTTP Cookie File
#  .adventofcode.com	TRUE	/	FALSE	0	session	<token-copied-from-browser-devtools>
curl -o inputs/$year/$zpd/input.txt --cookie .cookie.txt -A "mkday.sh by github.com/parbo via cURL" https://adventofcode.com/$year/day/$2/input

cat inputs/$year/$zpd/input.txt
