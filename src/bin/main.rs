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
    let image: [RGB8; 5] = [
        RGB8 {
            r: 255,
            g: 255,
            b: 255,
        },
        RGB8 { r: 255, g: 0, b: 0 },
        RGB8 { r: 0, g: 255, b: 0 },
        RGB8 { r: 0, g: 0, b: 255 },
        RGB8 {
            r: 255,
            g: 255,
            b: 255,
        },
    ];
    let delay = Delay::new();
    loop {
        for brightness in (0..32).into_iter().cycle() {
            info!("Hello world!");
            led_strip.write(image.into_iter()).unwrap();
            delay.delay_millis(500);
        }
    }
}
