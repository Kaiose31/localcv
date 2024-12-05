#import <CoreML/CoreML.h>
#import <Foundation/Foundation.h>

@interface ModelManager : NSObject

@property (nonatomic, strong, readwrite) MLModel *model;

+ (instancetype)sharedInstance;
- (void)preloadModel;
- (void)clearModel;
- (MLModel *)getModel;

@end

@implementation ModelManager

// Singleton pattern to ensure only one instance of the manager
+ (instancetype)sharedInstance {
    static ModelManager *sharedInstance = nil;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        sharedInstance = [[ModelManager alloc] init];
    });
    return sharedInstance;
}

// Initialize the manager and preload the model
- (instancetype)init {
    self = [super init];
    if (self) {
        [self preloadModel];  // Preload the model when the instance is created
    }
    return self;
}

- (void)preloadModel {
    NSError *error = nil;

    // Skip loading if the model is already loaded in memory
    // if (self.model != nil) {
    //     NSLog(@"Model is already loaded into memory.");
    //     return; // Skip loading if the model is already in memory
    // }

    // Check if the compiled model exists
    NSURL *compiledModelURL = [NSURL fileURLWithPath:@"models/depth_anything_v2.mlpackage/Data/com.apple.CoreML/model.mlmodelc"];
    
    if (![compiledModelURL checkResourceIsReachableAndReturnError:&error]) {
        // If the compiled model doesn't exist, compile it (this happens only once)
        NSURL *modelURL = [NSURL fileURLWithPath:@"models/depth_anything_v2.mlpackage/Data/com.apple.CoreML/model.mlmodel"];
        compiledModelURL = [MLModel compileModelAtURL:modelURL error:&error];
        
        if (error) {
            NSLog(@"Failed to compile the model: %@", error.localizedDescription);
            return;
        }
        
        [[NSFileManager defaultManager] copyItemAtURL:compiledModelURL toURL:[NSURL fileURLWithPath:@"models/depth_anything_v2.mlpackage/Data/com.apple.CoreML/model.mlmodelc"] error:&error];
    }

    // Create model configuration to specify compute units
    MLModelConfiguration *config = [[MLModelConfiguration alloc] init];
    config.computeUnits = MLComputeUnitsAll; // Use Neural Engine (NPU), GPU, or CPU as needed

    // Now, load the compiled model with the configuration
    self.model = [MLModel modelWithContentsOfURL:compiledModelURL configuration:config error:&error];
    
    if (error) {
        NSLog(@"Failed to load the compiled model: %@", error.localizedDescription);
        return;
    }
    
}

// Method to clear the model from memory
- (void)clearModel {
    self.model = nil;
    NSLog(@"Model cleared from memory.");
}

// Method to get the loaded model
- (MLModel *)getModel {
    return self.model;
}

@end
