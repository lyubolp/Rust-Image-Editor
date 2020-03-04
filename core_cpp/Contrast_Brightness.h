//
// Created by lyubo on 27.01.20 г..
//

#ifndef CPP_TESTS_CONTRAST_H
#define CPP_TESTS_CONTRAST_H
///contrast e [1.0 - 3.0]
///brightness e [0 - 100]
#include "opencv2/core/core.hpp"
#include "opencv2/imgproc/imgproc.hpp"
#include "opencv2/highgui/highgui.hpp"
#include "opencv2/core/mat.hpp"
using namespace cv;
void change_contrast_brightness(Mat image, double contrast, int brightness);
#endif //CPP_TESTS_CONTRAST_H
