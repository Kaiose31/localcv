#!/bin/bash

VIDEO_FILE="data/video1.mp4"
SRT_ADDRESS="srt://127.0.0.1:5000"

if [ ! -f "$VIDEO_FILE" ]; then
  echo "Error: Video file not found!"
  exit 1
fi

echo "Sending video file '$VIDEO_FILE' over SRT to $SRT_ADDRESS"
ffmpeg -re -i "$VIDEO_FILE" -c:v libx264 -preset veryfast -f mpegts "$SRT_ADDRESS"
