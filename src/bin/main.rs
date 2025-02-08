#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::time::RateExtU32 as _;
use esp_println::println;
use log::info;

use apa102_spi;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::{SmartLedsWrite, RGB8};
const SPI_FREQ_KHZ: u32 = 20_000;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();
    println!("starting init...");

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
    println!("midway");
    let spi = spi.with_sck(sclk);
    println!("after sclk");
    let spi = spi.with_mosi(mosi);
    println!("after mosi");

    let mut led_strip = apa102_spi::Apa102::new(spi);
    //////////////////////////////////////////////////

    println!("leds initialized");
    // write s simple 5 pixel pattern

    let delay = Delay::new();
    let mut loop_counter = 0usize;
    let mut brightness = 0u8;
    led_strip.set_brightness(brightness);
    info!("Setting brightness {}", brightness);
    loop {
        let image = (0u8..)
            .into_iter()
            .step_by(1)
            .skip(loop_counter % 256)
            .map(|hue| {
                hsv2rgb(Hsv {
                    hue,
                    sat: 255,
                    val: 255,
                })
            })
            .take(144);
        led_strip.write(image.into_iter()).unwrap();
        delay.delay_millis(10);
        loop_counter += 1;
        if loop_counter % (256 * 2) == 0 {
            brightness = (brightness + 1) % 32;
            info!("Setting brightness {}", brightness);
            led_strip.set_brightness(brightness);
        }
    }
}
