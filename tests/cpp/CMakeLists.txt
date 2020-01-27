cmake_minimum_required(VERSION 3.0)
project(cpp_tests CXX)


find_package( OpenCV REQUIRED )
include_directories( ${OpenCV_INCLUDE_DIRS} )

set(CMAKE_CXX_STANDARD 14)

add_executable(cpp_tests main.cpp)
target_link_libraries( cpp_tests ${OpenCV_LIBS} )
