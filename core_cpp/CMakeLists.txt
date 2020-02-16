cmake_minimum_required(VERSION 3.0)
project(core_cpp CXX)


find_package( OpenCV REQUIRED )
include_directories( ${OpenCV_INCLUDE_DIRS} )

set(CMAKE_CXX_STANDARD 14)

add_library(core_cpp STATIC library.cpp library.h IO.cpp IO.h
        Contrast_Brightness.cpp Contrast_Brightness.h draw_shapes.cpp crop.cpp crop.h
        laplace.cpp laplace.h blur.h)
target_link_libraries( core_cpp ${OpenCV_LIBS} )


install(TARGETS core_cpp DESTINATION .)