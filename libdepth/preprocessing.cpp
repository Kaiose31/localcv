#include <opencv2/opencv.hpp>
#include <iostream>

// C++ function to process an image
extern "C" float *processImage(const char *imagePath, int &dataSize)
{
    try
    {
        // Load the image
        cv::Mat image = cv::imread(imagePath, cv::IMREAD_COLOR);
        if (image.empty())
        {
            std::cerr << "Failed to load image: " << imagePath << std::endl;
            return nullptr;
        }

        // Resize the image to 518x518
        cv::resize(image, image, cv::Size(518, 518));

        // Normalize the image to [0, 1]
        image.convertTo(image, CV_32FC3, 1.0 / 255.0);

        // Convert to CHW format (Channels, Height, Width)
        std::vector<cv::Mat> channels(3);
        cv::split(image, channels);
        cv::Mat chw;
        cv::vconcat(channels, chw);

        // Flatten the CHW image to a 1D array
        dataSize = chw.total();
        float *flattenedData = new float[dataSize];
        std::memcpy(flattenedData, chw.ptr<float>(), dataSize * sizeof(float));

        return flattenedData;
    }
    catch (const std::exception &e)
    {
        std::cerr << "Error processing image: " << e.what() << std::endl;
        return nullptr;
    }
}

extern "C" void processDepthMap(float *depthData, int height, int width)
{
    // Step 1: Convert 1D array to cv::Mat
    cv::Mat depthMap(height, width, CV_32F, depthData);

    // Step 2: Normalize to range [0, 255]
    cv::normalize(depthMap, depthMap, 0, 255, cv::NORM_MINMAX);

    // Step 3: Convert to 8-bit grayscale
    cv::Mat depthMap8U;
    depthMap.convertTo(depthMap8U, CV_8U);

    // Step 4: Apply colormap
    cv::Mat depthMapColor;
    cv::applyColorMap(depthMap8U, depthMapColor, cv::COLORMAP_JET); // You can replace COLORMAP_JET with other colormaps

    // Step 5: Save or display the result
    cv::imwrite("depth_map_color.jpg", depthMapColor);
    // cv::imshow("Depth Map", depthMapColor);
}
