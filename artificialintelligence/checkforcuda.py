import tensorflow as tf
import torch
# pretty sure this eats the fuck out of the memory -> print(torch.cuda.max_memory_split_size())

print("TF INFO ->", tf.sysconfig.get_lib())
print("TF Version:",tf.__version__)
print("Num GPUs Available: ", tf.config.list_physical_devices('GPU'))
print("Is TensorFlow using CUDA? ", tf.test.is_built_with_cuda())
