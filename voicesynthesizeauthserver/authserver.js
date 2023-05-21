const http = require('http');
const { URL } = require('url');
const util = require('util');
const fs = require('fs');
const { TextToSpeechClient } = require('@google-cloud/text-to-speech');

const port = 3000;

const client = new TextToSpeechClient();
const server = http.createServer(async (req, res) => {
  const url = new URL(req.url, `http://${req.headers.host}`);
  const path = url.pathname;

  if (path === '/synthesize') {
    const text = url.searchParams.get('text');
    const outputFile = url.searchParams.get('outputFile');
    const voiceName = url.searchParams.get('voiceName');

    try {
      await synthesizeText(text, outputFile, voiceName);
      res.writeHead(200, { 'Content-Type': 'text/plain' });
      res.end('Audio content written to file: ' + outputFile);
    } catch (err) {
      console.error(err);
      res.writeHead(500, { 'Content-Type': 'text/plain' });
      res.end('Error: ' + err.message);
    }
  } else if (path === '/listVoices') {
    try {
      const voices = await listVoices();
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify(voices));
    } catch (err) {
      console.error(err);
      res.writeHead(500, { 'Content-Type': 'text/plain' });
      res.end('Error: ' + err.message);
    }
  }  else if (path === '/synthesizebytes') {
    const text = url.searchParams.get('text');
    const voiceName = url.searchParams.get('voiceName');

    try {
      const audioContent = await synthesizeTextRespondBytes(text, voiceName);
      res.writeHead(200, {
        'Content-Type': 'audio/mpeg',
        'Content-Length': audioContent.length,
      });
      res.end(audioContent, 'binary');
    } catch (err) {
      console.error(err);
      res.writeHead(500, { 'Content-Type': 'text/plain' });
      res.end('Error: ' + err.message);
    }
  } 
  else {
    res.writeHead(404, { 'Content-Type': 'text/plain' });
    res.end('Not Found');
  }
});


async function synthesizeTextRespondBytes(text, voiceName = 'en-GB-Wavenet-A') {
  const request = {
    input: { text: text },
    voice: { 
      name: voiceName,
      languageCode: 'en-US',
      ssmlGender: 'MALE'
    },
    audioConfig: { audioEncoding: 'MP3' },
  };

  const [response] = await client.synthesizeSpeech(request);
  
  return response.audioContent;
}

async function synthesizeText(text, outputFile, voiceName = 'en-AU-Wavenet-B') {
  const [result] = await client.listVoices({});
  const voices = result.voices;

  const maleUKVoices = voices.filter(v =>
    v.languageCodes.includes('en-GB') &&
    v.ssmlGender === 'MALE'
  );

  //3 is a keeper, 4 is decent, 5 is meh, 6 is good
  //8 is really good
  const voice = maleUKVoices[6];

  console.log("voice {}", voice.name);
  // Construct the synthesis request with the found voice
  const request = {
    input: { text: text },
    voice: {
      name: voice.name,
      ssmlGender: voice.ssmlGender,
      languageCode: voice.languageCodes[0],
    },
    audioConfig: { audioEncoding: 'MP3' },
  };

  const [response] = await client.synthesizeSpeech(request);
  const writeFile = util.promisify(fs.writeFile);
  await writeFile(outputFile, response.audioContent, 'binary');
  console.log(`Audio content written to file: ${outputFile}`);
}


async function listVoices() {

  const [result] = await client.listVoices({});
  const voices = result.voices;

  return voices.map((voice) => {
    return {
      name: voice.name,
      ssmlGender: voice.ssmlGender,
      naturalSampleRateHertz: voice.naturalSampleRateHertz,
      languageCodes: voice.languageCodes,
    };
  });
}

server.listen(port, () => {
  console.log(`Server running at http://localhost:${port}/`);
});
