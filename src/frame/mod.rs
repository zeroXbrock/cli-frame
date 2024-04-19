use crossterm::{
    cursor::{DisableBlinking, MoveTo, SavePosition},
    execute,
};
use term_size;

const FRAME_BG: &str = "▓";
const CLEAR: &str = " ";

pub struct Frame {
    width: usize,
    height: usize,
    lines_buffer: Box<[String]>,
    config: FrameConfig,
}

#[derive(Clone, Debug)]
pub struct FrameConfig {
    pub border_thickness: usize,
    pub padding: usize,
    pub margin: usize,
}

impl Default for FrameConfig {
    fn default() -> Self {
        Self {
            border_thickness: 1,
            padding: 1,
            margin: 1,
        }
    }
}

impl FrameConfig {
    pub fn frame(&self) -> Frame {
        Frame::new(self)
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_border_thickness(mut self, border_thickness: usize) -> Self {
        self.border_thickness = border_thickness;
        self
    }

    pub fn with_padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_margin(mut self, margin: usize) -> Self {
        self.margin = margin;
        self
    }
}

impl Frame {
    pub fn new(config: &FrameConfig) -> Self {
        // set frame to terminal size
        let (width, height) = if let Some((width, height)) = term_size::dimensions() {
            (width, height)
        } else {
            panic!("Unable to get terminal size. Using default: 80 x 24");
        };
        Self {
            width,
            height,
            lines_buffer: Box::new([]),
            config: config.clone(),
        }
    }

    fn content_width(&self) -> usize {
        self.width - self.framespace()
    }

    fn content_height(&self) -> usize {
        self.height - self.framespace()
    }

    fn framespace(&self) -> usize {
        (self.config.border_thickness + self.config.margin + self.config.padding) * 2
    }

    fn left_padding(&self) -> String {
        " ".repeat(self.config.padding)
    }

    fn left_margin(&self) -> String {
        " ".repeat(self.config.margin)
    }

    fn frame_col(&self) -> String {
        FRAME_BG.repeat(self.config.border_thickness)
    }

    fn frame_row(&self) -> String {
        FRAME_BG.repeat(self.frame_width())
    }

    fn frame_width(&self) -> usize {
        self.width - (self.config.margin * 2)
    }

    /// Render the frame.
    fn render(&self) {
        if self.lines_buffer.len() > 0 {
            reset_cursor();

            let top_margin = CLEAR.repeat(self.width);
            // render top margin
            for _ in 0..self.config.margin {
                println!("{}", top_margin);
            }

            // render top border
            let row_frame = format!("{}{}", self.left_margin(), self.frame_row());
            let row_frame = (0..self.config.border_thickness)
                .map(|_| row_frame.clone())
                .collect::<Vec<String>>()
                .join("\n");

            // render top padding
            let inner_padding_row = format!(
                "{}{}{}{}",
                self.left_margin(),
                self.frame_col(),
                CLEAR.repeat(self.content_width() + (self.config.padding * 2)),
                self.frame_col(),
            );

            // render top border w/ padding, margins
            println!("{}", row_frame);
            println!("{}", inner_padding_row);

            // render lines
            for line in self.lines_buffer.iter() {
                let right_padding =
                    CLEAR.repeat(self.content_width() - line.len() + self.config.padding);
                println!(
                    "{}{}{}{}{}{}",
                    self.left_margin(),
                    self.frame_col(),
                    self.left_padding(),
                    line,
                    right_padding,
                    self.frame_col(),
                );
            }

            // render bottom padding & border
            println!("{}", inner_padding_row);
            println!("{}", row_frame);
        }
    }

    /// Clear the frame.
    fn clear(&mut self) {
        // clear terminal entirely
        reset_cursor();

        for _ in 0..self.height {
            println!("{}", CLEAR.repeat(self.width));
        }

        self.lines_buffer = Box::new([]);
    }

    /// Update the frame by calling internal functions `clear` and `render`.
    pub fn update(&mut self, content: &str) {
        self.clear();
        let mut buf = vec![];
        for line in content.lines() {
            buf.extend(wrap_line(line, self.content_width()));
        }
        self.lines_buffer = buf.into_boxed_slice();
        self.render();
    }
}

/// Splits line into multiple lines by taking `width` characters per line.
fn wrap_line(line: &str, width: usize) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    if line.len() > width {
        let (truncated, remainder) = line.split_at(width);
        lines.push(truncated.to_owned());
        lines.extend(wrap_line(&remainder, width));
    } else {
        lines.push(line.to_owned());
    }

    lines
}

/// Reset the cursor to the top left of the terminal.
fn reset_cursor() {
    execute!(
        std::io::stdout(),
        SavePosition,
        DisableBlinking,
        MoveTo(0, 0),
    )
    .unwrap();
}
