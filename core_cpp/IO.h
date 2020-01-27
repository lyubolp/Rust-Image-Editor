//
// Created by lyubo on 26.01.20 г..
//

#ifndef RUST_IMAGE_EDITOR_IO_H
#define RUST_IMAGE_EDITOR_IO_H


#include "opencv2/core/core.hpp"
#include "opencv2/imgproc/imgproc.hpp"
#include "opencv2/highgui/highgui.hpp"
#include "opencv2/core/mat.hpp"
using namespace cv;


class IO{
private:
    String path;
public:
    IO() = default;
    ~IO() = default;

    bool save();
    bool save_as(char* path);

    Mat open(char* path);

    Mat opened_image;
};


#endif //RUST_IMAGE_EDITOR_IO_H
