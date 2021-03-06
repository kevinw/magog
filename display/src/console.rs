use calx_alg::split_line;
use euclid::{Point2D, Rect};
use std::io;
use std::io::prelude::*;
use std::mem;
use std::str;
use time;
use vitral::{Align, Context};

struct Message {
    expire_time_s: f64,
    text: String,
}

impl Message {
    fn new(text: String, time_start_s: f64) -> Message {
        const TIME_TO_READ_CHAR_S: f64 = 0.2;
        let expire_time_s = time_start_s + text.len() as f64 * TIME_TO_READ_CHAR_S;
        Message {
            expire_time_s: expire_time_s,
            text: text,
        }
    }
}

/// Output text container.
#[derive(Default)]
pub struct Console {
    lines: Vec<Message>,
    input_buffer: String,
    output_buffer: String,
    done_reading_s: f64,
}

impl Console {
    /// Draw the console as a regular message display.
    pub fn draw_small<C: Context>(&mut self, context: &mut C, screen_area: &Rect<f32>) {
        // TODO: Store color in draw context.
        let color = [1.0, 1.0, 1.0, 0.4];

        let t = time::precise_time_s();
        let h = context.current_font().height;
        let mut y = screen_area.max_y() - h;
        // The log can be very long, and we're always most interested in the latest ones, so
        // do a backwards iteration with an early exist once we hit a sufficiently old item.
        for msg in self.lines.iter().rev().take_while(|m| m.expire_time_s > t) {
            // The split_line iterator can't be reversed, need to do a bit of caching here.
            let fragments = split_line(
                &msg.text,
                |c| context.current_font().char_width(c).unwrap_or(0.0),
                screen_area.size.width,
            ).map(|x| x.to_string())
                .collect::<Vec<String>>();
            for line in fragments.iter().rev() {
                context.draw_text(Point2D::new(0.0, y), Align::Left, color, line);
                y -= h;
            }
        }
    }

    /// Draw the console as a big drop-down with a command prompt.
    pub fn draw_large<C: Context>(&mut self, context: &mut C, screen_area: &Rect<f32>) {
        // TODO: Store color in draw context.
        let color = [0.6, 0.6, 0.6, 1.0];
        let background = [0.0, 0.0, 0.6, 0.8];

        context.fill_rect(*screen_area, background);

        let h = context.current_font().height;
        let mut lines_left = (screen_area.size.height / h).ceil() as i32;
        let mut y = screen_area.max_y() - h;

        // TODO: Handle enter with text input.
        // TODO: Command history.
        context
            .bound(0, y as u32, screen_area.size.width as u32, h as u32)
            .text_input(color, &mut self.input_buffer);
        y -= h;
        lines_left -= 1;

        for msg in self.lines.iter().rev() {
            // XXX: Duplicated from draw_small.
            let fragments = split_line(
                &msg.text,
                |c| context.current_font().char_width(c).unwrap_or(0.0),
                screen_area.size.width,
            ).map(|x| x.to_string())
                .collect::<Vec<String>>();
            for line in fragments.iter().rev() {
                context.draw_text(Point2D::new(0.0, y), Align::Left, color, line);
                y -= h;
                lines_left -= 1;
            }

            if lines_left <= 0 {
                break;
            }
        }
    }

    fn end_message(&mut self) {
        let mut message_text = String::new();
        mem::swap(&mut message_text, &mut self.output_buffer);

        let now = time::precise_time_s();
        if now > self.done_reading_s {
            self.done_reading_s = now;
        }

        let message = Message::new(message_text, self.done_reading_s);
        assert!(message.expire_time_s >= self.done_reading_s);
        self.done_reading_s = message.expire_time_s;
        self.lines.push(message);
    }

    pub fn get_input(&mut self) -> String {
        let mut ret = String::new();
        mem::swap(&mut ret, &mut self.input_buffer);
        ret
    }
}

impl Write for Console {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = str::from_utf8(buf).expect("Wrote non-UTF-8 to Console");
        let mut lines = s.split('\n');
        lines.next().map(|text| self.output_buffer.push_str(text));

        for line in lines {
            self.end_message();
            self.output_buffer.push_str(line);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}
