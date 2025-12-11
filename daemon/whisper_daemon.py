import whisper
import socket
import os
import atexit

SOCKET_FILE = "/tmp/whisper_daemon.sock"

def delete_socket_file():
    if os.path.exists(SOCKET_FILE):
        os.remove(SOCKET_FILE)

def main():
    delete_socket_file()

    server = socket.socket(socket.AF_UNIX)
    server.bind(SOCKET_FILE)
    print("Socket initialized")

    model = whisper.load_model("small")
    print("Model loaded")

    while True:
        server.listen()
        conn, _ = server.accept()

        audio_file_to_transcribe = conn.recv(1024).decode('utf-8').strip()
        result = model.transcribe(audio_file_to_transcribe)
        print(result["text"])

if __name__ == "__main__":
    atexit.register(delete_socket_file)
    main()

