#!/bin/bash

VIDEO_FILES=("data/video1.mp4" "data/video2.mp4")

SRT_ADDRESSES=("udp://127.0.0.1:5000" "udp://127.0.0.1:5001")

RESOLUTION="640x480"

if [ "${#VIDEO_FILES[@]}" -ne "${#SRT_ADDRESSES[@]}" ]; then
  echo "Error: The number of video files and SRT addresses must match!"
  exit 1
fi

stream_video() {
  local video_file=$1
  local srt_address=$2

  if [ ! -f "$video_file" ]; then
    echo "Error: Video file '$video_file' not found!"
    return
  fi

  echo "Streaming video file '$video_file' to '$srt_address'"

  while true; do
    ffmpeg -re -i "$video_file" -vf scale="$RESOLUTION",format=gray -pix_fmt gray \
      -c:v libx264 -preset veryfast -g 50 -tune zerolatency -an -f h264 "$srt_address"
    sleep 1
  done
}

# Stream all videos in parallel
for i in "${!VIDEO_FILES[@]}"; do
  stream_video "${VIDEO_FILES[$i]}" "${SRT_ADDRESSES[$i]}" &
done

wait
