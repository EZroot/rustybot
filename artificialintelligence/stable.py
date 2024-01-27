# import asyncio
# from diffusers import StableDiffusionPipeline
# import torch

# model_id = "runwayml/stable-diffusion-v1-5"
# pipe = StableDiffusionPipeline.from_pretrained(model_id, torch_dtype=torch.float32)
# pipe = pipe.to("cuda")
# def dummy_checker(images, **kwargs): return images, False
# pipe.safety_checker = dummy_checker

# async def generate_image(prompt, filename):
#     result = pipe(prompt, height=512, width=512, num_inference_steps=200, guidance_scale=7.5).images[0]
#     result.save(filename)

# async def main():
#     prompts = ["a sea trout playing a violin under water"]
#     for i in range(1):
#         filename = f"file{i}.png"
#         await generate_image(prompts[i], filename)

# if __name__ == '__main__':
#     asyncio.run(main())

import os
import asyncio
from diffusers import StableDiffusionPipeline
import torch
from quart import Quart, jsonify, request

from PIL import Image

semaphore = asyncio.Semaphore(1)  # Set the maximum number of concurrent requests to 1

def image_grid(imgs, rows, cols):
    assert len(imgs) == rows * cols

    w, h = imgs[0].size
    grid = Image.new('RGB', size=(cols * w, rows * h))
    grid_w, grid_h = grid.size

    for i, img in enumerate(imgs):
        grid.paste(img, box=(i % cols * w, i // cols * h))
    return grid


model_id = "runwayml/stable-diffusion-v1-5"
pipe = StableDiffusionPipeline.from_pretrained(model_id, torch_dtype=torch.float32)
pipe = pipe.to("cuda")

def dummy_checker(images, **kwargs):
    return images, False

pipe.safety_checker = dummy_checker

app = Quart(__name__)

async def generate_image_async(prompt, height, width, num_inference_steps, guidance_scale):
    result = await asyncio.to_thread(
        pipe,
        prompt=prompt,
        height=height,
        width=width,
        num_inference_steps=num_inference_steps,
        guidance_scale=guidance_scale
    )
    return result.images[0]

@app.route('/generateimg')
async def generate_image():
    async with semaphore:
        prompt = request.args.get('prompt', default='a photo of a banana on a pyramid', type=str)
        height = int(request.args.get('height', default=512, type=int))
        width = int(request.args.get('width', default=512, type=int))
        num_inference_steps = int(request.args.get('num_inference_steps', default=200, type=int))
        guidance_scale = float(request.args.get('guidance_scale', default=7.5, type=float))
        img_count = int(request.args.get('img_count', default=1, type=int))
        use_columns = bool(request.args.get('use_columns', default=True, type=bool))

        num_images = img_count
        results = []

        for i in range(num_images):
            image = await generate_image_async(prompt, height, width, num_inference_steps, guidance_scale)
            results.append(image)
            torch.cuda.empty_cache()  # Clear CUDA cache to release GPU memory

        num_columns = int(num_images ** 0.5)  # Calculate the number of columns for the grid
        num_rows = (num_images + num_columns - 1) // num_columns  # Calculate the number of rows for the grid

        grid = image_grid(results, rows=num_rows, cols=num_columns) if use_columns else image_grid(results, rows=num_images, cols=1)
        filename = f"./gen_pics/{prompt.replace(' ', '_').lower()}.png"
        await asyncio.to_thread(grid.save, filename)
        return jsonify({'image_path': os.path.abspath(filename)})

if __name__ == '__main__':
    app.run(debug=False, port=6969)
