# rs_OpenCV

A demo project using Rust and OpenCV to display live webcam video and record it based on user input.

## Features

- Displays live video from the default webcam
- Press 'r' to start/stop recording (shows "RECORDING" overlay while recording)
- Press 'q' to quit the application
- Saves recorded video as an mp4 file (filename includes a timestamp)

## How to Run

1. Make sure you have Rust and OpenCV installed.
2. In the project directory, run:

   ```bash
   cargo run
   ```

3. Follow the on-screen instructions:
   - Press `r` to start/stop recording
   - Press `q` to quit

## Dependencies

- [opencv](https://crates.io/crates/opencv) crate

## Notes

- Generated video files (`video_capture_*.mp4`) and build artifacts (`target/`) are excluded from version control via `.gitignore`.