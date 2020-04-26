//! General Purpose Input/Output (GPIO)

use core::marker::PhantomData;
use core::sync::atomic::AtomicU32;
use crate::pac;
use crate::fpioa::{IoPin, Pull};
use crate::bit_utils::{u32_atomic_set_bit, u32_atomic_toggle_bit, u32_bit_is_set, u32_bit_is_clear};
use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, InputPin, ToggleableOutputPin};

/// Extension trait to split a GPIO peripheral into independent pins
pub trait GpioExt {
    /// Split the GPIO peripheral into parts
    /// 
    /// todo: require ownership of sysctl part
    fn split(self) -> Parts;
}

macro_rules! def_gpio_pins {
    ($($GPIOX: ident: ($num: expr, $gpiox: ident);)+) => {
        
impl GpioExt for pac::GPIO {
    fn split(self) -> Parts {
        // todo: use sysctl part to enable clock
        Parts { 
            $( $gpiox: $GPIOX { _ownership: () }, )+
        }
    }
}

/// GPIO peripheral parts
pub struct Parts {
    $( pub $gpiox: $GPIOX, )+
}

pub use gpio_pins::*;

/// All GPIO pins
pub mod gpio_pins {
    use super::GpioIndex;
$(
    /// GPIO pin
    pub struct $GPIOX {
        pub(crate) _ownership: ()
    }

    impl GpioIndex for $GPIOX {
        const INDEX: u8 = $num;
    }
)+
}
    };
}

def_gpio_pins! {
    GPIO0: (0, gpio0);
    GPIO1: (1, gpio1);
    GPIO2: (2, gpio2);
    GPIO3: (3, gpio3);
    GPIO4: (4, gpio4);
    GPIO5: (5, gpio5);
    GPIO6: (6, gpio6);
    GPIO7: (7, gpio7);
}

/// GPIO Index
pub trait GpioIndex {
    const INDEX: u8;
}

/// Input mode (type state)
pub struct Input<MODE>(MODE);

/// Floating input (type state)
pub struct Floating;

/// Pull down input (type state)
pub struct PullDown;

/// Pull up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output;

/// Marker trait for active states
pub trait Active {}

impl Active for Input<Floating> {}

impl Active for Input<PullUp> {}

impl Active for Input<PullDown> {}

impl Active for Output {}

/// GPIO wrapper struct
pub struct Gpio<GPIO, PIN, MODE> {
    gpio: GPIO,
    pin: PIN,
    _mode: PhantomData<MODE>,
}

impl<GPIO, PIN> Gpio<GPIO, PIN, Input<Floating>> {
    // todo: verify default GPIO mode
    pub fn new(gpio: GPIO, pin: PIN) -> Gpio<GPIO, PIN, Input<Floating>> {
        Gpio { gpio, pin, _mode: PhantomData }
    }
}

impl<GPIO: GpioIndex, PIN, MODE> InputPin for Gpio<GPIO, PIN, Input<MODE>> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> { 
        let r: &u32 = unsafe { &*(&(*pac::GPIO::ptr()).data_input as *const _ as *const _) };
        Ok(u32_bit_is_set(r, GPIO::INDEX as usize))
    }

    fn is_low(&self) -> Result<bool, Self::Error> { 
        let r: &u32 = unsafe { &*(&(*pac::GPIO::ptr()).data_input as *const _ as *const _) };
        Ok(u32_bit_is_clear(r, GPIO::INDEX as usize))
    }
}

impl<GPIO: GpioIndex, PIN: IoPin, MODE: Active> Gpio<GPIO, PIN, MODE> {
    pub fn into_floating_input(mut self) -> Gpio<GPIO, PIN, Input<Floating>> {
        self.pin.set_io_pull(Pull::None);
        let r: &AtomicU32 = unsafe { &*(&(*pac::GPIO::ptr()).direction as *const _ as *const _) };
        u32_atomic_set_bit(r, false, GPIO::INDEX as usize);
        Gpio { gpio: self.gpio, pin: self.pin, _mode: PhantomData }
    }

    pub fn into_pull_up_input(mut self) -> Gpio<GPIO, PIN, Input<PullUp>> {
        self.pin.set_io_pull(Pull::Up);
        let r: &AtomicU32 = unsafe { &*(&(*pac::GPIO::ptr()).direction as *const _ as *const _) };
        u32_atomic_set_bit(r, false, GPIO::INDEX as usize);
        Gpio { gpio: self.gpio, pin: self.pin, _mode: PhantomData }
    }

    pub fn into_pull_down_input(mut self) -> Gpio<GPIO, PIN, Input<PullDown>> {
        self.pin.set_io_pull(Pull::Down);
        let r: &AtomicU32 = unsafe { &*(&(*pac::GPIO::ptr()).direction as *const _ as *const _) };
        u32_atomic_set_bit(r, false, GPIO::INDEX as usize);
        Gpio { gpio: self.gpio, pin: self.pin, _mode: PhantomData }
    }

    pub fn into_push_pull_output(mut self) -> Gpio<GPIO, PIN, Output> {
        self.pin.set_io_pull(Pull::Down);
        let r: &AtomicU32 = unsafe { &*(&(*pac::GPIO::ptr()).direction as *const _ as *const _) };
        u32_atomic_set_bit(r, true, GPIO::INDEX as usize);
        Gpio { gpio: self.gpio, pin: self.pin, _mode: PhantomData }
    }
}

impl<GPIO: GpioIndex, PIN> OutputPin for Gpio<GPIO, PIN, Output> {
    type Error = core::convert::Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let r: &AtomicU32 = unsafe { &*(&(*pac::GPIO::ptr()).data_output as *const _ as *const _) };
        u32_atomic_set_bit(r, true, GPIO::INDEX as usize);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        let r: &AtomicU32 = unsafe { &*(&(*pac::GPIO::ptr()).data_output as *const _ as *const _) };
        u32_atomic_set_bit(r, false, GPIO::INDEX as usize);
        Ok(())
    }
}

impl<GPIO: GpioIndex, PIN> StatefulOutputPin for Gpio<GPIO, PIN, Output> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        let r: &u32 = unsafe { &*(&(*pac::GPIO::ptr()).data_output as *const _ as *const _) };
        Ok(u32_bit_is_set(r, GPIO::INDEX as usize))
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> { 
        let r: &u32 = unsafe { &*(&(*pac::GPIO::ptr()).data_output as *const _ as *const _) };
        Ok(u32_bit_is_clear(r, GPIO::INDEX as usize))
    }
}

// todo: fix atomic operation

// impl<PIN> ToggleableOutputPin for Gpio<GPIO6, PIN, Output> {
//     type Error = core::convert::Infallible;

//     fn toggle(&mut self) -> Result<(), Self::Error> { 
//         let r: &AtomicU32 = unsafe { &*(&(*GPIO::ptr()).data_output as *const _ as *const _) };
//         u32_atomic_toggle_bit(r, 6);
//         Ok(())
//     }
// }

impl<GPIO: GpioIndex, PIN> embedded_hal::digital::v2::toggleable::Default for Gpio<GPIO, PIN, Output> {}
