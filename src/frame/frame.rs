use crate::frame::constants::{FRAME_BG, SPACE};

/// Implement the FrameRender trait to render frames for a custom writer.
pub trait FrameRender {
    /// Initialize struct implementing FrameRender with default values if needed.
    fn new() -> Self;
    /// Create a new frame engine with the given configuration.
    fn new_frame_engine(&self, config: &FrameConfig) -> FrameEngine<impl FrameRender>;
    /// Render a single line of text.
    fn render_line(&self, line: &str);
    /// Reset the cursor to the top left of the interface.
    fn reset_cursor(&self);
    /// Clear the interface.
    fn clear(&self);
}

pub struct FrameEngine<R: FrameRender> {
    lines_buffer: Box<[String]>,
    config: FrameConfig,
    render_engine: R,
}

#[derive(Clone, Debug)]
pub struct FrameConfig {
    pub border_thickness: usize,
    pub padding: usize,
    pub margin: usize,
    pub width: usize,
    pub height: usize,
}

impl Default for FrameConfig {
    fn default() -> Self {
        Self {
            border_thickness: 1,
            padding: 1,
            margin: 1,
            width: 80,
            height: 24,
        }
    }
}

impl FrameConfig {
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

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }
}

impl<R: FrameRender> FrameEngine<R> {
    pub fn new(config: &FrameConfig, render_engine: R) -> Self {
        Self {
            lines_buffer: Box::new([]),
            config: config.clone(),
            render_engine: render_engine,
        }
    }

    fn content_width(&self) -> usize {
        self.config.width - self.framespace()
    }

    // fn content_height(&self) -> usize {
    //     self.height - self.framespace()
    // }

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
        self.config.width - (self.config.margin * 2)
    }

    fn clear(&mut self) {
        self.render_engine.clear();
        self.lines_buffer = Box::new([]);
    }

    /// Render the frame.
    fn render(&self) {
        if self.lines_buffer.len() > 0 {
            self.render_engine.reset_cursor();

            let top_margin = SPACE.repeat(self.config.width);
            // render top margin
            for _ in 0..self.config.margin {
                self.render_engine.render_line(&top_margin);
            }

            // render top border
            let row_frame = format!("{}{}", self.left_margin(), self.frame_row());
            let row_frame = (0..self.config.border_thickness)
                .map(|_| row_frame.clone())
                .collect::<Vec<String>>()
                .join("\n");
            self.render_engine.render_line(&row_frame);

            // render top padding
            let inner_padding_row = format!(
                "{}{}{}{}",
                self.left_margin(),
                self.frame_col(),
                SPACE.repeat(self.content_width() + (self.config.padding * 2)),
                self.frame_col(),
            );
            self.render_engine.render_line(&inner_padding_row);

            // render lines
            for line in self.lines_buffer.iter() {
                let right_padding =
                    SPACE.repeat(self.content_width() - line.len() + self.config.padding);
                self.render_engine.render_line(&format!(
                    "{}{}{}{}{}{}",
                    self.left_margin(),
                    self.frame_col(),
                    self.left_padding(),
                    line,
                    right_padding,
                    self.frame_col(),
                ));
            }

            // render bottom padding & border
            self.render_engine.render_line(&inner_padding_row);
            self.render_engine.render_line(&row_frame);
        }
    }

    /// Update the frame; calls `clear`, updates buffer, then calls `render`.
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
        lines.extend(wrap_line(&remainder.trim_start(), width));
    } else {
        lines.push(line.to_owned());
    }

    lines
}
