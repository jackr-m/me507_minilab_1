#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*; // if info!() or other debug macros are used
// use core::fmt::Write;
// use heapless::String;
// use embassy_stm32::dma::NoDma;
// use embassy_stm32::usart::{Config, UartTx};

use embassy_executor::Spawner;

// use embassy_stm32::gpio::{Level, Output, Speed};

use embassy_time::Timer;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::hz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;

//noinspection RsUnusedImport
use {defmt_rtt as _, panic_probe as _};



#[embassy_executor::main]
/// Main function, blinks an LED for 200ms on, 300ms off, and prints the current loop number to the console.
async fn main(_spawner: Spawner) {
    // Hardware objects
    let p = embassy_stm32::init(Default::default());
    // let mut usart = UartTx::new(p.USART2, p.PA2, p.DMA1_CH6, Config::default()).unwrap();
    // let mut led = Output::new(p.PA5, Level::High, Speed::Low); // Define LED pin

    let ch1 = PwmPin::new_ch1(p.PA5, OutputType::PushPull);
    let mut pwm = SimplePwm::new(p.TIM2, Some(ch1), None, None, None, hz(2), Default::default());

    // Variables
    let mut loop_counter: u16 = 0; // Create loop counter
    // const MAX_STRING_LENGTH: usize = "LED has blinked  times\n".len() + 5; // Define max string length
    // let mut msg: String<MAX_STRING_LENGTH> = String::new(); // Create blank string for UART print
    let max = pwm.get_max_duty();
    let duty = (max as f32 * (200.0/300.0)) as u16;
    
    info!("Max Duty Cycle: {}", max);

    pwm.enable(Channel::Ch1);
    info!("Starting LED Blinking..."); // debug way to do it
    // core::writeln!(&mut msg, "Starting LED Blinking...").unwrap();
    // usart.blocking_write(msg.as_bytes()).unwrap();
    
    loop {
        // led.set_high();
        info!("LED Has Blinked {} times", loop_counter); // debug way to do it
        // msg.clear(); // NEED to clear the string or else it keeps getting longer
        // core::writeln!(&mut msg, "LED has blinked {} times", loop_counter).unwrap();
        // usart.blocking_write(msg.as_bytes()).unwrap();

        pwm.set_duty(Channel::Ch1, duty);
        Timer::after_millis(10).await;

        
        loop_counter = loop_counter.wrapping_add(1); // use a wrapping add to avoid panics during overflow
    }
    
    // pwm.disable(Channel::Ch1);
}