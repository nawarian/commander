import torch
from transformers import pipeline
from datasets import load_dataset
import soundfile as sf
from time import time
from io import BytesIO

from pydantic import BaseModel
from fastapi import FastAPI, Response
from fastapi.responses import StreamingResponse

app = FastAPI()
voice_id = 7306

now = time()
synth = pipeline("text-to-speech", "microsoft/speecht5_tts")
print(f"microsoft/speecht5_tts took {time() - now} seconds to load")

now = time()
embeddings_ds = load_dataset("Matthijs/cmu-arctic-xvectors", split="validation")
print(f"Matthijs/cmu-arctic-xvectors took {time() - now} seconds to load")

speaker_embedding = torch.tensor(embeddings_ds[voice_id]["xvector"]).unsqueeze(0)

class SynthInput(BaseModel):
    text: str
    voice_id: int

@app.post("/synth")
async def synth_text(input: SynthInput):
    global speaker_embedding
    global voice_id
    if input.voice_id != voice_id:
        speaker_embedding = torch.tensor(embeddings_ds[input.voice_id]["xvector"]).unsqueeze(0)
        voice_id = input.voice_id

    now = time()
    with torch.no_grad():
        speech = synth(input.text,
                       forward_params={"speaker_embeddings": speaker_embedding})
        print(f"Took {time() - now} seconds to synthesize {input}")

        audio_buff = BytesIO()
        sf.write(audio_buff, speech["audio"], speech["sampling_rate"], format="wav")
        audio_buff.seek(0)
        return StreamingResponse(content=audio_buff, media_type="audio/wav")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8080)

