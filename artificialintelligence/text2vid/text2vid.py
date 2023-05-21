import torch
from diffusers import DiffusionPipeline, DPMSolverMultistepScheduler
from diffusers.utils import export_to_video

pipe = DiffusionPipeline.from_pretrained("damo-vilab/text-to-video-ms-1.7b", torch_dtype=torch.float16, variant="fp16")
pipe.scheduler = DPMSolverMultistepScheduler.from_config(pipe.scheduler.config)
pipe.enable_model_cpu_offload()
pipe.enable_vae_slicing()

def dummy_checker(images, **kwargs):
    return images, False

pipe.safety_checker = dummy_checker

while True:
    user_input = input("Enter something (or 'q' to quit): ")
    if user_input == 'q':
        break
    batch_size = 2
    prompt_list = [user_input] * batch_size
    num_inference_steps = 500

    video_frames = pipe(user_input, num_inference_steps=num_inference_steps, height=400, width=400).frames #[]
    #for i in range(0, len(prompt_list), batch_size):
      #  batch_prompts = prompt_list[i:i + batch_size]
       # batch_frames = pipe(batch_prompts, num_inference_steps=num_inference_steps).frames
        #video_frames.extend(batch_frames)

    file_path = f"{user_input}.mp4"
    video_path = export_to_video(video_frames, output_video_path=file_path)
    torch.cuda.empty_cache()  # Clear CUDA cache to release GPU memory
    print(f"Path: {video_path}")


# while True:
#     user_input = input("Enter something (or 'q' to quit): ")
#     if user_input == 'q':
#         break
#     prompt = "Spiderman is surfing"
#     video_frames = pipe(prompt, num_inference_steps=30).frames
#     file_path = f"{prompt}.mp4"
#     video_path = export_to_video(video_frames, output_video_path=file_path)
#     torch.cuda.empty_cache()  # Clear CUDA cache to release GPU memory
#     print(f"Path: {video_path}")