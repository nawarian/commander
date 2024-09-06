FROM python:3.10-slim

RUN apt update && apt install -y \
  git \
  && rm -rf /var/lib/apt/lists/*

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Pre-download model and datasets
# model: microsoft/speecht5_tts
# dataset: Matthijs/cmu-arctic-xvectors
RUN python -c "from transformers import AutoModel; AutoModel.from_pretrained('microsoft/speecht5_tts')" && \
  python -c "from datasets import load_dataset; load_dataset('Matthijs/cmu-arctic-xvectors')"

WORKDIR /app
COPY api.py .

EXPOSE 8080

CMD ["python", "api.py"]

