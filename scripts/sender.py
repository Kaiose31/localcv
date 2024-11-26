import os
from multiprocessing import Process
from typing import List
from pathlib import Path
import sys
import logging
import datetime
import subprocess

HOST = "127.0.0.1"


logger = logging.getLogger()
logging.basicConfig(filename=f"logs/{datetime.datetime.today().strftime('%Y-%m-%d')}.log", level=logging.DEBUG)


def video_paths(dir: Path, devices: int) -> tuple[List[str], List[str]]:
    return ([os.path.join(dir, x) for x in os.listdir(dir)[:devices]], [f"udp://{HOST}:{port}" for port in range(5000, 5000 + devices)])


def stream_video(vid, url, res):
    logger.debug(f"streaming {vid}@{res} to {url}")
    command = [
        "ffmpeg",
        "-re",
        "-i",
        vid,
        "-r",
        "10",
        "-vf",
        f"scale={res},format=gray",
        "-pix_fmt",
        "gray",
        "-c:v",
        "libx264",
        "-preset",
        "veryfast",
        "-g",
        "50",
        "-tune",
        "zerolatency",
        "-an",
        "-f",
        "h264",
        "-v",
        "quiet",
        "-stats",
        url,
    ]

    try:
        res = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=True)
        logger.debug(f"stdout: {res.stdout}")
        logger.debug(f"stderr: {res.stderr}")
    except Exception as e:
        logger.error(f"An unexpected error occurred: {e}")


if __name__ == "__main__":
    devices = sys.argv[1]
    width = sys.argv[2]
    height = sys.argv[3]
    res = width + "x" + height
    vids, urls = video_paths(Path("data"), int(devices))

    procs = []
    for vid, url in zip(vids, urls):
        proc = Process(target=stream_video, args=(vid, url, res))
        procs.append(proc)
        proc.start()

    for p in procs:
        p.join()
