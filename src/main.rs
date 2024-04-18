/*
* Project-Name: Sensor controlled Relay
* Breif: This product used to control a relay according to the sensor value
* Author - Jishnu E
* Date - 18/04/24

*/

// Include HAL library 
use esp_idf_hal::{delay::FreeRtos,adc::{self, *},gpio::PinDriver, peripherals::Peripherals};


fn main() {
    // services for esp idf in RUST
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Get all the peripherals
    let peripherals = Peripherals::take().unwrap();

    // Initialize Pin 8 as an output to drive the LED
    let mut led_pin = PinDriver::output(peripherals.pins.gpio8).unwrap();

    // ADC initialization 
    let mut adc1 = AdcDriver::new(
        peripherals.adc1,
        &adc::config::Config::new().calibration(true),
    )
    .unwrap();
    let mut a1_ch0 =
        adc::AdcChannelDriver::<{adc::attenuation::DB_11 }, _>::new(peripherals.pins.gpio0)
            .unwrap();

    // Loop forever to check the input sensor value with delay of 1 sec
    loop 
    {
           // by using a control flow statement like 'match' we can  read the adc value
           match adc1.read(&mut a1_ch0) 
           {
                    Ok(x) => {
                                log::info!("A1_CH0: {x}\n");
                                if x > 100 
                                {
                                    // Turn on LED (here using inverse logic)
                                    led_pin.set_low().unwrap();
                                    log::info!("LED ON");
                                } 
                                else 
                                {
                                    // Turn off LED
                                    led_pin.set_high().unwrap();
                                    log::info!("LED OFF");
                                }

                              },
                    Err(e) => log::info!("err reading A1_CH0: {e}\n"),
            }
            FreeRtos::delay_ms(1000);
    }
}



