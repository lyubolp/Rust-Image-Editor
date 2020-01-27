#include <iostream>
#include "library.h"


extern "C"
{
int **open_file(char *path) {
    IO io;
    Mat result = io.open(path);
    if (!result.data) {
        std::cout << "Could not open the file" << std::endl;
        return nullptr;
    }
    int **matrix = new int *[result.rows];

    for (size_t row = 0; row < result.rows; row++) {
        matrix[row] = new int[result.cols];
    }

    for (size_t row = 0; row < result.rows; row++) {
        for (size_t col = 0; col < result.cols; col++) {
            matrix[row][col] = result.at<Vec3b>(row, col)[0];
        }
    }
    std::cout << result.rows << " " << result.cols << std::endl;
    return matrix;
}

}