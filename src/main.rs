mod frame;

use frame::FrameConfig;
use resolve_path::PathResolveExt;
use std::borrow::Cow;
use std::{fs, path::Path};

fn main() {
    // read the file contents of frame/mod.rs
    let test_file_path = match "src/main.rs".resolve() {
        Cow::Borrowed(p) => p.canonicalize().expect("non canonical path"),
        Cow::Owned(p) => p,
    };
    let contents = fs::read_to_string(test_file_path).expect("Failed to read file");

    // let mut frame = FrameConfig::default().frame();
    let mut frame = FrameConfig::new()
        .with_border_thickness(2)
        .with_padding(2)
        .with_margin(2)
        .frame();

    frame.update("
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus ac feugiat elit, quis molestie est. Aenean non sem sem. Donec auctor, ligula porttitor porttitor tempus, magna purus semper est, at aliquet dui ante at elit. Etiam maximus erat et nunc molestie, id aliquam ipsum vulputate. Maecenas justo lorem, convallis ac imperdiet sit amet, luctus quis quam. Fusce justo urna, maximus sed enim ut, euismod congue metus. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer commodo iaculis arcu, vitae imperdiet magna porta ut. Integer tristique ipsum ut mauris semper vulputate. Pellentesque eget velit vel purus dictum auctor vel vitae enim.

    Duis mi ex, scelerisque at urna vel, aliquet aliquet lorem. Vivamus at dapibus urna. Aenean vitae tempor est. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris at velit eu nunc venenatis gravida vitae sit amet est. Donec vitae scelerisque elit. Sed massa nisl, fermentum vel feugiat sit amet, scelerisque sed urna. Sed maximus vulputate libero a sagittis. Morbi eget nisl quis massa gravida accumsan vitae in nunc. Suspendisse viverra consectetur tortor eu sollicitudin. Sed convallis lacinia mi finibus dapibus. Donec imperdiet dui feugiat nisi porttitor ornare. Morbi posuere egestas pretium. Praesent id tincidunt augue. Mauris posuere bibendum vestibulum. Suspendisse in tristique tortor, quis volutpat lectus.
    ");
    // sleep for a second
    std::thread::sleep(std::time::Duration::from_secs(4));
    frame.update(&contents);
}
