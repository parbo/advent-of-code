#!/bin/sh

BASENAME=$1
FRAMERATE=$2

ffmpeg -i "${BASENAME}_%06d.png" -filter_complex "[0:v] palettegen" "${BASENAME}_palette.png"
ffmpeg -framerate ${FRAMERATE} -i "${BASENAME}_%06d.png" -i "${BASENAME}_palette.png" -filter_complex "[0:v][1:v] paletteuse" "${BASENAME}.gif"
