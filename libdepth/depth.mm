#import <Foundation/Foundation.h>
#import <CoreML/CoreML.h>

extern "C" float* processImage(const char* imagePath, int& dataSize);
extern "C" void processDepthMap(float* depthData, int height, int width);
extern "C" int inference();

int inference(){ 
 @autoreleasepool {
        NSError *error = nil;

        // Path to the image file
        const char* imagePath = "libdepth/interior-design-of-a-house-1571460.jpg";

        // Get the 1D flattened data from the C++ function
        int dataSize;
        float* imageData = processImage(imagePath, dataSize);

        if (!imageData) {
            NSLog(@"Failed to process the image.");
            return -1;
        }

        // Define the shape for 1x3x518x518
        NSArray<NSNumber *> *shape = @[@1, @3, @518, @518];
        NSArray<NSNumber *> *strides = @[@(518 * 518 * 3), @(518 * 518), @(518), @1];

        // Create the MLMultiArray with the flattened image data
        MLMultiArray *multiArray = [[MLMultiArray alloc]
            initWithDataPointer:imageData
                         shape:shape
                      dataType:MLMultiArrayDataTypeFloat32
                       strides:strides
                   deallocator:^(void *bytes) {
                       delete[] (float*)bytes;
                   }
                         error:&error];

        if (error) {
            NSLog(@"Error creating MLMultiArray: %@", error.localizedDescription);
            return -1;
        }

        // Load the MLModel from .mlpackage
        NSURL *modelURL = [NSURL fileURLWithPath:@"models/depth_anything_v2.mlpackage"];
        NSURL *compiledModelURL = [MLModel compileModelAtURL:modelURL error:&error];

        if (error) {
            NSLog(@"Failed to compile the model: %@", error.localizedDescription);
            return -1;
        }

        MLModel *model = [MLModel modelWithContentsOfURL:compiledModelURL error:&error];
        if (error) {
            NSLog(@"Failed to load the compiled model: %@", error.localizedDescription);
            return -1;
        }
        
        // Wrap the MultiArray in an MLFeatureValue
        MLFeatureValue *featureValue = [MLFeatureValue featureValueWithMultiArray:multiArray];
        
        // Prepare the input for the model as an MLFeatureProvider
        NSDictionary *inputDict = @{ @"input": featureValue };
        
        // Use the correct method to initialize the MLDictionaryFeatureProvider
        MLDictionaryFeatureProvider *inputFeatureProvider = [[MLDictionaryFeatureProvider alloc] initWithDictionary:inputDict error:&error];
        
        if (error) {
            NSLog(@"Failed to initialize inputFeatureProvider: %@", error.localizedDescription);
            return -1;
        }
        
        // Run inference
        id<MLFeatureProvider> output = [model predictionFromFeatures:inputFeatureProvider error:&error];
        
        if (error) {
            NSLog(@"Failed to run inference: %@", error.localizedDescription);
            return -1;
        }
        
        // Process the output (you can extract your model output here)
        NSLog(@"Inference completed successfully. Output: %@", output);
        
        // Extract the output feature value
        MLFeatureValue *outputFeatureValue = [output featureValueForName:@"var_1471"]; // Replace with the actual output key name

        if (!outputFeatureValue || !outputFeatureValue.multiArrayValue) {
            NSLog(@"Failed to extract valid output.");
            return -1;
        }

        // Extract the MLMultiArray
        MLMultiArray *outputArray = outputFeatureValue.multiArrayValue;

        // Get the size of the output array
        int outputSize = (int)outputArray.count;

        // Convert MLMultiArray to a 1D float pointer array
        float *outputData = new float[outputSize];
        for (int i = 0; i < outputSize; i++) {
            outputData[i] = [outputArray objectAtIndexedSubscript:i].floatValue;
        }

        // Print the first 10 values of the output array
        for (int i = 0; i < 10 && i < outputSize; i++) {
            NSLog(@"Output value at index %d: %f", i, outputData[i]);
        }
        processDepthMap(outputData, 518, 518);
        // Free the memory allocated for the output data
        delete[] outputData;

    }
    return 0;
}