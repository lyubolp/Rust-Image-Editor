#include <iostream>
#include "library.h"

int **convert_mat_to_memory(Mat image) {
    int **matrix = new int *[image.rows];

    for (size_t row = 0; row < image.rows; row++) {
        matrix[row] = new int[image.cols];
    }

    for (size_t row = 0; row < image.rows; row++) {
        for (size_t col = 0; col < image.cols; col++) {
            matrix[row][col] = image.at<Vec3b>(row, col)[0] * pow(256, 2)
                    + image.at<Vec3b>(row, col)[1] * pow(256, 1)
                    + image.at<Vec3b>(row, col)[2];
        }
    }
    return matrix;
}

Mat convert_memory_to_image(int** memory, int rows, int cols)
{
    Mat image(rows, cols, CV_8UC3);

    for (size_t row = 0; row < image.rows; row++) {
        for (size_t col = 0; col < image.cols; col++) {
            image.at<Vec3b>(row, col)[0] = memory[row][col] / 256 / 256;
            image.at<Vec3b>(row, col)[1] = memory[row][col] / 256 % 256;
            image.at<Vec3b>(row, col)[2] = memory[row][col] % 256;
        }
    }
    return image;
}


extern "C" {
int **open_file(const char *path) {
    Mat result = io_open(path);
    if (!result.data) {
        return nullptr;
    }

    return convert_mat_to_memory(result);
}

int save_file(int** matrix, const char* path, int rows, int cols)
{
    Mat image = convert_memory_to_image(matrix, rows, cols);
    return io_save(image, path);
}

int get_rows(const char *path) {
    Mat result = io_open(path);
    if (!result.data) {
        return -1;
    }
    return result.rows;
}

int get_cols(const char *path) {
    Mat result = io_open(path);
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