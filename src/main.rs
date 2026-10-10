#![no_std]
#![no_main]

use embedded_hal::digital::PinState;
use rp2040_hal::{self as hal, usb::UsbBus};

use chrono::DateTime;

use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

#[used]
#[unsafe(link_section = ".boot2")]
static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

mod clock;
mod display;
mod numbers;
mod scheduler;
mod segment;
mod serial;

use clock::{Clock, to_digits_pacific};
use display::DisplayState;
use scheduler::{Scheduler, Task};

use crate::serial::parse_command;

const XOSC_CRYSTAL_FREQ: u32 = 12_000_000; // Typically found in BSP crates

#[rp2040_hal::entry]
fn main() -> ! {
    // setup garbage
    let mut peripherals = hal::pac::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(peripherals.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let timer = hal::Timer::new(peripherals.TIMER, &mut peripherals.RESETS, &clocks);

    let usb_bus = UsbBusAllocator::new(UsbBus::new(
        peripherals.USBCTRL_REGS,
        peripherals.USBCTRL_DPRAM,
        clocks.usb_clock,
        true, // force VBUS detect (Pico has no VBUS sense pin wired to USB)
        &mut peripherals.RESETS,
    ));

    let mut serial = SerialPort::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("alexng353")
            .product("alarm clock")
            .serial_number("0001")])
        .unwrap()
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    // region: GPIO
    let sio = hal::Sio::new(peripherals.SIO);

    let pins = hal::gpio::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    // ok so i have gpio 2, 3, 4, 5, 6, 7, 8, 9
    // as pins 11, 10, 7, 1, 2, 3, 4, 5
    //
    // Display pin 11 / GPIO 2: TOP
    // Pin 10 / GPIO 3: TL
    // Pin 7 / GPIO 4: TR
    // Pin 1 / GPIO 5: BL
    // Pin 2 / GPIO 6: B
    // Pin 3 / GPIO 7: unconfirmed; reference mapping is DP/colon
    // Pin 4 / GPIO 8: BR
    // Pin 5 / GPIO 9: M

    // Array indices must match SegmentMask bits: T, TR, BR, B, BL, TL, M, DP.
    let segment_pins = [
        pins.gpio2
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
        pins.gpio4
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
        pins.gpio8
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
        pins.gpio6
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
        pins.gpio5
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
        pins.gpio3
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
        pins.gpio9
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
        pins.gpio7
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
    ];

    let digit_pins = [
        pins.gpio10
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
        pins.gpio11
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
        pins.gpio12
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
        pins.gpio13
            .into_push_pull_output_in_state(PinState::High)
            .into_dyn_pin(),
    ];
    // endregion

    let mut display = DisplayState::new(segment_pins, digit_pins);
    let mut scheduler = Scheduler::new(timer.get_counter());
    let mut clock = Clock::new(
        DateTime::from_timestamp_secs(1788945983).unwrap(),
        timer.get_counter(),
    );

    let mut usb_buf = [0u8; 64];
    let mut line_buf = [0u8; 64];
    let mut line_len = 0;
    let mut overflow = false;

    loop {
        // we poll the usb at every given chance, but never block
        if usb_dev.poll(&mut [&mut serial]) {
            match serial.read(&mut usb_buf) {
                Err(_e) => {
                    // do nothing
                }
                Ok(0) => {
                    // do nothing
                }
                Ok(count) => {
                    for b in usb_buf.iter().take(count) {
                        if *b == b'\n' {
                            if overflow {
                                let _ = serial.write(b"err too long\n");
                            } else {
                                let parsed = parse_command(&line_buf, line_len);
                                match parsed {
                                    Ok(action) => match action {
                                        serial::Action::Bootsel => {
                                            hal::rom_data::reset_to_usb_boot(0, 0)
                                        }
                                        serial::Action::Pong => {
                                            let _ = serial.write(b"pong\n");
                                        }
                                        serial::Action::SetTime(n) => {
                                            clock.update_reference(n, timer.get_counter());
                                        }
                                    },
                                    Err(msg) => {
                                        let _ = serial.write(msg.as_bytes());
                                    }
                                }
                            }

                            line_len = 0;
                            overflow = false;
                        } else if overflow {
                            continue;
                        } else if line_len == line_buf.len() {
                            overflow = true
                        } else {
                            line_buf[line_len] = *b;
                            line_len += 1;
                        }
                    }
                }
            }
        }

        if let Some(schedule) = scheduler.next(timer.get_counter()) {
            match schedule.task {
                Task::Refresh => display.refresh(),
                Task::ChangeTime => {
                    let now = clock.get_time(timer.get_counter());
                    let digits = to_digits_pacific(now);
                    display.change_time(digits, now.timestamp_millis() % 1000 < 500);
                }
                Task::GetTime => {
                    // literally do nothing until we implement uart
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
