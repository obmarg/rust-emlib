use super::bindings;
use super::leuart::LEUART;
use super::usart::USART;

/// Peripherals contains all the LEUART peripherals.
///
/// Users should get an instance of this and then distribute the individual
/// leuart peripherals wherever they need to go
pub struct Peripherals {
    pub leuart0: LEUART,
    pub usart0: USART,
    pub usart1: USART,
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
            leuart0: LEUART {
                ptr: bindings::LEUART0_BASE as *mut bindings::LEUART_TypeDef,
            },
            usart0: USART {
                ptr: bindings::USART0_BASE as *mut bindings::USART_TypeDef,
            },
            usart1: USART {
                ptr: bindings::USART1_BASE as *mut bindings::USART_TypeDef,
            },
        })
    }
}
