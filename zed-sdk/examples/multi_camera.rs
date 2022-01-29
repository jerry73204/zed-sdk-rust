use anyhow::{bail, ensure, Result};
use chrono::{Local, SecondsFormat};
use dialoguer as dial;
use sdk::CameraBuilder;
use std::{
    fs,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering::*},
        Arc,
    },
    thread,
};
use zed_sdk as sdk;

fn main() -> Result<()> {
    // Get the list of cameras
    let devices = sdk::get_device_list();

    if devices.is_empty() {
        bail!("no camera available");
    }

    // Let user chooses a camera
    let choices: Vec<usize> = {
        let items: Vec<String> = devices
            .iter()
            .map(|dev| {
                format!(
                    "id={}\tsn={}\tmodel={}\tstate={}\t",
                    dev.id, dev.sn, dev.camera_model, dev.camera_state
                )
            })
            .collect();
        dial::MultiSelect::new()
            .with_prompt("Choose camera")
            .items(&items)
            .defaults(&vec![true; items.len()])
            .interact()?
    };
    ensure!(!choices.is_empty(), "Error: No camera is selected!");

    let default_output_dir = make_timestamp();
    let output_dir: String = dial::Input::new()
        .default(default_output_dir)
        .with_prompt("Output directory")
        .interact()?;
    fs::create_dir_all(&output_dir)?;

    let terminate = Arc::new(AtomicBool::new(false));
    let output_dir = Arc::new(output_dir);

    // Wait for user to press Ctrl-C
    {
        let terminate = terminate.clone();
        ctrlc::set_handler(move || {
            terminate.store(true, SeqCst);
            eprintln!("Terminating...");
        })?;
    }

    eprintln!("Start recording...");
    eprintln!("Press Ctrl-C to stop");

    let handles: Vec<_> = choices
        .into_iter()
        .map(|choice| {
            let output_dir = output_dir.clone();
            let camera_id = devices[choice].id;
            let terminate = terminate.clone();

            thread::spawn(move || {
                let output_file = Path::new(&*output_dir).join(format!("{}.svo", camera_id));

                // Open camera and start recording
                let camera = CameraBuilder::new().open_usb(camera_id)?;
                let mut camera = camera.enable_recording(output_file, Default::default())?;

                while !terminate.load(SeqCst) {
                    let _grab = camera.grab(Default::default())?;
                    // let output_file = format!("{}.jpg", make_timestamp());
                    // grab.save_current_image(View::SL_VIEW_LEFT, output_file)?;
                }

                // Stop camera
                let camera = camera.disable_recording();

                camera.close();

                anyhow::Ok(())
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap()?;
    }

    eprintln!("Recording stopped");

    Ok(())
}

fn make_timestamp() -> String {
    Local::now()
        .to_rfc3339_opts(SecondsFormat::Nanos, false)
        .replace([':', '.'], "-")
}
