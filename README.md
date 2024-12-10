# localcv 

Real-time Parallel and Distributed Depth Estimation for Vision Systems
# Prerequisites (for building from source)
1. [ffmpeg 7.1+](https://ffmpeg.org/)
2. [OpenCV 4.10+](https://opencv.org/)
3. [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
4. Python 3.8+ 
5. [Xcode CoreML tools](https://apple.github.io/coremltools/docs-guides/)


# Directory setup
1. create models directory on project root. program finds the ml model in `models/depth_anything_v2.mlpackage` path.
2. create data directory on project root. program finds input data in `data/` directory. place videos in this directory.
3. create logs directory on project root for capturing logs from `scripts/sender.py`.

# Build and run
N = Number of threads
1. Running without renderer(generates latency results in `outputs/`)
    ```
    ./run.sh <N>
    ```
2. Running with renderer:
     ```
    ./run.sh <N> -r
    ```

# Extras
The script file includes the following components:
1. main.py: Contains code for converting a PyTorch model to Core ML.
2. coremlinference.py: Implements model inferencing in Python.
3. sender.py: Handles streaming video sending functionality.

[Troubleshooting libclang.dylib path issue on MacOS](https://github.com/twistedfall/opencv-rust/blob/master/TROUBLESHOOTING.md)
