import argparse
import socket
import os
import atexit

import whisper

SOCKET_FILE_PATH = "/tmp/whisper_daemon.sock"

def delete_socket_file():
    if os.path.exists(SOCKET_FILE_PATH):
        os.remove(SOCKET_FILE_PATH)

def parse_cli_args():
    parser = argparse.ArgumentParser(description="Daemon that loads the whisper model and listens for audio file paths to transcribe")
    parser.add_argument("--model-size", type=str, default="small", help="size of whisper_model to use", choices = {"small", "base"})
    args = parser.parse_args()
    return (args.model_size)

def main():
    (model_size) = parse_cli_args()

    delete_socket_file()

    server = socket.socket(socket.AF_UNIX)
    server.bind(SOCKET_FILE_PATH)
    print("Socket initialized")

    final_model_size = f'{model_size}.en'
    model = whisper.load_model(final_model_size)
    print(f"Model whisper {final_model_size} loaded")

    server.listen()

    while True:
        conn, _ = server.accept()

        audio_file_to_transcribe = conn.recv(1024).decode('utf-8').strip()

        try:
            result = model.transcribe(audio_file_to_transcribe)
        except Exception as e:
            print("error transcribing file", e)

        conn.sendall(result["text"].encode("utf-8"))
        conn.close()

if __name__ == "__main__":
    atexit.register(delete_socket_file)
    main()

