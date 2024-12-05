#import <Foundation/Foundation.h>
#import <CoreML/CoreML.h>
#import "ModelManager.mm"

extern "C" float * inference(float *imageData);

float * inference(float *imageData){
     @autoreleasepool {

        NSError *error = nil;
       
        // Access the preloaded model
        ModelManager *modelManager = [ModelManager sharedInstance];
        [modelManager preloadModel];
        MLModel *model = modelManager.model;
       
        if (!model) {
            NSLog(@"Model is not preloaded.");
        }
     
        if (!imageData) {
            NSLog(@"Failed to process the image.");
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
        }

        // Prepare input
        MLFeatureValue *featureValue = [MLFeatureValue featureValueWithMultiArray:multiArray];
        NSDictionary *inputDict = @{ @"input": featureValue };
        MLDictionaryFeatureProvider *inputFeatureProvider = [[MLDictionaryFeatureProvider alloc] initWithDictionary:inputDict error:&error];

        if (error) {
            NSLog(@"Failed to initialize inputFeatureProvider: %@", error.localizedDescription);
        }
        // Run inference
        id<MLFeatureProvider> output = [model predictionFromFeatures:inputFeatureProvider error:&error];
        
        if (error) {
            NSLog(@"Failed to run inference: %@", error.localizedDescription);
        }

        // Extract output
        MLFeatureValue *outputFeatureValue = [output featureValueForName:@"var_1471"];
        if (!outputFeatureValue || !outputFeatureValue.multiArrayValue) {
            NSLog(@"Failed to extract valid output.");
        }

        MLMultiArray *outputArray = outputFeatureValue.multiArrayValue;
        int outputSize = (int)outputArray.count;

        float *outputData = new float[outputSize];
        for (int i = 0; i < outputSize; i++) {
            outputData[i] = [outputArray objectAtIndexedSubscript:i].floatValue;
        }

        return outputData;
    }
}

