use opencv::prelude::*;
use opencv::{Result, core, highgui, imgcodecs, imgproc, videoio};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<()> {
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;

    if !opened {
        panic!("Unable to open default camera!");
    }

    // Get camera properties for video writer
    let frame_width = cam.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let frame_height = cam.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    let fps = cam.get(videoio::CAP_PROP_FPS)?;
    let fps = if fps > 0.0 { fps } else { 30.0 }; // Default to 30 FPS if unable to get camera FPS

    let mut video_writer: Option<videoio::VideoWriter> = None;
    let mut is_recording = false;

    println!("Press 'r' to start/stop recording, 'q' to quit");

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            // Add recording indicator
            if is_recording {
                let text = "RECORDING";
                let font_face = imgproc::FONT_HERSHEY_SIMPLEX;
                let font_scale = 1.0;
                let color = core::Scalar::new(0.0, 0.0, 255.0, 0.0); // Red color
                let thickness = 2;
                let baseline = &mut 0;

                let text_size =
                    imgproc::get_text_size(text, font_face, font_scale, thickness, baseline)?;
                let text_x = frame_width - text_size.width - 10;
                let text_y = text_size.height + 10;

                imgproc::put_text(
                    &mut frame,
                    text,
                    core::Point::new(text_x, text_y),
                    font_face,
                    font_scale,
                    color,
                    thickness,
                    imgproc::LINE_8,
                    false,
                )?;
            }

            // Write frame to video file if recording
            if let Some(ref mut writer) = video_writer {
                writer.write(&frame)?;
            }

            highgui::imshow(window, &frame)?;
        }

        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            match key as u8 as char {
                'r' | 'R' => {
                    if is_recording {
                        // Stop recording
                        println!("Stopping video recording...");
                        video_writer = None;
                        is_recording = false;
                    } else {
                        // Start recording
                        let timestamp = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let filename = format!("video_capture_{}.mp4", timestamp);

                        let fourcc = videoio::VideoWriter::fourcc('m', 'p', '4', 'v')?;
                        let writer = videoio::VideoWriter::new(
                            &filename,
                            fourcc,
                            fps,
                            core::Size::new(frame_width, frame_height),
                            true,
                        )?;

                        if writer.is_opened()? {
                            println!("Started recording to: {}", filename);
                            video_writer = Some(writer);
                            is_recording = true;
                        } else {
                            println!("Failed to open video writer for: {}", filename);
                        }
                    }
                }
                's' | 'S' => {
                    // Save screenshot
                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let filename = format!("screenshot_{}.png", timestamp);
                    let params = core::Vector::<i32>::new();
                    if imgcodecs::imwrite(&filename, &frame, &params).is_ok() {
                        println!("Screenshot saved to: {}", filename);
                    } else {
                        println!("Failed to save screenshot.");
                    }
                }
                'q' | 'Q' => {
                    println!("Quitting...");
                    break;
                }
                _ => {}
            }
        }
    }

    // Clean up video writer if still recording
    if video_writer.is_some() {
        println!("Finalizing video file...");
    }

    Ok(())
}
