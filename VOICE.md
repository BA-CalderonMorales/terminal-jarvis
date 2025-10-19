# Voice Recognition Setup

Terminal Jarvis supports **100% offline voice recognition** using OpenAI's Whisper model.

## Quick Start

### 1. Download the Model (One-Time, 75 MB)

**Option A: Using the helper script (recommended)**
```bash
./scripts/utils/download-whisper-model.sh
```

**Option B: Manual download**
```bash
# Create models directory
mkdir -p ./models

# Download OpenAI Whisper Tiny (English)
curl -L -o ./models/ggml-tiny.en.bin \
  https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.en.bin
```

### 2. Build with Voice Support

```bash
cargo build --release --features local-voice
```

### 3. Run Terminal Jarvis

```bash
./target/release/terminal-jarvis
```

Select the voice option from the interactive menu.

---

## Model Information

**Model:** OpenAI Whisper Tiny (English-only)
- **License:** MIT (100% open source)
- **Size:** 75 MB
- **Creators:** OpenAI (verified, public company)
- **Repository:** https://github.com/openai/whisper
- **Stars:** 67,000+
- **Downloads:** Most popular ASR on HuggingFace

**Privacy & Security:**
- [SUCCESS] No API key required
- [SUCCESS] 100% offline processing after download
- [SUCCESS] All voice data stays on your device
- [SUCCESS] No internet connection needed (after initial download)
- [SUCCESS] Zero ongoing costs

---

## Alternative Models

If you need better accuracy, you can download larger models:

**Base Model (142 MB - Better Accuracy)**
```bash
curl -L -o ./models/ggml-base.en.bin \
  https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin
```

**Small Model (466 MB - Best Quality)**
```bash
curl -L -o ./models/ggml-small.en.bin \
  https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.en.bin
```

Terminal Jarvis will automatically use the best model it finds.

---

## Supported Platforms

### Linux
**Requirements:** `alsa-utils` (for audio recording)
```bash
sudo apt-get install alsa-utils
```

### macOS
**Requirements:** `sox` (for audio recording)
```bash
brew install sox
```

### Windows
**Requirements:** FFmpeg (for audio recording)
- Download from: https://ffmpeg.org/download.html
- Add to PATH

---

## Troubleshooting

### "No verified whisper model found"
- Download the model using the instructions above
- Verify the file is in `./models/ggml-tiny.en.bin`
- Check file size: Should be ~75 MB

### "Recording failed"
- Linux: Install `alsa-utils`
- macOS: Install `sox`
- Windows: Install FFmpeg

### Model file too small/large
- Re-download the model (may have been corrupted)
- Verify download URL is correct
- Check available disk space

---

## Technical Details

**How It Works:**
1. Records audio using platform-specific tools (arecord/sox/ffmpeg)
2. Converts audio to 16kHz mono WAV format
3. Processes audio locally using Whisper model
4. Returns transcribed text

**Performance:**
- Tiny model: ~2000 FPS (fastest)
- Base model: ~1500 FPS (balanced)
- Small model: ~1000 FPS (most accurate)

**Memory Usage:**
- Tiny: ~273 MB RAM during inference
- Base: ~350 MB RAM during inference
- Small: ~500 MB RAM during inference

---

## Frequently Asked Questions

**Q: Do I need an OpenAI API key?**
A: No. The Whisper models are open source (MIT licensed) and run completely offline on your device.

**Q: Does this use the OpenAI API?**
A: No. This uses the local Whisper models. There are no API calls, no internet required (after download), and no costs.

**Q: Is my voice data sent to OpenAI?**
A: No. All processing happens locally on your device. Your voice data never leaves your machine.

**Q: Can I use this without internet?**
A: Yes. After the initial model download, everything works 100% offline.

**Q: Is this really free?**
A: Yes. The models are MIT licensed (open source) and there are no ongoing costs.

**Q: Who created these models?**
A: OpenAI (the company behind ChatGPT). The team is public and the models are widely trusted by the developer community.

**Q: Are there alternatives to Whisper?**
A: Yes. Terminal Jarvis also supports Vosk (40 MB model) via the `vosk-voice` feature flag, but Whisper is recommended for better accuracy and community support.

---

## License

The Whisper models are licensed under the MIT License by OpenAI.
See: https://github.com/openai/whisper/blob/main/LICENSE
