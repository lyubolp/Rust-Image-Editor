//
// Created by lyubo on 16.02.20 г..
//

#ifndef CPP_TESTS_BLUR
#define CPP_TESTS_BLUR
#include "opencv2/core/core.hpp"
#include "opencv2/imgproc/imgproc.hpp"
#include "opencv2/highgui/highgui.hpp"
#include "opencv2/core/mat.hpp"
using namespace cv;

Mat blur_cpp(Mat image, int kernel_size)
{
    GaussianBlur(image, image, Size(kernel_size, kernel_size), 1, 1, BORDER_DEFAULT );

    return image;
}

#endif //CPP_TESTS_BLUR