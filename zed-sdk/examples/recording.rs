use anyhow::{bail, Result};
use chrono::{Local, SecondsFormat};
use dialoguer as dial;
use sdk::CameraBuilder;
use std::sync::{
    atomic::{AtomicBool, Ordering::*},
    Arc,
};
use zed_sdk as sdk;

fn main() -> Result<()> {
    // Get the list of cameras
    let devices = sdk::get_device_list();

    if devices.is_empty() {
        bail!("no camera available");
    }

    // Let user chooses a camera
    let choice = {
        let items: Vec<String> = devices
            .iter()
            .map(|dev| {
                format!(
                    "id={}\tsn={}\tmodel={}\tstate={}\t",
                    dev.id, dev.sn, dev.camera_model, dev.camera_state
                )
            })
            .collect();
        dial::Select::new()
            .with_prompt("Choose camera")
            .items(&items)
            .default(0)
            .interact()?
    };
    let camera_id = devices[choice].id;

    // Ask for output path
    let default_output_file = format!("{}.svo", make_timestamp());
    let output_file: String = dial::Input::new()
        .default(default_output_file)
        .with_prompt("Output file")
        .interact()?;

    // Open camera and start recording
    let camera = CameraBuilder::new().open_usb(camera_id)?;
    let mut camera = camera.enable_recording(output_file, Default::default())?;
    eprintln!("Start recording...");
    eprintln!("Press Ctrl-C to stop");

    // Wait for user to press Ctrl-C
    let terminate = Arc::new(AtomicBool::new(false));
    {
        let terminate = terminate.clone();
        ctrlc::set_handler(move || {
            terminate.store(true, SeqCst);
        })?;
    }

    while !terminate.load(SeqCst) {
        let _grab = camera.grab(Default::default())?;
        // let output_file = format!("{}.jpg", make_timestamp());
        // grab.save_current_image(View::SL_VIEW_LEFT, output_file)?;
    }

    // Stop camera
    eprintln!("Stop recording");
    let camera = camera.disable_recording();

    camera.close();

    Ok(())
}

fn make_timestamp() -> String {
    Local::now()
        .to_rfc3339_opts(SecondsFormat::Nanos, false)
        .replace([':', '.'], "-")
}
