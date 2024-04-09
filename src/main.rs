#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*; // if info!() or other debug macros are used
use embassy_executor::Spawner;
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

    let ch1 = PwmPin::new_ch1(p.PA5, OutputType::PushPull);
    let mut pwm = SimplePwm::new(p.TIM2, Some(ch1), None, None, None, hz(2), Default::default());

    // Variables
    let max = pwm.get_max_duty(); // equates to 65416
    let duty = (max as f32 * (200.0/(200.0+300.0))) as u16; // equates to 25806
    
    info!("Max Duty Cycle: {}", max);

    pwm.enable(Channel::Ch1);
    info!("Starting LED Blinking..."); // debug way to do it
    
    loop {

        pwm.set_duty(Channel::Ch1, duty);
        Timer::after_millis(10).await;

    }
    
    // pwm.disable(Channel::Ch1);
}