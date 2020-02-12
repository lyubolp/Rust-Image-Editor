//
// Created by lyubo on 26.01.20 г..
//

#ifndef RUST_IMAGE_EDITOR_IO_H
#define RUST_IMAGE_EDITOR_IO_H

#include <iostream>
#include "opencv2/core/core.hpp"
#include "opencv2/imgproc/imgproc.hpp"
#include "opencv2/highgui/highgui.hpp"
#include "opencv2/core/mat.hpp"
using namespace cv;


int io_save(Mat image, const char* path);

Mat io_open(const char* path);


#endif //RUST_IMAGE_EDITOR_IO_H
