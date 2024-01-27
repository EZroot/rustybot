import torch
import librosa
import soundfile as sf
from transformers import Wav2Vec2ForCTC, Wav2Vec2Processor
import subprocess
from quart import Quart, request, jsonify
import requests

MODEL_ID = "jonatasgrosman/wav2vec2-large-xlsr-53-english"

app = Quart(__name__)
processor = Wav2Vec2Processor.from_pretrained(MODEL_ID)
model = Wav2Vec2ForCTC.from_pretrained(MODEL_ID)
print("Transcription model loadeed {}",MODEL_ID)

def transcribe_audio_file(path_to_audio):
    print("Transcribing...")
    # Load and preprocess the converted audio file
    speech_array, sampling_rate = librosa.load(path_to_audio, sr=16_000)
    inputs = processor(speech_array, sampling_rate=16_000, return_tensors="pt", padding=True)

    with torch.no_grad():
        logits = model(inputs.input_values, attention_mask=inputs.attention_mask).logits

    predicted_ids = torch.argmax(logits, dim=-1)
    predicted_sentences = processor.batch_decode(predicted_ids)
    formatted_sentences = " ".join(predicted_sentences)  # Combine sentences into a single string
    return formatted_sentences

def convert_to_mp3(input_file, output_file):
    print("Converting audio to mp3")
    try:
        subprocess.check_call(['ffmpeg','-y', '-i', input_file, '-codec:a', 'libmp3lame', '-qscale:a', '2', output_file])
    except subprocess.CalledProcessError as e:
        print(f"Conversion failed: {e}")

@app.route('/transcribe')
async def transcribe():
    path_to_audio = './gen_commands/saved_command.wav'
    path_to_audio_converted = './gen_commands/command_converted.mp3'
    convert_to_mp3(path_to_audio, path_to_audio_converted)

    predicted_sentences = transcribe_audio_file(path_to_audio_converted)

    response = {'command': predicted_sentences}
    parsed = requests.utils.quote(predicted_sentences)
    url = 'http://192.168.0.5:3030/derp?recieved_command={}'.format(parsed)
    requests.get(url)
    print("Reponse sent: {} ", response)
    return jsonify(response)

if __name__ == '__main__':
    app.run(debug=False,port=4269)

