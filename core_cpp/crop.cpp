//
// Created by lyubo on 14.02.20 г..
//

#include "crop.h"
#include <iostream>
Mat crop_mat(Mat image,
             int row_start, int col_start, int new_rows, int new_cols) {

    Rect crop_region;
    crop_region.x = col_start;
    crop_region.y = row_start;

    crop_region.width = new_cols;
    crop_region.height = new_rows;

    return image(crop_region);
}
