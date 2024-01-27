import socket
import wave
import signal
import threading
import queue
import math
import requests

# Set the server address and port
SERVER_IP = '192.168.0.5'  # Replace with the server's IP address
SERVER_PORT = 7679  # Replace with the server's port number
SPEAKING_VOLUMN = 2000 # usual talking is 1000-2000, quiet is 200-500 or so
MAX_BUFFER_SIZE = 50000 # how much audio to listen to till we try again
RECORDING_LISTEN_TICK = 5 # how many times it will wait for speaking threshold before saving recording 
output_file = "./gen_commands/saved_command.wav"  # Output file name

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
    max_buffer_size = MAX_BUFFER_SIZE  # Adjust the size as per your requirement
    previous_volumn = 0
    
    timeout_tick = 0

    is_recording_command = False
    while not stopped:
        try:
            # Get the current audio bytes from the queue
            current_audio_bytes = audio_queue.get()

            # Add the current audio bytes to the buffer
            buffer_lock.acquire()
            buffer_audio_bytes += current_audio_bytes
            volume_audio_bytes += current_audio_bytes
            buffer_size += len(current_audio_bytes)
            prev_volumn = previous_volumn
            timeout = timeout_tick
            recording_command = is_recording_command
            buffer_lock.release()


            # Check if the buffer size reaches the maximum limit
            if buffer_size >= max_buffer_size:
                # Calculate the volume of the audio
                volume = calculate_volume(volume_audio_bytes)
                if not recording_command and volume > SPEAKING_VOLUMN and prev_volumn > SPEAKING_VOLUMN:
                    print("Command Started...")
                    buffer_lock.acquire()
                    is_recording_command = True
                    buffer_size = 0
                    #buffer_audio_bytes = b''
                    volume_audio_bytes = b''
                    buffer_lock.release()
                 
                if recording_command:
                    if prev_volumn < SPEAKING_VOLUMN:
                        # Reset the buffer size and audio bytes
                        #print("Listening to command: Timeout [{}]",timeout)
                        buffer_lock.acquire()
                        buffer_size = 0
                        volume_audio_bytes = b''
                        timeout_tick+=1
                        buffer_lock.release()
                        if timeout > RECORDING_LISTEN_TICK:
                            print("Threshold reached, saving audio: len {}", len(buffer_audio_bytes))
                            save_audio(buffer_audio_bytes)
                            buffer_lock.acquire()
                            buffer_audio_bytes = b''
                            timeout_tick=0
                            is_recording_command = False
                            buffer_lock.release()
                    else:
                        #print("Resetting timeout")
                        buffer_lock.acquire()
                        buffer_size = 0
                        volume_audio_bytes = b''
                        timeout_tick=0
                        buffer_lock.release()
    
                if not recording_command:
                    #Reset the buffer size and audio bytes
                    #print("Not recording, resetting buffer")
                    buffer_lock.acquire()
                    buffer_size = 0
                    buffer_audio_bytes = b''
                    volume_audio_bytes = b''
                    buffer_lock.release()

                print(f"Volume curr/prev {volume}/{prev_volumn}")
                buffer_lock.acquire()
                previous_volumn = volume
                buffer_lock.release()
                # #sys.stdout.flush()          

        except Exception as e:
            print("Error buffering audio:", str(e))
            break

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
        wav_file.close()
        print("Saved audio")
        url = 'http://localhost:4269/transcribe'
        print(f"Sending transcription request{url}")
        requests.get(url)

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




