#/bin/sh

for bin in target/release/day*
do
    if [ -r $bin ] && [ -x $bin ]; then
	$bin
    fi
done
