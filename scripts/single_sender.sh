#!/bin/bash

VIDEO_FILE="data/video1.mp4"
SRT_ADDRESS="udp://127.0.0.1:5000"

RESOLUTION="640x480"


if [ ! -f "$VIDEO_FILE" ]; then
  echo "Error: Video file not found!"
  exit 1
fi

echo "Sending video file '$VIDEO_FILE' over SRT to $SRT_ADDRESS"

while true
do
  ffmpeg -re -i "$VIDEO_FILE" -vf scale="$RESOLUTION",format=gray -pix_fmt gray -c:v libx264 -preset veryfast -g 50 -tune zerolatency -an -f h264 "$SRT_ADDRESS"
  sleep 1 
done