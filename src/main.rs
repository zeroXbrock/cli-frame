mod animations;
mod frame;

use animations::Animation;
use frame::{console::ConsoleFrame, FrameConfig, FrameRender};
use resolve_path::PathResolveExt;
use std::borrow::Cow;
use std::fs;

fn main() {
    // read the file contents of main.rs
    let test_file_path = match "src/main.rs".resolve() {
        Cow::Borrowed(p) => p.canonicalize().expect("non canonical path"),
        Cow::Owned(p) => p,
    };
    let contents = fs::read_to_string(test_file_path).expect("Failed to read file");

    // set frame configuration options
    let frame_config = FrameConfig::new()
        .with_border_thickness(2)
        .with_padding(2)
        .with_margin(1);

    // create a console frame, prints the frame to console
    let mut frame = ConsoleFrame::new().new_frame_engine(&frame_config);

    // update the frame with the contents of the file
    frame.update(&contents);
    // sleep for a bit
    std::thread::sleep(std::time::Duration::from_secs(3));

    // generate an animation with ascii art
    let ani_frames = vec![
        Animation::Camera.frames(),
        Animation::Loading.frames(),
        Animation::Globe.frames(),
    ];
    for ani in ani_frames {
        // loop each animation a few times
        for _ in 0..5 {
            for frame_txt in &ani {
                frame.update(*frame_txt);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
