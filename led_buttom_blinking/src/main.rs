#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::{
    DriveMode, Input, InputConfig, Level, Output, OutputConfig, Pull
};
use esp_hal::main;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let led_config = OutputConfig::default()
        .with_drive_mode(DriveMode::PushPull);

    let button_config = InputConfig::default().with_pull(Pull::Up);

    let mut led = Output::new(
        peripherals.GPIO4,
        Level::High,
        led_config
    );

    let buttom = Input::new(
        peripherals.GPIO0,
        button_config
    );

    let mut blinkdelay = 1_000_000_u32;

    led.set_low();

    loop{
        for _i in 1..blinkdelay{
            if buttom.is_low(){
                blinkdelay = blinkdelay - 2_5000_u32;

                if blinkdelay < 2_5000{
                    blinkdelay = 1_000_000_u32;
                }
            }
        }
        led.toggle();
    }
}
