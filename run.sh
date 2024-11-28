#!/bin/bash
WIDTH="640"
HEIGHT="480"
python3 scripts/sender.py "$1" "$WIDTH" "$HEIGHT" &>/dev/null &
PYTHON_PID=$!
cleanup() {
    echo "Terminating Sender"
    kill "$PYTHON_PID" 2>/dev/null
}
trap cleanup EXIT
cargo build -r
OPENCV_FFMPEG_LOGLEVEL=0 ./target/release/localcv "$1" "$WIDTH" "$HEIGHT"