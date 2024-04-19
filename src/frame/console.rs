use crate::frame::constants::SPACE;
use crate::frame::FrameRender;
use crossterm::cursor::{DisableBlinking, MoveTo, SavePosition};
use crossterm::execute;

use super::{Frame, FrameConfig};

#[derive(Clone)]
pub struct ConsoleFrame {
    width: usize,
    height: usize,
}

impl FrameRender for ConsoleFrame {
    /// Create a new console frame, which is used to create a Frame that renders to the console.
    fn new() -> Self {
        // set frame to terminal size
        let (width, height) = if let Some((width, height)) = term_size::dimensions() {
            (width, height)
        } else {
            panic!("Unable to get terminal size. Using default: 80 x 24");
        };
        Self { width, height }
    }

    /// Create a new frame with the given configuration.
    fn frame(&self, config: &FrameConfig) -> Frame<Self> {
        let config = config
            .clone()
            .with_width(self.width)
            .with_height(self.height);
        Frame::new(&config, self.to_owned())
    }

    /// Render a **single, pre-formatted** line of text to the console.
    fn render_line(&self, line: &str) {
        println!("{}", line)
    }

    /// Clear the console.
    fn clear(&self) {
        self.reset_cursor();
        for _ in 0..self.height {
            self.render_line(&format!("{}", SPACE.repeat(self.width)));
        }
    }

    /// Reset the cursor to the top left of the terminal.
    fn reset_cursor(&self) {
        execute!(
            std::io::stdout(),
            SavePosition,
            DisableBlinking,
            MoveTo(0, 0),
        )
        .expect("Failed to reset cursor");
    }
}
