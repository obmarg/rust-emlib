use super::{hal, nb};

use super::bindings;

pub struct LEUART {
    pub(crate) ptr: *mut bindings::LEUART_TypeDef,
}

/// A serial interface for our LEUARTs
pub struct Serial {
    leuart: LEUART,
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

impl Serial {
    pub fn new(
        leuart: LEUART,
        baud_rate: u32,
        status: Status,
        parity: Parity,
        stop_bits: StopBits,
    ) -> Serial {
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
        unsafe { bindings::LEUART_Init(leuart.ptr, &init) }
        Serial { leuart: leuart }
    }
}

/// Serial Interface Error
pub enum Error {
    /// A buffer overrun.
    Overrun,
}

impl hal::serial::Read<u8> for Serial {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        unsafe {
            return Ok(bindings::LEUART_Rx(self.leuart.ptr));
        }
    }
}

impl hal::serial::Write<u8> for Serial {
    type Error = Error;

    fn flush(&mut self) -> nb::Result<(), Error> {
        unsafe {
            return Ok(());
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
        unsafe {
            bindings::LEUART_Tx(self.leuart.ptr, byte);

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
