cmake_minimum_required(VERSION 3.27.9)
project(LibDepth C)
add_library(depth STATIC depth.c)
install(TARGETS depth DESTINATION .)

