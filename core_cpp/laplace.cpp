//
// Created by lyubo on 14.02.20 г..
//

#include "laplace.h"

Mat laplace(Mat image, int kernel_size){
    GaussianBlur( image, image, Size(3,3), 0, 0, BORDER_DEFAULT );

    int ddepth = CV_16S;

    Mat laplacian_gray;
    Laplacian( image, laplacian_gray, ddepth, kernel_size);
    Mat result;
    convertScaleAbs( laplacian_gray, result );

    return result;
}
