#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::time::RateExtU32 as _;
use esp_println::{print, println};
use log::info;

use apa102_spi;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::SmartLedsWrite;
const SPI_FREQ_KHZ: u32 = 20_000;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    //////////////////////////////////////////////////
    // Set up LED output
    //////////////////////////////////////////////////
    // we don't use any pins for CS and MISO because
    // we're only interested in MOSI and SCLK
    // to control the leds.
    let sclk = peripherals.GPIO32;
    let mosi = peripherals.GPIO33;
    let spi: Spi<_> = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(SPI_FREQ_KHZ.kHz())
            .with_mode(esp_hal::spi::Mode::_0),
    )
    .unwrap();
    let spi = spi.with_sck(sclk);
    let spi = spi.with_mosi(mosi);

    let mut led_strip = apa102_spi::Apa102::new(spi);
    //////////////////////////////////////////////////

    let delay = Delay::new();
    for chip_brightness in (0..32).cycle() {
        // USE THIS TO TEST CHIP-NATIVE OR HSV-BASED BRIGHTNESS
        info!("Chip-native brightness: {:>2}", chip_brightness);
        led_strip.set_brightness(chip_brightness);
        for lib_brightness in (0..=255) {
            print!("\r\tLib brightness: {:>3}", lib_brightness);
            let image = (0u8..)
                .into_iter()
                .step_by(2)
                .map(|hue| {
                    hsv2rgb(Hsv {
                        hue,
                        sat: 255,
                        val: 255,
                    })
                })
                .take(144);
            led_strip
                .write(smart_leds::brightness(image.into_iter(), lib_brightness))
                .unwrap();
            delay.delay_millis(100);
        }
        println!()
    }
    unreachable!();
}
