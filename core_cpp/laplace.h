//
// Created by lyubo on 14.02.20 г..
//

#ifndef CPP_TESTS_LAPLACE_H
#define CPP_TESTS_LAPLACE_H

#include "opencv2/core/core.hpp"
#include "opencv2/imgproc/imgproc.hpp"
#include "opencv2/highgui/highgui.hpp"
#include "opencv2/core/mat.hpp"
using namespace cv;

Mat laplace(Mat image, int kernel_size);
#endif //CPP_TESTS_LAPLACE_H
