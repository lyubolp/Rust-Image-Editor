//
// Created by lyubo on 27.01.20 г..
//

#include "Contrast_Brightness.h"

void change_contrast_brightness(Mat image, double contrast = 1.0, int brightness = 0)
{
    for(size_t row = 0; row < image.rows; row++)
    {
        for(size_t column = 0; column < image.cols; column++)
        {
            for(int channel = 0; channel < 3; channel++)
            {
                image.at<Vec3b>(row, column)[channel] =
                        saturate_cast<uchar>(contrast * image.at<Vec3b>(row, column)[channel] + brightness);
            }
        }
    }
}