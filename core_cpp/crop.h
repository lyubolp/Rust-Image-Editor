//
// Created by lyubo on 14.02.20 г..
//

#ifndef CPP_TESTS_CROP_H
#define CPP_TESTS_CROP_H

#include "opencv2/core/core.hpp"
#include "opencv2/imgproc/imgproc.hpp"
#include "opencv2/highgui/highgui.hpp"
#include "opencv2/core/mat.hpp"
using namespace cv;

Mat crop_mat(Mat image,
             int row_start, int col_start, int new_rows, int new_cols);

#endif //CPP_TESTS_CROP_H
