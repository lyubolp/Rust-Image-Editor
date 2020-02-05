#include <iostream>
#include "library.h"

int **convert_mat_to_memory(Mat image) {
    int **matrix = new int *[image.rows];

    for (size_t row = 0; row < image.rows; row++) {
        matrix[row] = new int[image.cols];
    }

    for (size_t row = 0; row < image.rows; row++) {
        for (size_t col = 0; col < image.cols; col++) {
            matrix[row][col] = image.at<Vec3b>(row, col)[0];
        }
    }
    return matrix;
}


extern "C" {
int **open_file(char *path) {
    IO io;
    Mat result = io.open(path);
    if (!result.data) {
        return nullptr;
    }

    return convert_mat_to_memory(result);
}

int get_rows(char *path) {
    IO io;
    Mat result = io.open(path);
    if (!result.data) {
        return -1;
    }
    return result.rows;
}

int get_cols(char *path) {
    IO io;
    Mat result = io.open(path);
    if (!result.data) {
        return -1;
    }
    return result.cols;
}

void free_memory(int** memory, int rows)
{
    for(int i = 0; i < rows; i++)
    {
        delete[] memory[i];
    }
    delete[] memory;
}
}