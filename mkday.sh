#!/bin/bash

year=$1
zpd=$(printf "%02d" $2)
mkdir -p $year
cp -R template $year/$zpd
sed -i.bak "s/template/day$zpd/g" "$year/$zpd/Cargo.toml"
rm "$year/$zpd/Cargo.toml.bak"
