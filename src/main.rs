#![no_std]
#![no_main]

use panic_halt as _;
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    let mut counter = 0;

    loop {
        // Heart beat on the LED
        led.set_high();
        arduino_hal::delay_ms(500);
        led.set_low();
        arduino_hal::delay_ms(500);
        // Print the current state of the LED
        counter+=1;
        ufmt::uwrite!(&mut serial, "LED: {}\r\n",counter ).ok();
    }

       
}
