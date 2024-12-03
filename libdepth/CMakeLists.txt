cmake_minimum_required(VERSION 3.27.9)
project(LibDepth CXX OBJCXX)
add_library(depth STATIC preprocessing.cpp depth.mm)

set_target_properties(depth PROPERTIES
    CXX_STANDARD 17
    CXX_STANDARD_REQUIRED ON
    CXX_EXTENSIONS OFF
    OBJCXX_STANDARD 17
    OBJCXX_STANDARD_REQUIRED ON
)

target_include_directories(depth PRIVATE
    /opt/homebrew/opt/opencv/include/opencv4
)

target_link_libraries(depth PRIVATE
    /opt/homebrew/opt/opencv/lib/libopencv_core.dylib
    /opt/homebrew/opt/opencv/lib/libopencv_imgproc.dylib
    /opt/homebrew/opt/opencv/lib/libopencv_imgcodecs.dylib
    "-framework Foundation"
    "-framework CoreML"
)

install(TARGETS depth DESTINATION .)

