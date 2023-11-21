use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1};

use pc_keyboard::layouts::Us104Key;
use x86_64::instructions::port::Port;

use spinlock::SpinNoIrq;

#[cfg(feature = "irq")]
use crate::irq::register_handler;

static KEYBOARD: SpinNoIrq<Keyboard<Us104Key, ScancodeSet1>> = SpinNoIrq::new(Keyboard::new(
    ScancodeSet1::new(),
    Us104Key,
    HandleControl::Ignore,
));
// static KEYBOARD_BUFFER: SpinNoIrq<KeyboardBuffer> = SpinNoIrq::new(KeyboardBuffer::new());

/// 键盘中断号
const KEYBOARD_IRQ_NUM: u8 = 0x21;
/// 键盘输入缓存端口
const KEYBOARD_BUFFER_PORT: u16 = 0x60;
/// buffer大小
// const KEYBOARD_BUFFER_SIZE: usize = 100;

// /// 键盘输入缓存，环形队列
// struct KeyboardBuffer {
//     buffer: [u8; KEYBOARD_BUFFER_SIZE],
//     head: usize,
//     tail: usize,
//     size: usize,
// }

// impl KeyboardBuffer {
//     const fn new() -> Self {
//         Self {
//             buffer: [0; KEYBOARD_BUFFER_SIZE],
//             head: 0,
//             tail: 0,
//             size: 0,
//         }
//     }

//     fn push(&mut self, data: u8) {
//         if self.size < KEYBOARD_BUFFER_SIZE {
//             self.buffer[self.tail] = data;
//             self.tail = (self.tail + 1) % KEYBOARD_BUFFER_SIZE;
//             self.size += 1;
//         }
//     }

//     fn pop(&mut self) -> Option<u8> {
//         if self.size > 0 {
//             let data = self.buffer[self.head];
//             self.head = (self.head + 1) % KEYBOARD_BUFFER_SIZE;
//             self.size -= 1;
//             Some(data)
//         } else {
//             None
//         }
//     }
// }
use crate::console::put2stdin;

/// 键盘中断处理函数
/// 键盘驱动只将键值缓存在buffer中，由应用程序读取并定义具体行为
fn keyboard_irq_handler() {
    let mut port = Port::new(KEYBOARD_BUFFER_PORT);
    let scancode = unsafe { port.read() };

    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(c) => put2stdin(c as u8),
                _ => {}
            }
        }
    }
}

/// 初始化键盘
pub(super) fn init() {
    #[cfg(feature = "irq")]
    {
        register_handler(KEYBOARD_IRQ_NUM as usize, keyboard_irq_handler);
    }
}

// pub fn getchar() -> Option<u8> {
//     KEYBOARD_BUFFER.lock().pop()
// }
