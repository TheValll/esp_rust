#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{DriveMode, DriveStrength, Level, Output, OutputConfig, Pull},
    main,
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let led_pin_conf = OutputConfig::default()
        .with_drive_mode(DriveMode::PushPull)
        .with_drive_strength(DriveStrength::_10mA)
        .with_pull(Pull::None);

    let mut led_pin = Output::new(
        peripherals.GPIO1,
        Level::Low,
        led_pin_conf,
    );

    loop {
        led_pin.set_high();
        println!("LED ON");
        delay.delay_millis(1000);
        led_pin.set_low();
        println!("LED OFF");
        delay.delay_millis(1000);
    }
}
