#![no_std]
#![no_main]

use bsp::{entry, hal, hal::fugit::RateExtU32, hal::Clock, pac};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rp_pico as bsp;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let mut timer = hal::timer::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let sio = hal::Sio::new(pac.SIO);
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Onboard LED
    let mut led_pin = pins.led.into_push_pull_output();

    // UART: GPIO0 and GPIO1
    let uart_pins = (pins.gpio0.into_function(), pins.gpio1.into_function());
    let uart_config = hal::uart::UartConfig::new(
        115200.Hz(),
        hal::uart::DataBits::Eight,
        None,
        hal::uart::StopBits::One,
    );
    let uart = hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(uart_config, clocks.peripheral_clock.freq())
        .unwrap();

    loop {
        uart.write_full_blocking(b"Switching LED on\r\n");
        led_pin.set_high().unwrap();
        timer.delay_ms(500);
        uart.write_full_blocking(b"Switching LED off\r\n");
        led_pin.set_low().unwrap();
        timer.delay_ms(500);
    }
}
