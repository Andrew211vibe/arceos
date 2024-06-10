//! ps2_key

use spinlock::SpinNoIrq;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};
use axlog::*;

#[cfg(feature = "irq")]
use crate::irq;

static PS2DRIVER: SpinNoIrq<Ps2Driver> = SpinNoIrq::new(Ps2Driver::new());
static PS2: SpinNoIrq<Ps2> = SpinNoIrq::new(Ps2::new());
static KEYBOARD: SpinNoIrq<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
    SpinNoIrq::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));

const IRQ_CODE: usize = 0x21;
const MAX_BUFFER_LEN: usize = 255;
const TIMEOUT: usize = 100;

bitflags::bitflags! {
    pub struct StatusFlags: u8 {
        const OUTPUT_FULL = 1;
        const INPUT_FULL = 1 << 1;
        const SYSTEM = 1 << 2;
        const COMMAND = 1 << 3;
        // Chipset specific
        const KEYBOARD_LOCK = 1 << 4;
        // Chipset specific
        const SECOND_OUTPUT_FULL = 1 << 5;
        const TIME_OUT = 1 << 6;
        const PARITY = 1 << 7;
    }
}

#[derive(Debug)]
pub enum Error {
    CommandRetry,
    NoMoreTries,
    ReadTimeout,
    WriteTimeout,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
#[allow(dead_code)]
enum Command {
    ReadConfig = 0x20,
    WriteConfig = 0x60,
    DisableSecond = 0xA7,
    EnableSecond = 0xA8,
    TestSecond = 0xA9,
    TestController = 0xAA,
    TestFirst = 0xAB,
    Diagnostic = 0xAC,
    DisableFirst = 0xAD,
    EnableFirst = 0xAE,
    WriteSecond = 0xD4
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
#[allow(dead_code)]
enum KeyboardCommand {
    EnableReporting = 0xF4,
    SetDefaultsDisable = 0xF5,
    SetDefaults = 0xF6,
    Reset = 0xFF
}

struct Ps2Driver {
    data: Port<u8>,
    status: PortReadOnly<u8>,
    cmd: PortWriteOnly<u8>,
}

impl Ps2Driver {
    const fn new() -> Self {
        Ps2Driver {
            data: Port::new(0x60),
            status: PortReadOnly::new(0x64),
            cmd: PortWriteOnly::new(0x64),
        }
    }

    fn status(&mut self) -> StatusFlags {
        StatusFlags::from_bits_truncate(unsafe { self.status.read() })
    }

    pub fn read_data(&mut self) -> Result<u8, Error> {
        for _ in 0..TIMEOUT {
            if self.status().contains(StatusFlags::OUTPUT_FULL) {
                return Ok(unsafe { self.data.read() });
            }
        }
        Err(Error::ReadTimeout)
    }

    pub fn write_data(&mut self, data: u8) -> Result<(), Error> {
        for _ in 0..TIMEOUT {
            if self.status().contains(StatusFlags::OUTPUT_FULL) {
                unsafe { self.data.write(data); }
                return Ok(());
            }
        }
        Err(Error::WriteTimeout)
    }

    pub fn write_cmd(&mut self, cmd: Command) -> Result<(), Error> {
        for _ in 0..TIMEOUT {
            if self.status().contains(StatusFlags::OUTPUT_FULL) {
                unsafe { self.cmd.write(cmd as u8); }
                return Ok(());
            }
        }
        Err(Error::WriteTimeout)
    }

    pub fn init_keyboard(&mut self) -> Result<(), Error> {
        // Disable devices
        // disable first device
        self.write_cmd(Command::DisableFirst)?;
        // disable second device(should be ignored if is "single channel" device)
        self.write_cmd(Command::DisableSecond)?;

        // Flush the output buffer
        let _ = self.read_data();

        // TODO: Set the controller configuration byte
        // TODO: Perform controller self test

        // Enable devices
        self.write_cmd(Command::EnableFirst)?;
        self.write_cmd(Command::EnableSecond)?;

        // TODO: Reset keyboard
        // TODO: Set keyboard to default

        Ok(())
    }
}

struct Ps2 {
    queue: [u8; MAX_BUFFER_LEN],
    head: usize,
    tail: usize,
}

impl Ps2 {
    const fn new() -> Self {
        Ps2 {
            queue: [b'\0'; MAX_BUFFER_LEN],
            head: 0,
            tail: 0,
        }
    }

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
    let mut driver = PS2DRIVER.lock();

    loop {
        match driver.read_data() {
            Ok(scancode) => {
                if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                    if let Some(key) = keyboard.process_keyevent(key_event) {
                        match key {
                            DecodedKey::Unicode(character) => ax_print!("{}", character),
                            // DecodedKey::RawKey(key) => ax_print!("{:?}", key),
                            _ => (),
                        }
                    }
                }
            },
            Err(_) => break
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