#/bin/sh

for bin in */*.rs
do
    echo $bin
    cargo run --release --bin day"$(dirname "$bin")" -- $args
done
