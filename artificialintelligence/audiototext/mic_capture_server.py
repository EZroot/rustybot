import sounddevice as sd
import numpy as np
import wave
import socket
import threading

# Set the server address and port
SERVER_IP = '192.168.0.4'  # Replace with your server IP address
SERVER_PORT = 7679  # Replace with your desired port number

# Set the desired audio parameters
sample_rate = 44100  # Sample rate (Hz)
output_file = "output.wav"  # Output file name

# Create a WAV file to save the audio
wav_file = wave.open(output_file, 'wb')
wav_file.setnchannels(1)  # Mono audio
wav_file.setsampwidth(2)  # 16-bit audio
wav_file.setframerate(sample_rate)

# Create a socket server
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_socket.bind((SERVER_IP, SERVER_PORT))
server_socket.listen(1)
print("Server started. Waiting for client connection...")

# Define the audio callback function
def audio_callback(indata, frames, time, status):
    # Convert audio data to bytes
    audio_bytes = (indata * 32767).astype(np.int16).tobytes()

    # Write the audio data to the WAV file
    wav_file.writeframes(audio_bytes)

    # Send the audio data to the client
    client_socket.sendall(audio_bytes)

def handle_client(client_socket):
    try:
        # Start the audio stream
        stream = sd.InputStream(callback=audio_callback, channels=1, samplerate=sample_rate)
        stream.start()

        # Keep streaming until client disconnects
        while True:
            # Receive data from the client (optional)
            data = client_socket.recv(1024)

            # Do something with the received data if needed

            # If the client disconnects, break the loop
            if len(data) == 0:
                break

    except Exception as e:
        print("Error handling client:", str(e))

    finally:
        # Stop the audio stream
        stream.stop()
        stream.close()

        # Close the client socket
        client_socket.close()

        print("Client disconnected")

# Accept a client connection
client_socket, client_address = server_socket.accept()
print("Client connected:", client_address)

# Create a thread to handle the client connection
client_thread = threading.Thread(target=handle_client, args=(client_socket,))
client_thread.start()

# Wait for the thread to finish
client_thread.join()

# Close the WAV file
wav_file.close()

# Close the server socket
server_socket.close()

print(f"Recording saved to {output_file}")
