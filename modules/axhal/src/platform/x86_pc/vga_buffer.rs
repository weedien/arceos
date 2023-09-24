//! VGA text mode.

use lazy_init::LazyInit;
use spinlock::SpinNoIrq;

static VGA: SpinNoIrq<VgaTextMode> = SpinNoIrq::new(VgaTextMode::new());

/// The height of the vga text buffer (normally 25 lines).
const VGA_BUFFER_HEIGHT: usize = 25;
/// The width of the vga text buffer (normally 80 columns).
const VGA_BUFFER_WIDTH: usize = 80;
/// The MMIO address of VGA buffer.
const VGA_BASE_ADDR: usize = 0xb8000;

/// The standard color palette in VGA text mode.
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
enum VgaTextColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Purple = 5,
    Brown = 6,
    Gray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightPurple = 13,
    Yellow = 14,
    White = 15,
}

/// A combination of a foreground and a background color.
#[derive(Clone, Copy)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /// Create a new `ColorCode` with the given foreground and background colors.
    const fn new(fg: VgaTextColor, bg: VgaTextColor) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

/// Character for the VGA text buffer, including an ASCII character and a `ColorCode`.
struct VgaTextChar(u8, ColorCode);

/// A structure representing the VGA text buffer.
#[repr(transparent)]
struct VgaTextBuffer {
    chars: [[VgaTextChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

struct VgaTextMode {
    current_x: usize,
    current_y: usize,
    current_color: ColorCode,
    buffer: LazyInit<&'static mut VgaTextBuffer>,
}

impl VgaTextMode {
    const fn new() -> Self {
        Self {
            current_x: 0,
            current_y: 0,
            current_color: ColorCode::new(VgaTextColor::White, VgaTextColor::Black),
            buffer: LazyInit::new(),
        }
    }

    fn scroll_up(&mut self, line: usize) {
        if line > VGA_BUFFER_HEIGHT {
            return;
        }

        let buffer = &mut self.buffer.chars;

        let size = line * VGA_BUFFER_WIDTH * core::mem::size_of::<VgaTextChar>();
        let src = &buffer[line][0] as *const VgaTextChar;
        let dst = &mut buffer[0][0] as *mut VgaTextChar;
        unsafe {
            core::ptr::copy(src, dst, size);
        }
        self.current_y -= line;
    }

    fn putchar(&mut self, ch: u8) {
        match ch {
            b'\r' => {
                self.current_x = 0;
            }
            b'\n' => {
                self.current_y += 1;
            }
            _ => {
                self.buffer.chars[self.current_y][self.current_x] = VgaTextChar(ch, self.current_color);
                self.current_x += 1;
            }
        }

        if self.current_x >= VGA_BUFFER_WIDTH {
            self.current_x = 0;
            self.current_y += 1;
        }
        if self.current_y >= VGA_BUFFER_HEIGHT {
            self.scroll_up(self.current_y - VGA_BUFFER_HEIGHT + 1);
        }
    }
}

pub fn init() {
    let mut vga = VGA.lock();
    unsafe {
        vga.buffer.init_by(& mut *(VGA_BASE_ADDR as *mut VgaTextBuffer));
    }
    for y in 0..VGA_BUFFER_HEIGHT {
        for x in 0..VGA_BUFFER_WIDTH {
            vga.buffer.chars[y][x] = VgaTextChar(b' ', vga.current_color);
        }
    }
}

pub fn putchar(c: u8) {
    let mut vga = VGA.lock();
    match c {
        b'\n' => {
            vga.putchar(b'\r');
            vga.putchar(b'\n');
        }
        c => vga.putchar(c),
    }
}

pub fn getchar() -> Option<u8> {
    None
}
