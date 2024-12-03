import argparse
import torch
import coremltools as ct
from depth_anything_v2.dpt import DepthAnythingV2

def export_to_coreml(model, coreml_path, input_size=518):
    # Create a dummy input matching model input dimensions
    dummy_input = torch.randn(1, 3, input_size, input_size)
    
    # Trace the PyTorch model
    traced_model = torch.jit.trace(model, dummy_input)

    # Convert the traced model to Core ML
    coreml_model = ct.convert(
        traced_model,
        inputs=[ct.TensorType(name="input", shape=dummy_input.shape)]
    )
    
    # Save the Core ML model
    coreml_model.save(coreml_path)
    print(f"Model successfully converted to CoreML: {coreml_path}")

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Depth Anything V2 - CoreML Export')
    parser.add_argument('--encoder', type=str, default='vitb', choices=['vits', 'vitb', 'vitl', 'vitg'])
    parser.add_argument('--coreml-path', type=str, default='depth_anything_v2.mlpackage')
    args = parser.parse_args()

    # Configure the model
    model_configs = {
        'vits': {'encoder': 'vits', 'features': 64, 'out_channels': [48, 96, 192, 384]},
        'vitb': {'encoder': 'vitb', 'features': 128, 'out_channels': [96, 192, 384, 768]},
        'vitl': {'encoder': 'vitl', 'features': 256, 'out_channels': [256, 512, 1024, 1024]},
        'vitg': {'encoder': 'vitg', 'features': 384, 'out_channels': [1536, 1536, 1536, 1536]}
    }

    model = DepthAnythingV2(**model_configs[args.encoder])
    model.load_state_dict(torch.load(f'checkpoints/depth_anything_v2_{args.encoder}.pth'))
    model.eval()  # Set the model to evaluation mode

    # Export the model to Core ML
    export_to_coreml(model, args.coreml_path)
