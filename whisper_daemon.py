import whisper

def main():
    model = whisper.load_model("small")
    result = model.transcribe("test2.wav")
    print(result["text"])

if __name__ == "__main__":
    main()
