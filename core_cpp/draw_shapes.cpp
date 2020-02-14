//
// Created by lyubo on 13.02.20 г..
//

#include "draw_shapes.h"

Mat draw_circle(Mat image, const Point& center, const int& radius, const Scalar& color, const int& thickness)
{
    circle(image, center, radius, color, thickness);
    return image;
}
Mat draw_rectangle(Mat image, const Point& first, const Point& second, const Scalar& color, const int& thickness)
{
    rectangle(image, first, second, color, thickness);
    return image;
}
Mat draw_line(Mat image, const Point& first, const Point& second, const Scalar& color, const int& thickness)
{
    line(image, first, second, color, thickness);
    return image;
}
