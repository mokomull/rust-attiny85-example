extern crate embedded_hal as hal;
extern crate attiny85_hal;

pub struct Switch<Input, Output, Led> {
    input: Input,
    output: Output,
    led: Led,
    active: bool,
    was_pressed: bool,
}

// #[duplicate(
//     input_type output_type led_type;
//     [ PB0 ]    [ PB3 ]     [ PB2 ];
//     [ PB1 ]    [ PB4 ]     [ PB5 ];
// )]
impl<Input, Output, Led> Switch<Input, Output, Led> where
    Input: hal::digital::v2::InputPin,
    Output: hal::digital::v2::OutputPin,
    Led: hal::digital::v2::OutputPin,
    Input::Error: core::fmt::Debug,
    Output::Error: core::fmt::Debug,
    Led::Error: core::fmt::Debug,
{
    pub fn new(input: Input, output: Output, led: Led) -> Self {
    // pub fn new(input: portb::input_type, output: portb::output_type, led: portb::led_type) -> Self {
        Switch { input, output, led, active: false, was_pressed: false }
    }

    pub fn check(&mut self) {
        let pressed = self.is_pressed();

        if !self.was_pressed && pressed {
            self.active = !self.active;
            self.was_pressed = true;

            self.set_state(self.active);
        } else if self.was_pressed && !pressed {
            self.was_pressed = false;

            // TODO: check to see if minimum time limit has been met
            // if it has been, turn the effect off
        }
    }

    fn is_pressed(&mut self) -> bool {
        self.input.is_low().unwrap()
    }

    fn set_state(&mut self, state: bool) {
        self.set_led(state);
        self.set_switch(state);
    }

    fn set_led(&mut self, state: bool) {
        if state {
            self.led.set_high().unwrap();
        } else {
            self.led.set_low().unwrap();
        }
    }

    fn set_switch(&mut self, state: bool) {
        if state {
            self.output.set_high().unwrap();
        } else {
            self.output.set_low().unwrap();
        }
    }
}
