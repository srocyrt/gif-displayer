use std::fs;

use gif;

use ascii_displayer::screen::{self, BitMap, Screen};
// use ascii_displayer::screen;

// pub struct MyFrame {
//     delay: u16,
//     top: u16,
//     left: u16,
//     width: u16,
//     height: u16,
//     buffer: &'a borrow::Cow<'a, [u8]>,
//     // palette: Option<Vec<u8>>,
// }

enum GifFrame {
    Raw {
        delay: u16,
        top: u16,
        left: u16,
        width: u16,
        height: u16,
        palette: Option<Vec<u8>>,
        buffer: Box<[u8]>,
    },
    Cached {
        delay: u16,
        str: String,
    },
    // Optimized()
}

pub struct GifDisplayer {
    width: usize,
    height: usize,
    global_palette: Box<[u8]>,
    frames: Box<[GifFrame]>,
    current_frame: usize,
    current_screen: ascii_displayer::screen::Screen,
}

impl GifDisplayer {
    pub fn new(file: fs::File) -> Self {
        let options = gif::DecodeOptions::new();
        let decoder = options.read_info(file).unwrap();
        let global_palette = decoder
            .global_palette()
            .expect("Tofix: Global palette not found.")
            .into();
        // Box::
        let width = decoder.width() as usize;
        let height = decoder.height() as usize;
        let frames = GifDisplayer::parsing_gif(decoder);
        let current_screen = screen::Screen::new(width, height, 0x00_00_00_00);
        GifDisplayer {
            width,
            height,
            global_palette,
            current_frame: 0,
            frames,
            current_screen,
        }
    }

    fn parsing_gif(mut decoder: gif::Decoder<fs::File>) -> Box<[GifFrame]> {
        let mut frames: Vec<GifFrame> = vec![];
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            let gif::Frame {
                delay,
                top,
                left,
                width,
                height,
                ref palette,
                ref buffer,
                ..
            } = *frame;
            frames.push(GifFrame::Raw {
                delay,
                top,
                left,
                width,
                height,
                palette: palette.clone(),
                buffer: Box::from(buffer.as_ref()),
            });
        }
        frames.into_boxed_slice()
    }

    pub fn to_string(&mut self) -> String {
        match self.frames[self.current_frame] {
            GifFrame::Raw {
                delay,
                top,
                left,
                width,
                height,
                ref palette,
                ref buffer,
            } => {
                let width = width as usize;
                let height = height as usize;
                let mut frame = screen::BitMap::new(width, height);
                debug_assert_eq!(buffer.len(), width * height);
                for (idx, color) in buffer.iter().enumerate() {
                    let color = *color as usize;
                    let rbg_bytes = if let Some(ref p) = palette {
                        [0xFF, p[3 * color + 2], p[3 * color + 1], p[3 * color]]
                    } else {
                        [
                            0xFF,
                            self.global_palette[3 * color + 2],
                            self.global_palette[3 * color + 1],
                            self.global_palette[3 * color],
                        ]
                    };
                    *frame.get_mut(screen::Coord::Idx(idx)).unwrap() =
                        u32::from_le_bytes(rbg_bytes);
                }

                let mut x = frame.to_string();
                x.shrink_to_fit();
                self.frames[self.current_frame] = GifFrame::Cached {
                    delay: 5,
                    str: x,
                };
                // std::mem::replace(&mut self.frames[self.current_frame], GifFrame::Cached { delay: 5, str: "123".into() });
                self.to_string()
                // format!("???")
            }
            GifFrame::Cached { delay, ref str } => str.clone(),
        }
    }

    pub fn next_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.frames.len();

        // let MyFrame {
        //     delay,
        //     top,
        //     left,
        //     width,
        //     height,
        //     ..
        // } = self.frames[self.current_frame];
    }

    pub fn size(&self) -> usize {
        self.frames.len()
    }
}
