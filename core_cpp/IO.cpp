//
// Created by lyubo on 26.01.20 г..
//

#include "IO.h"

Mat IO::open(char* path)
{
    this->opened_image = imread(path);
    return this->opened_image;
}

bool IO::save() {
    try {
        imwrite(this->path, this->opened_image);
        return true;
    }
    catch (std::runtime_error& ex) {
        return false;
    }
}

bool IO::save_as(char* path) {
    try {
        imwrite(path, this->opened_image);
        return true;
    }
    catch (std::runtime_error& ex) {
        return false;
    }
}