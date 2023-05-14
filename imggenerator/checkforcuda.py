import tensorflow as tf

print("TF INFO ->", tf.sysconfig.get_lib())
print("TF Version:",tf.__version__)
print("Num GPUs Available: ", tf.config.list_physical_devices('GPU'))
print("Is TensorFlow using CUDA? ", tf.test.is_built_with_cuda())