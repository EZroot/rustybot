import socket
import wave
import signal
import threading
import queue
import math

import torch
import librosa
from transformers import Wav2Vec2ForCTC, Wav2Vec2Processor

#transcriptions
MODEL_ID = "jonatasgrosman/wav2vec2-large-xlsr-53-english"
SAMPLE_RATE = 16_000

# Set the server address and port
SERVER_IP = '192.168.0.4'  # Replace with the server's IP address
SERVER_PORT = 7679  # Replace with the server's port number
SPEAKING_VOLUMN = 700 # usual talking is 1000-2000, quiet is 200-500 or so
output_file = "received_audio.wav"  # Output file name

# Set the audio parameters
sample_rate = 44100  # Sample rate (Hz)
channels = 1  # Mono audio
sample_width = 2  # 16-bit audio

# Create a WAV file to save the received audio
wav_file = wave.open(output_file, 'wb')
wav_file.setnchannels(channels)
wav_file.setsampwidth(sample_width)
wav_file.setframerate(sample_rate)

# Variable to indicate if Ctrl+C is pressed
stopped = False

# Signal handler for Ctrl+C
def signal_handler(sig, frame):
    global stopped
    stopped = True
    print("\nCtrl+C pressed. Stopping audio reception.")

# Register the signal handler
signal.signal(signal.SIGINT, signal_handler)


# Load the Wav2Vec2 model and processor
processor = Wav2Vec2Processor.from_pretrained(MODEL_ID)
model = Wav2Vec2ForCTC.from_pretrained(MODEL_ID)
print("Transcribe AI model {} loaded.", MODEL_ID)

# Create a socket client
client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client_socket.connect((SERVER_IP, SERVER_PORT))
print("Connected to server")

# Queue to store audio bytes
audio_queue = queue.Queue()

# Lock for thread synchronization
buffer_lock = threading.Lock()

# Thread for buffering audio
def buffer_audio():
    buffer_size = 0
    buffer_audio_bytes = b''
    volume_audio_bytes = b''
    max_buffer_size = 1000000  # Adjust the size as per your requirement
    prev_volumn = 0

    while not stopped:
        try:
            # Get the current audio bytes from the queue
            current_audio_bytes = audio_queue.get()

            # Add the current audio bytes to the buffer
            buffer_lock.acquire()
            buffer_audio_bytes += current_audio_bytes
            volume_audio_bytes += current_audio_bytes
            buffer_size += len(current_audio_bytes)
            buffer_lock.release()


            # Check if the buffer size reaches the maximum limit
            if buffer_size >= max_buffer_size:
                # Calculate the volume of the audio
                volume = calculate_volume(volume_audio_bytes)
                if prev_volumn > SPEAKING_VOLUMN:
                    if volume < SPEAKING_VOLUMN: 
                        print(f"Saving audio cuz volumn was low")
                        save_audio(buffer_audio_bytes)
                        # Reset the buffer size and audio bytes
                        buffer_lock.acquire()
                        buffer_size = 0
                        buffer_audio_bytes = b''
                        volume_audio_bytes = b''
                        buffer_lock.release()
                    else:
                        # Reset the buffer size and audio bytes
                        buffer_lock.acquire()
                        buffer_size = 0
                        #buffer_audio_bytes = b''
                        volume_audio_bytes = b''
                        buffer_lock.release()
                else:
                    # Reset the buffer size and audio bytes
                    buffer_lock.acquire()
                    buffer_size = 0
                    #buffer_audio_bytes = b''
                    volume_audio_bytes = b''
                    buffer_lock.release()
                prev_volumn = volume
                print(f"Volume: {volume}")
                print(f"Prev_Volume: {prev_volumn}")

        except Exception as e:
            print("Error buffering audio:", str(e))
            break

    # Save the remaining audio if any
    if buffer_size > 0:
        volume = calculate_volume(volume_audio_bytes)
        save_audio(buffer_audio_bytes)
        print(f"Volume: {volume}")


def perform_transcription(audio_path):
    
    # Load the audio file as an array
    audio_array, _ = librosa.load(audio_path, sr=SAMPLE_RATE)

    # Preprocess the audio input
    inputs = processor(audio_array, sampling_rate=SAMPLE_RATE, return_tensors="pt", padding=True)

    # Perform transcription
    with torch.no_grad():
        logits = model(inputs.input_values, attention_mask=inputs.attention_mask).logits

    predicted_ids = torch.argmax(logits, dim=-1)
    transcriptions = processor.batch_decode(predicted_ids)

    # Return the transcriptions
    return '\n'.join(transcriptions)

# Function to save audio to a file
def save_audio(audio_bytes):
    try:
        # Open a new WAV file with the same name and parameters
        wav_file = wave.open(output_file, 'wb')
        wav_file.setnchannels(channels)
        wav_file.setsampwidth(sample_width)
        wav_file.setframerate(sample_rate)
        # Write the audio bytes to the WAV file
        wav_file.writeframes(audio_bytes)
        print("Saved audio")
        transcriptions = perform_transcription(f"./{output_file}")
        print("Trancription: ",transcriptions)
    except Exception as e:
        print("Error saving audio:", str(e))

# Function to calculate the volume of audio
def calculate_volume(audio_bytes):
    samples = []
    for i in range(0, len(audio_bytes), sample_width):
        sample = int.from_bytes(audio_bytes[i:i+sample_width], byteorder='little', signed=True)
        samples.append(sample)

    rms = math.sqrt(sum([sample ** 2 for sample in samples]) / len(samples))
    return rms

# Start the buffering audio thread
buffer_thread = threading.Thread(target=buffer_audio)
buffer_thread.start()


try:
    # Receive and buffer audio data from the server
    while not stopped:
        try:
            # Receive audio data from the server
            audio_bytes = client_socket.recv(1024)

            # Break the loop if no more data is received
            if len(audio_bytes) == 0:
                break

            # Add the audio bytes to the queue
            audio_queue.put(audio_bytes)

        except Exception as e:
            print("Error receiving audio:", str(e))
            break

finally:
    # Close the client socket
    client_socket.close()

    # Wait for the buffering audio thread to finish
    buffer_thread.join()

    # Close the WAV file
    wav_file.close()

    print(f"Received audio saved to {output_file}")




