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

Mat convert_memory_to_image(int **memory, int rows, int cols, int type) {
    Mat image(rows, cols, type);

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

int save_file(int **matrix, const char *path, int rows, int cols, int type) {
    Mat image = convert_memory_to_image(matrix, rows, cols, type);
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

int get_type(const char *path) {
    Mat result = io_open(path);
    if (!result.data) {
        return -1;
    }
    return result.type();
}

void free_memory(int **memory, int rows) {
    for (int i = 0; i < rows; i++) {
        delete[] memory[i];
    }
    delete[] memory;
}

int **change_brightness(int **matrix, int rows, int cols, int type, int brightness) {
    Mat temp = convert_memory_to_image(matrix, rows, cols, type);
    change_contrast_brightness(temp, 1.0, brightness);
    return convert_mat_to_memory(temp);
}

int **change_contrast(int **matrix, int rows, int cols, int type, double contrast) {
    Mat temp = convert_memory_to_image(matrix, rows, cols, type);
    change_contrast_brightness(temp, contrast, 0);
    return convert_mat_to_memory(temp);
}

/*
Mat draw_circle(Mat image, const Point& center, const int& radius, const Scalar& color, const int& thickness);
Mat draw_rectangle(Mat image, const Point& first, const Point& second, const Scalar& color, const int& thickness);
Mat draw_line(Mat image, const Point& first, const Point& second, const Scalar& color, const int& thickness);
 */
int **draw_circle(int **image,
                  int rows, int cols, int type,
                  int center_row, int center_col,
                  int radius, int color,
                  int thickness) {
    Mat temp = convert_memory_to_image(image, rows, cols, type);
    Scalar color_cv(color % 256, color / 256 % 256, color / 256 / 256);
    temp = draw_circle(temp, Point(center_row, center_col), radius, color_cv, thickness);

    return convert_mat_to_memory(temp);
}

int **draw_rectangle(int **image,
                     int rows, int cols, int type,
                     int first_row, int first_col,
                     int second_row, int second_col,
                     int color, int thickness) {
    Mat temp = convert_memory_to_image(image, rows, cols, type);
    Scalar color_cv(color % 256, color / 256 % 256, color / 256 / 256);
    temp = draw_rectangle(temp, Point(first_row, first_col), Point(second_row, second_col),
                          color_cv, thickness);

    return convert_mat_to_memory(temp);
}

int **draw_line(int **image,
                int rows, int cols, int type,
                int first_row, int first_col,
                int second_row, int second_col,
                int color, int thickness) {
    Mat temp = convert_memory_to_image(image, rows, cols, type);
    Scalar color_cv(color % 256, color / 256 % 256, color / 256 / 256);
    temp = draw_line(temp, Point(first_row, first_col), Point(second_row, second_col),
                     color_cv, thickness);

    return convert_mat_to_memory(temp);
}

int **crop_c(int **image,
           int original_rows, int original_cols, int type,
           int row_start, int col_start, int new_rows, int new_cols) {
    Mat temp = convert_memory_to_image(image, original_rows, original_cols, type);
    temp = crop_mat(temp, row_start, col_start, new_rows, new_cols);
    return convert_mat_to_memory(temp);
}

int **laplace_c(int **image, int rows, int cols, int type, int kernel_size)
{
    Mat temp = convert_memory_to_image(image, rows, cols, type);
    temp = laplace(temp, kernel_size);
    return convert_mat_to_memory(temp);
}

int **blur_c(int **image, int rows, int cols, int type, int kernel_size){
    std::cout << kernel_size<< std::endl;
    Mat temp = convert_memory_to_image(image, rows, cols, type);
    temp = blur_cpp(temp, kernel_size);
    return convert_mat_to_memory(temp);
}
}