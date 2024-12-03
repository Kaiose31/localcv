// ModelManager.h
#import <CoreML/CoreML.h>

@interface ModelManager : NSObject

// Declare the model property as readwrite in the header file
@property (nonatomic, strong, readwrite) MLModel *model;

+ (instancetype)sharedInstance;

- (void)preloadModel;
- (void)clearModel;
- (MLModel *)getModel;

@end
