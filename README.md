# localcv 


## Research: 
1. Current stack: can run low powered hardware with simple algorithms, cant run anything above 8Gflops. Requires cloud processing for heavy computation,aggregation,user management, which means connecting to internet and maybe violating privacy. No Iot should need to connect to the internet. Using local AI server, we move all computation to an optimized GPU env while handling storage,aggregation and management on the local network.

2. Ideal solution is open source software that can be run with few steps, provides interface to connect video devices and run any cv task on them. Additionally, provide video archiving, search, browsing etc in the future.
IRL requires reverse engineering some hardware to fetch their video streams.

3. In the project,  we demonstrate the viability and security of this solution over traditional IOT by running a local network communication and inference stack which is massively parallelized. We achieve performance higher than 1Gflop per device on depth estimation and plot latency over number of connected devices.


## Technical details

1. Communication Protocol: SRT protocol communication: Uses UDP as transport layer with low latency and is secure. Simple To implement and scalable on local network. TCP is not needed when sending video stream over local network (transmitting video streams over encrypted local network channel) (current ondevice chip specs: https://www.ambarella.com/wp-content/uploads/Ambarella_CV22S_Product_Brief_14SEP2021.pdf)

2. Scaling: Local Hardware and network can be vertically scaled to decrease latency over number of devices. A network of 500Gb/s bandwidth can support max 100 HD video stream devices concurrently.

1080p stream requires ~ 5 Mb/s\
100 Streams ~ 500 Mb/s\

AI Server Specs (Target): \
FLOPS: ~330 TFLOPS  \
GPU: 4x RTX 4090 \
GPU MEMORY: 48GB @ 21GB/s\
MEMORY: 128GB @ 200 GB/s\
CPU: 32 Core AMD EPYC\
NETWORK: WIFI 6E @ 9.6 GB/s\
COST: ~$4900\

## TODO: 
- [x] single thread SRT server capable of accepting multiple connections. [Baseline implementation](https://github.com/Haivision/srt)
- [x] parallel server cluster
- [x] render multiple video streams.
- [ ] create binding for C/C++ call to inference func.
- [ ] display depth image with original in renderer.
- [ ] compiled C/C++ binary with model and inference on GPU
- [ ] benchmark frame processing time, latency/num of devices 

## Benchmark
Use ffmpeg with script to create video streams that from data that hit the target server.


