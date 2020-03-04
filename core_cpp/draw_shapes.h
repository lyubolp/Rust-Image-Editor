//
// Created by lyubo on 13.02.20 г..
//

#ifndef CPP_TESTS_DRAW_SHAPES_H
#define CPP_TESTS_DRAW_SHAPES_H
#include "opencv2/core/core.hpp"
#include "opencv2/imgproc/imgproc.hpp"
#include "opencv2/highgui/highgui.hpp"
#include "opencv2/core/mat.hpp"
using namespace cv;

Mat draw_circle(Mat image, const Point& center, const int& radius, const Scalar& color, const int& thickness);
Mat draw_rectangle(Mat image, const Point& first, const Point& second, const Scalar& color, const int& thickness);
Mat draw_line(Mat image, const Point& first, const Point& second, const Scalar& color, const int& thickness);
#endif //CPP_TESTS_DRAW_SHAPES_H
