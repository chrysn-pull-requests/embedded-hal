//! Digital I/O

/// Single digital push-pull output pin
pub trait OutputPin {
    /// Drives the pin low
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    fn set_low(&mut self);

    /// Drives the pin high
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    fn set_high(&mut self);
}

/// Push-pull output pin that can read its output state
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
#[cfg(feature = "unproven")]
pub trait StatefulOutputPin {
    /// Is the pin in drive high mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn is_set_high(&self) -> bool;

    /// Is the pin in drive low mode?
    ///
    /// *NOTE* this does *not* read the electrical state of the pin
    fn is_set_low(&self) -> bool;
}

/// Output pin that can be toggled
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
///
/// See [toggleable](toggleable) to use a software implementation if
/// both [OutputPin](trait.OutputPin.html) and
/// [StatefulOutputPin](trait.StatefulOutputPin.html) are
/// implemented. Otherwise, implement this using hardware mechanisms.
#[cfg(feature = "unproven")]
pub trait ToggleableOutputPin {
    /// Toggle pin output.
    fn toggle(&mut self);
}

/// If you can read **and** write the output state, a pin is
/// toggleable by software.
///
/// ```
/// use embedded_hal::digital::{OutputPin, StatefulOutputPin, ToggleableOutputPin};
/// use embedded_hal::digital::toggleable;
///
/// /// A virtual output pin that exists purely in software
/// struct MyPin {
///     state: bool
/// }
///
/// impl OutputPin for MyPin {
///    fn set_low(&mut self) {
///        self.state = false;
///    }
///    fn set_high(&mut self) {
///        self.state = true;
///    }
/// }
///
/// impl StatefulOutputPin for MyPin {
///    fn is_set_low(&self) -> bool {
///        !self.state
///    }
///    fn is_set_high(&self) -> bool {
///        self.state
///    }
/// }
///
/// /// Opt-in to the software implementation.
/// impl toggleable::Default for MyPin {}
///
/// let mut pin = MyPin { state: false };
/// pin.toggle();
/// assert!(pin.is_set_high());
/// pin.toggle();
/// assert!(pin.is_set_low());
/// ```
#[cfg(feature = "unproven")]
pub mod toggleable {
    use super::{OutputPin, StatefulOutputPin, ToggleableOutputPin};

    /// Software-driven `toggle()` implementation.
    ///
    /// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
    pub trait Default: OutputPin + StatefulOutputPin {}

    impl<P> ToggleableOutputPin for P
    where
        P: Default,
    {
        /// Toggle pin output
        fn toggle(&mut self) {
            if self.is_set_low() {
                self.set_high();
            } else {
                self.set_low();
            }
        }
    }
}

/// Single digital input pin
///
/// *This trait is available if embedded-hal is built with the `"unproven"` feature.*
#[cfg(feature = "unproven")]
pub trait InputPin {
    /// Is the input pin high?
    fn is_high(&self) -> bool;

    /// Is the input pin low?
    fn is_low(&self) -> bool;
}
