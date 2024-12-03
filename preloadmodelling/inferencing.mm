#import <Foundation/Foundation.h>
#import <CoreML/CoreML.h>
#import "ModelManager.mm"

extern "C" float* processImage(const char* imagePath, int& dataSize);
extern "C" void processDepthMap(float* depthData, int height, int width);

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        NSError *error = nil;
       
        // Access the preloaded model
        ModelManager *modelManager = [ModelManager sharedInstance];
        MLModel *model = modelManager.model;
       
        if (!model) {
            NSLog(@"Model is not preloaded.");
            return -1;
        }

        // Path to the image file
        const char* imagePath = "/Users/adityanayak/Depth-Anything-V2/assets/examples/demo01.jpg";

        // Process the image
        int dataSize;
        float* imageData = processImage(imagePath, dataSize);

        if (!imageData) {
            NSLog(@"Failed to process the image.");
            return -1;
        }

        // Create MLMultiArray for input
        NSArray<NSNumber *> *shape = @[@1, @3, @518, @518];
        NSArray<NSNumber *> *strides = @[@(518 * 518 * 3), @(518 * 518), @(518), @1];

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

        // Prepare input
        MLFeatureValue *featureValue = [MLFeatureValue featureValueWithMultiArray:multiArray];
        NSDictionary *inputDict = @{ @"input": featureValue };
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

        // Extract output
        MLFeatureValue *outputFeatureValue = [output featureValueForName:@"var_1471"];
        if (!outputFeatureValue || !outputFeatureValue.multiArrayValue) {
            NSLog(@"Failed to extract valid output.");
            return -1;
        }

        MLMultiArray *outputArray = outputFeatureValue.multiArrayValue;
        int outputSize = (int)outputArray.count;

        float *outputData = new float[outputSize];
        for (int i = 0; i < outputSize; i++) {
            outputData[i] = [outputArray objectAtIndexedSubscript:i].floatValue;
        }

        // Process and display output
        for (int i = 0; i < 10 && i < outputSize; i++) {
            NSLog(@"Output value at index %d: %f", i, outputData[i]);
        }

        processDepthMap(outputData, 518, 518);
        delete[] outputData;
    }
    return 0;
}
