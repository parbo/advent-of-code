#!/bin/bash

zpd=$(printf "%02d" $1)
cp -R template $zpd
sed -i '' "s/template/day$zpd/g" "$zpd/Cargo.toml"
