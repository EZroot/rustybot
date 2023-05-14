# import threading
# from diffusers import StableDiffusionPipeline

# model_id = "runwayml/stable-diffusion-v1-5"

# def generate_image(prompt, filename, output_size):
#     pipe = StableDiffusionPipeline.from_pretrained(model_id)
#     def dummy_checker(images, **kwargs): return images, False
#     pipe.safety_checker = dummy_checker
#     result = pipe(prompt, width=output_size[0], height=output_size[1])
#     image = result.images[0]
#     image.save(filename+".png")

# def run_in_thread(func, *args):
#     thread = threading.Thread(target=func, args=args)
#     thread.start()
#     return thread

# if __name__ == '__main__':
#     threads = []
#     for i in range(1):
#         filename = f"file{i}"
#         thread = run_in_thread(generate_image, "A man in a bunny suit holding a scythe", filename, (512, 512))
#         threads.append(thread)

#     for thread in threads:
#         thread.join()

from diffusers import StableDiffusionPipeline
import torch

model_id = "runwayml/stable-diffusion-v1-5"
pipe = StableDiffusionPipeline.from_pretrained(model_id, torch_dtype=torch.float16)
pipe = pipe.to("cuda")

prompt = "a photo of an astronaut riding a horse on mars"
image = pipe(prompt).images[0]  

image.save("astronaut_rides_horse.png")
