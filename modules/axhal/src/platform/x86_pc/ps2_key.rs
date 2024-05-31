//! ps2_key

use spinlock::SpinNoIrq;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use x86_64::instructions::port::Port;
use axlog::*;

#[cfg(feature = "irq")]
use crate::irq;

static PS2: SpinNoIrq<Ps2> = SpinNoIrq::new(Ps2::new());
static KEYBOARD: SpinNoIrq<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
    SpinNoIrq::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));

const IRQ_CODE: usize = 0x21;
const MAX_BUFFER_LEN: usize = 255;

#[derive(Debug)]
pub enum Error {
    CommandRetry,
    NoMoreTries,
    ReadTimeout,
    WriteTimeout,
}

struct Ps2 {
    // data: Port<u8>,
    queue: [u8; MAX_BUFFER_LEN],
    head: usize,
    tail: usize,
}

impl Ps2 {
    const fn new() -> Self {
        Ps2 {
            // data: Port::new(0x60),
            queue: [b'\0'; MAX_BUFFER_LEN],
            head: 0,
            tail: 0,
        }
    }

    // fn read(&mut self) -> u8 {
    //     unsafe { self.data.read() }
    // }

    fn enqueue(&mut self, ch: char) {
        if self.tail >= MAX_BUFFER_LEN {
            self.tail = 0;
        }
        self.queue[self.tail] = ch as u8;
        self.tail += 1;
    }

    fn dequeue(&mut self) -> Option<u8> {
        if self.head >= MAX_BUFFER_LEN {
            self.head = 0;
        }
        if self.tail - self.head > 0 {
            let idx = self.head;
            self.head += 1;
            Some(self.queue[idx])
        } else {
            None
        }
    }
}

pub fn handler() {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => ax_print!("{}", character),
                // DecodedKey::RawKey(key) => ax_print!("{:?}", key),
                _ => (),
            }
        }
    }
}

pub(super) fn init() {
    #[cfg(feature = "irq")]
    {
        if irq::register_handler(IRQ_CODE, handler) == false {
            ax_println!("regist ps2 handler failed");
        }
    }
}