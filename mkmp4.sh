#!/bin/sh

BASENAME=$1
FRAMERATE=$2

ffmpeg -framerate ${FRAMERATE} -i "${BASENAME}_%06d.png" -pix_fmt yuv420p -c:v libx264 "${BASENAME}.mp4"
