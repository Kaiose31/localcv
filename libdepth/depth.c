#include <stdio.h>

/// TODO: Change this to inference function
// inputs: Row major array of video frame, size n
//
void test_inference(float *data, int size)
{

    for (int i = 0; i <= size * size; i++)
    {
        printf("%d", data[i]);
    }
}