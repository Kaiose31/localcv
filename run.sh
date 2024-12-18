#!/bin/bash

# INPUT PARAMS
WIDTH="640"
HEIGHT="480"
DURATION="60"
FPS="10"
# END INPUT PARAMS

python3 scripts/sender.py "$1" "$WIDTH" "$HEIGHT" "$FPS" "$DURATION" &>/dev/null &
PYTHON_PID=$!
cleanup() {
    echo "Terminating Sender"
    kill "$PYTHON_PID" 2>/dev/null
}

trap cleanup EXIT
cargo build -r
OPENCV_FFMPEG_LOGLEVEL=0 ./target/release/localcv "$1" "$WIDTH" "$HEIGHT" $2
