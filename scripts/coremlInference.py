# import argparse
# import os
# import cv2
# import numpy as np
# import matplotlib
# import glob
# import coremltools as ct

# def preprocess_image(image, input_size):
#     # Resize and normalize image for the model
#     image = cv2.resize(image, (input_size, input_size))  # Resize to match input size of the model
    
#     # Convert to float32 and normalize to [0, 1]
#     image = image.astype(np.float32) / 255.0

#     # Convert to (C, H, W) format (Channels, Height, Width)
#     image = np.transpose(image, (2, 0, 1))

#     # Add batch dimension (1, C, H, W)
#     image = np.expand_dims(image, axis=0)

#     return image


# def postprocess_depth_map(depth_map):
#     # Normalize depth to [0, 255]
#     depth_map = (depth_map - depth_map.min()) / (depth_map.max() - depth_map.min()) * 255.0
#     depth_map = depth_map.astype(np.uint8)

#     # Apply color map to convert grayscale depth map to RGB
#     cmap = matplotlib.colormaps.get_cmap('Spectral_r')
#     depth_rgb = cmap(depth_map)[:, :, :3]  # Take only the RGB channels
#     depth_rgb = (depth_rgb * 255).astype(np.uint8)  # Convert to 8-bit RGB

#     return depth_rgb

# def run_inference(coreml_model, img_path, input_size, outdir):
#     # Load image
#     raw_image = cv2.imread(img_path)
    
#     if raw_image is None:
#         print(f"Error reading image: {img_path}")
#         return
    
#     print(f"Preprocessing image: {img_path}")
#     # Preprocess the image
#     image_input = preprocess_image(raw_image, input_size)
    
#     try:
#         # Run inference using the Core ML model
#         prediction = coreml_model.predict({'input': image_input})
#         print(f"Prediction: {prediction}")  # Print full prediction output
#     except Exception as e:
#         print(f"Error during inference: {e}")
#         return
    
#     # Extract depth map from prediction
#     if 'var_1471' in prediction:
#         depth_map = prediction['var_1471']
#         depth_map = np.squeeze(depth_map, axis=0)
#         print(f"Depth map before postprocessing: {depth_map.shape}")
#     else:
#         print(f"Error: Expected output key 'var_1471' not found in prediction.")
#         return
#     print(f"Depth map RGB shape before: {depth_map.shape}") 
#     # Postprocess depth map (convert to RGB)
#     depth_map_rgb = postprocess_depth_map(depth_map)
    
#     # Print the shape of the RGB depth map
#     print(f"Depth map RGB shape: {depth_map_rgb.shape}")  # This will show (Height, Width, 3)
    
#     # Save the RGB depth map image
#     output_path = os.path.join(outdir, os.path.splitext(os.path.basename(img_path))[0] + '.png')
#     print(f"Saving depth map to: {output_path}")
    
#     if depth_map_rgb is not None and depth_map_rgb.size > 0:
#         cv2.imwrite(output_path, depth_map_rgb)
#     else:
#         print("Error: Depth map is empty or invalid.")


# if __name__ == '__main__':
#     parser = argparse.ArgumentParser(description='Depth Anything V2 - Store RGB Output')
#     parser.add_argument('--coreml-path', type=str, required=True, help='Path to the CoreML model (.mlpackage)')
#     parser.add_argument('--img-path', type=str, required=True, help='Path to image or folder of images')
#     parser.add_argument('--outdir', type=str, required=True, help='Directory to save output images')
#     parser.add_argument('--input-size', type=int, default=518, help='Input size for the model (e.g., 518)')
#     args = parser.parse_args()

#     # Load the CoreML model
#     coreml_model = ct.models.MLModel(args.coreml_path)
    
#     print(f"Model input description: {coreml_model.input_description}")
#     print(f"Model output description: {coreml_model.output_description}")
    
#     # Check if input path is a file or directory
#     if os.path.isfile(args.img_path):
#         if args.img_path.endswith('txt'):
#             with open(args.img_path, 'r') as f:
#                 filenames = f.read().splitlines()
#         else:
#             filenames = [args.img_path]
#     else:
#         filenames = glob.glob(os.path.join(args.img_path, '**/*'), recursive=True)
    
#     print(f"Found {len(filenames)} files to process.")
#     for filename in filenames:
#         print(f"Processing: {filename}")
#         run_inference(coreml_model, filename, args.input_size, args.outdir)



import argparse
import os
import cv2
import numpy as np
import matplotlib
import glob
import coremltools as ct

def preprocess_image(image, input_size):
    # Resize and normalize image for the model
    image = cv2.resize(image, (input_size, input_size))  # Resize to match input size of the model
    
    # Convert to float32 and normalize to [0, 1]
    image = image.astype(np.float32) / 255.0

    # Convert to (C, H, W) format (Channels, Height, Width)
    image = np.transpose(image, (2, 0, 1))

    # Add batch dimension (1, C, H, W)
    image = np.expand_dims(image, axis=0)
    # Print the first 10 values of the processed image
    print("First 10 values of the processed image:", image.flatten()[:100])
    return image


def postprocess_depth_map(depth_map):
    # Normalize depth to [0, 255]
    depth_map = (depth_map - depth_map.min()) / (depth_map.max() - depth_map.min()) * 255.0
    depth_map = depth_map.astype(np.uint8)

    # Apply color map to convert grayscale depth map to RGB
    cmap = matplotlib.colormaps.get_cmap('Spectral_r')
    depth_rgb = cmap(depth_map)[:, :, :3]  # Take only the RGB channels
    depth_rgb = (depth_rgb * 255).astype(np.uint8)  # Convert to 8-bit RGB

    return depth_rgb

def run_inference(coreml_model, img_path, input_size, outdir):
    # Load image
    raw_image = cv2.imread(img_path)
    
    if raw_image is None:
        print(f"Error reading image: {img_path}")
        return
    
    print(f"Preprocessing image: {img_path}")
    # Preprocess the image
    image_input = preprocess_image(raw_image, input_size)
    
    try:
        # Run inference using the Core ML model
        prediction = coreml_model.predict({'input': image_input})
        print(f"Prediction: {prediction}")  # Print full prediction output
    except Exception as e:
        print(f"Error during inference: {e}")
        return
    
    # Extract depth map from prediction
    if 'var_1471' in prediction:
        depth_map = prediction['var_1471']
        print("First 10 values of the processed image:", depth_map.flatten()[:100])
        depth_map = np.squeeze(depth_map, axis=0)
        print(f"Depth map before postprocessing: {depth_map.shape}")
    else:
        print(f"Error: Expected output key 'var_1471' not found in prediction.")
        return
    print(f"Depth map RGB shape before: {depth_map.shape}") 
    # Postprocess depth map (convert to RGB)
    depth_map_rgb = postprocess_depth_map(depth_map)
    
    # Print the shape of the RGB depth map
    print(f"Depth map RGB shape: {depth_map_rgb.shape}")  # This will show (Height, Width, 3)
    
    # Save the RGB depth map image
    output_path = os.path.join(outdir, os.path.splitext(os.path.basename(img_path))[0] + '.png')
    print(f"Saving depth map to: {output_path}")
    
    if depth_map_rgb is not None and depth_map_rgb.size > 0:
        cv2.imwrite(output_path, depth_map_rgb)
    else:
        print("Error: Depth map is empty or invalid.")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Depth Anything V2 - Store RGB Output')
    parser.add_argument('--coreml-path', type=str, required=True, help='Path to the CoreML model (.mlpackage)')
    parser.add_argument('--img-path', type=str, required=True, help='Path to image or folder of images')
    parser.add_argument('--outdir', type=str, required=True, help='Directory to save output images')
    parser.add_argument('--input-size', type=int, default=518, help='Input size for the model (e.g., 518)')
    args = parser.parse_args()

    # Load the CoreML model
    coreml_model = ct.models.MLModel(args.coreml_path)
    
    print(f"Model input description: {coreml_model.input_description}")
    print(f"Model output description: {coreml_model.output_description}")
    
    # Check if input path is a file or directory
    if os.path.isfile(args.img_path):
        if args.img_path.endswith('txt'):
            with open(args.img_path, 'r') as f:
                filenames = f.read().splitlines()
        else:
            filenames = [args.img_path]
    else:
        filenames = glob.glob(os.path.join(args.img_path, '**/*'), recursive=True)
    
    print(f"Found {len(filenames)} files to process.")
    for filename in filenames:
        print(f"Processing: {filename}")
        run_inference(coreml_model, filename, args.input_size, args.outdir)





