// ModelManager.mm
#import "ModelManager.h"

@implementation ModelManager

+ (instancetype)sharedInstance {
    static ModelManager *sharedInstance = nil;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        sharedInstance = [[ModelManager alloc] init];
    });
    return sharedInstance;
}

- (instancetype)init {
    self = [super init];
    if (self) {
        [self preloadModel];
    }
    return self;
}

- (void)preloadModel {
    NSError *error = nil;
    
    // Check if the compiled model exists
    NSURL *compiledModelURL = [NSURL fileURLWithPath:@"/Users/adityanayak/Depth-Anything-V2/depth_anything_v2.mlpackage/Data/com.apple.CoreML/model.mlmodelc"];
    
    if (![compiledModelURL checkResourceIsReachableAndReturnError:&error]) {
        // If the compiled model doesn't exist, compile it
        NSURL *modelURL = [NSURL fileURLWithPath:@"/Users/adityanayak/Depth-Anything-V2/depth_anything_v2.mlpackage/Data/com.apple.CoreML/model.mlmodel"];
        compiledModelURL = [MLModel compileModelAtURL:modelURL error:&error];
        
        if (error) {
            NSLog(@"Failed to compile the model: %@", error.localizedDescription);
            return;
        }
    }
    
    // Load the compiled model
    self.model = [MLModel modelWithContentsOfURL:compiledModelURL error:&error];
    
    if (error) {
        NSLog(@"Failed to load the compiled model: %@", error.localizedDescription);
        return;
    }
    
    NSLog(@"Model preloaded successfully.");
}

- (void)clearModel {
    self.model = nil;
    NSLog(@"Model cleared from memory.");
}

- (MLModel *)getModel {
    return self.model;
}

@end
