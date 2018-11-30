use core::ffi::c_void;

use hal::blocking::spi::{Transfer, Write};
use hal::spi::{Mode, Phase, Polarity};

use super::bindings;

/// Represents a single USART port.
///
/// Usually you'll take this from the Peripherals struct rather than creating it
/// yourself.
///
/// Note that in real life you could have multiple things attached to a single
/// USART instance, but for now this is unsupported in rust. Need to figure out  
/// how to share a USART while ensuring pins are not shared.
pub struct USART {
    pub(crate) ptr: *mut bindings::USART_TypeDef,
}

pub enum Pin {
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
    Pin8,
    Pin9,
    Pin10,
    Pin11,
    Pin12,
    Pin13,
    Pin14,
    Pin15,
    Pin16,
    Pin17,
    Pin18,
    Pin19,
    Pin20,
    Pin21,
    Pin22,
    Pin23,
    Pin24,
    Pin25,
    Pin26,
    Pin27,
    Pin28,
    Pin29,
    Pin30,
    Pin31,
}

fn pin_to_number(pin: Pin) -> u8 {
    match pin {
        Pin::Pin0 => 0,
        Pin::Pin1 => 1,
        Pin::Pin2 => 2,
        Pin::Pin3 => 3,
        Pin::Pin4 => 4,
        Pin::Pin5 => 5,
        Pin::Pin6 => 6,
        Pin::Pin7 => 7,
        Pin::Pin8 => 8,
        Pin::Pin9 => 9,
        Pin::Pin10 => 10,
        Pin::Pin11 => 11,
        Pin::Pin12 => 12,
        Pin::Pin13 => 13,
        Pin::Pin14 => 14,
        Pin::Pin15 => 15,
        Pin::Pin16 => 16,
        Pin::Pin17 => 17,
        Pin::Pin18 => 18,
        Pin::Pin19 => 19,
        Pin::Pin20 => 20,
        Pin::Pin21 => 21,
        Pin::Pin22 => 22,
        Pin::Pin23 => 23,
        Pin::Pin24 => 24,
        Pin::Pin25 => 25,
        Pin::Pin26 => 26,
        Pin::Pin27 => 27,
        Pin::Pin28 => 28,
        Pin::Pin29 => 29,
        Pin::Pin30 => 30,
        Pin::Pin31 => 31,
    }
}

// TODO: should probably be a module specifically for SPI

/// The pins on a USART to run our SPI on.
pub struct SPIPins {
    /// The transmit pin
    tx: Pin,
    /// The receive pin
    rx: Pin,
    /// The clock pin
    clk: Pin,
    /// The chip select pin
    cs: Pin,
}

/// The bit order for an SPI
pub enum BitOrder {
    /// Most significant bit first
    MSBFirst,
    /// Least significant bit first
    LSBFirst,
}

/// Implements SPI on top of a USART.
///
/// Note that we don't support slave mode just now.
pub struct SPI {
    #[allow(dead_code)]
    port: USART,
    handle_data: bindings::SPIDRV_HandleData,
}

/// Errors that can happen when working with SPI.
///
/// Not really implemented properly just now.  Can do that later.
pub enum SPIError {
    Unknown(u32),
    TransferTooBig,
}

impl SPI {
    pub fn new(
        port: USART,
        pins: SPIPins,
        bit_rate: u32,
        clock_mode: Mode,
        bit_order: BitOrder,
    ) -> Result<SPI, SPIError> {
        let mut config = bindings::SPIDRV_Init {
            port: port.ptr,
            portLocationTx: pin_to_number(pins.tx),
            portLocationRx: pin_to_number(pins.rx),
            portLocationClk: pin_to_number(pins.clk),
            portLocationCs: pin_to_number(pins.cs),
            bitRate: bit_rate,
            frameLength: 8,
            dummyTxValue: 0,
            type_: bindings::SPIDRV_Type_spidrvMaster,
            bitOrder: match bit_order {
                BitOrder::MSBFirst => bindings::SPIDRV_BitOrder_spidrvBitOrderMsbFirst,
                BitOrder::LSBFirst => bindings::SPIDRV_BitOrder_spidrvBitOrderLsbFirst,
            },
            clockMode: match (clock_mode.polarity, clock_mode.phase) {
                (Polarity::IdleLow, Phase::CaptureOnFirstTransition) => {
                    bindings::SPIDRV_ClockMode_spidrvClockMode0
                }
                (Polarity::IdleLow, Phase::CaptureOnSecondTransition) => {
                    bindings::SPIDRV_ClockMode_spidrvClockMode1
                }
                (Polarity::IdleHigh, Phase::CaptureOnFirstTransition) => {
                    bindings::SPIDRV_ClockMode_spidrvClockMode2
                }
                (Polarity::IdleHigh, Phase::CaptureOnSecondTransition) => {
                    bindings::SPIDRV_ClockMode_spidrvClockMode3
                }
            },
            csControl: bindings::SPIDRV_CsControl_spidrvCsControlAuto,
            slaveStartMode: bindings::SPIDRV_SlaveStart_spidrvSlaveStartImmediate,
        };

        // TODO: Figure out if there's a better way to do this...
        // uninitialized sounds like a nightmare
        unsafe {
            let mut handle_data: bindings::SPIDRV_HandleData = { core::mem::uninitialized() };
            match bindings::SPIDRV_Init(&mut handle_data, &mut config) {
                bindings::ECODE_OK => Ok(SPI {
                    port: port,
                    handle_data: handle_data,
                }),
                error => Err(SPIError::Unknown(error)),
            }
        }
    }
}

impl Transfer<u8> for SPI {
    type Error = SPIError;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        if words.len() > (i32::max_value() as usize) {
            return Err(SPIError::TransferTooBig);
        }
        match unsafe {
            bindings::SPIDRV_MTransferB(
                &mut self.handle_data,
                words.as_ptr() as *const c_void,
                words.as_mut_ptr() as *mut c_void,
                words.len() as i32,
            )
        } {
            bindings::ECODE_OK => Ok(words),
            error => Err(SPIError::Unknown(error)),
        }
    }
}

impl Write<u8> for SPI {
    type Error = SPIError;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        if words.len() > (i32::max_value() as usize) {
            return Err(SPIError::TransferTooBig);
        }
        match unsafe {
            bindings::SPIDRV_MTransmitB(
                &mut self.handle_data,
                words.as_ptr() as *const c_void,
                words.len() as i32,
            )
        } {
            bindings::ECODE_OK => Ok(()),
            error => Err(SPIError::Unknown(error)),
        }
    }
}
