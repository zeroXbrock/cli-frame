mod frame;

use frame::FrameConfig;

fn main() {
    // let mut frame = FrameConfig::default().frame();
    let mut frame = FrameConfig::new()
        .with_border_thickness(2)
        .with_padding(2)
        .with_margin(2)
        .frame();
    frame.update("hey!\nthis is a multiline string\nwith 4 lines!\nlet me tell you something I'll tell you waht I've got half a mind to tell you what, you better lisen up boy I tell ya, you're in for it and I'm gonna give it to ya boy howdy I mean it buster I am so got dang serious mister");
    // sleep for a second
    std::thread::sleep(std::time::Duration::from_secs(2));
    frame.update("hey!\nthis is # multiline string\nwith 4 lines!\nlet me tell you something I'll tell you w#ht I've got h#lf # mind to tell you wh#t, you better lisen up boy I tell y#, you're in for it #nd I'm gonn# give it to y# boy howdy I mean it buster I am so got dang serious mister");
}
