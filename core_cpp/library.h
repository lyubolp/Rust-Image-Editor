#ifndef CORE_CPP_LIBRARY_H
#define CORE_CPP_LIBRARY_H

#include <opencv2/opencv.hpp>
using namespace cv;


class IO{
private:
    String path;
public:
    IO() = default;
    ~IO() = default;
    Mat open(char* path);
    bool save(Mat image);
    bool save_as(String path, Mat image);

};

#endif //CORE_CPP_LIBRARY_H
