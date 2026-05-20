#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{
    DriveMode, Input, InputConfig, Level, Output, OutputConfig, Pull
};
use esp_hal::main;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let led_config = OutputConfig::default()
        .with_drive_mode(DriveMode::PushPull);
    let buttom_config = InputConfig::default().with_pull(Pull::Up);

    let mut leds = [Output::new(peripherals.GPIO3,Level::High,led_config), Output::new(peripherals.GPIO4,Level::High,led_config), Output::new(peripherals.GPIO5,Level::High,led_config)];

    let buttom = Input::new(
        peripherals.GPIO0,
        buttom_config
    );

    loop {
        if buttom.is_low(){
            for led in &mut leds{   
                led.toggle();
            }
            delay.delay_millis(200);
        }
    }
}