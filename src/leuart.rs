use super::{hal, nb};

use super::bindings;

/// This trait lets us get at the emlib LEUART type for a struct.
pub trait LEUART {
    unsafe fn get_ptr() -> *mut bindings::LEUART_TypeDef;
}

/// This struct represents LEUART0.
///
/// There should only be one of these, exposed through the Peripherals struct.
pub struct LEUART0 {
    #[allow(dead_code)]
    private: (),
}

impl LEUART for LEUART0 {
    unsafe fn get_ptr() -> *mut bindings::LEUART_TypeDef {
        return bindings::LEUART0_BASE as *mut bindings::LEUART_TypeDef;
    }
}


/// Peripherals contains all the LEUART peripherals.
///
/// Users should get an instance of this and then distribute the individual
/// leuart peripherals wherever they need to go
pub struct Peripherals {
    pub leuart0: LEUART0,
    // TODO: Need to make this a singleton somehow...
}

static mut GOT_PERIPHERALS: bool = false;

impl Peripherals {
    /// Gets the LEUART Peripherals, if they haven't already been got.
    ///
    /// This function is unsafe to be run in a threaded/interrupt context.
    ///
    /// Ideally it should be called at the start of a program to initialise
    /// things.
    pub unsafe fn get() -> Option<Peripherals> {
        // This could be unsafe if we had threads or this was called from an
        // interrupt.  So don't do that.
        if GOT_PERIPHERALS {
            return None;
        }
        GOT_PERIPHERALS = true;

        Some(Peripherals {
            leuart0: LEUART0 { private: () },
        })
    }
}

/// A serial interface for our LEUARTs
pub struct Serial<Port> {
    #[allow(dead_code)]
    port: Port,
}

/// Controls the status of a LEUART
pub enum Status {
    /// Diable both the receiver and the transmitter.
    Disabled,
    /// Enable the receiver only
    Receiver,
    /// Enable the transmitter only
    Transmitter,
    /// Enable the transmitter and the receiver
    TransmitterReceiver,
}

/// Controls whether parity bits are used with the LEUART.
pub enum Parity {
    /// No parity bits.
    NoParity,
    /// Even parity bits
    Even,
    /// Odd parity bits
    Odd,
}

/// StopBits setting for LEUART
pub enum StopBits {
    One,
    Two,
}

impl Serial<LEUART0> {
    pub fn new(
        port: LEUART0,
        baud_rate: u32,
        status: Status,
        parity: Parity,
        stop_bits: StopBits,
    ) -> Serial<LEUART0> {
        let init = bindings::LEUART_Init_TypeDef {
            baudrate: baud_rate,
            enable: match status {
                Status::Disabled => bindings::LEUART_Enable_TypeDef_leuartDisable,
                Status::Receiver => bindings::LEUART_Enable_TypeDef_leuartEnableRx,
                Status::Transmitter => bindings::LEUART_Enable_TypeDef_leuartEnableTx,
                Status::TransmitterReceiver => bindings::LEUART_Enable_TypeDef_leuartEnable,
            },
            parity: match parity {
                Parity::NoParity => bindings::LEUART_Parity_TypeDef_leuartNoParity,
                Parity::Even => bindings::LEUART_Parity_TypeDef_leuartEvenParity,
                Parity::Odd => bindings::LEUART_Parity_TypeDef_leuartOddParity,
            },
            stopbits: match stop_bits {
                StopBits::One => bindings::LEUART_Stopbits_TypeDef_leuartStopbits1,
                StopBits::Two => bindings::LEUART_Stopbits_TypeDef_leuartStopbits2,
            },
            refFreq: 0,
            databits: bindings::LEUART_Databits_TypeDef_leuartDatabits8,
        };
        unsafe { bindings::LEUART_Init(LEUART0::get_ptr(), &init) }
        Serial { port: port }
    }
}

/// Serial Interface Error
pub enum Error {
    /// A buffer overrun.
    Overrun,
}

impl<Port> hal::serial::Read<u8> for Serial<Port>
where
    Port: LEUART,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        unsafe {
            let leuart = Port::get_ptr();
            if ((*leuart).STATUS & bindings::LEUART_STATUS_RXDATAV) == 0 {
                return Err(nb::Error::WouldBlock);
            }
            return Ok(bindings::LEUART_Rx(leuart));
        }
    }
}

impl<Port> hal::serial::Write<u8> for Serial<Port>
where
    Port: LEUART,
{
    type Error = Error;

    fn flush(&mut self) -> nb::Result<(), Error> {
        unsafe {
            let leuart = Port::get_ptr();
            if ((*leuart).STATUS & bindings::LEUART_STATUS_TXC) != 0 {
                return Err(nb::Error::WouldBlock);
            }

            return Ok(());
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
        unsafe {
            let leuart = Port::get_ptr();
            if ((*leuart).STATUS & bindings::LEUART_STATUS_TXBL) != 0 {
                return Err(nb::Error::WouldBlock);
            }

            bindings::LEUART_Tx(leuart, byte);

            Ok(())
        }
    }
}

impl<Port> hal::blocking::serial::write::Default<u8> for Serial<Port> where Port: LEUART {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
