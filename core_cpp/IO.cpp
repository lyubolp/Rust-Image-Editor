//
// Created by lyubo on 26.01.20 г..
//

#include "IO.h"

Mat io_open(const char* path)
{
    return imread(path);
}

int io_save(Mat image, const char* path) {
    try {
        imwrite(path, image);
        return true;
    }
    catch (std::runtime_error& ex) {
        return false;
    }
}