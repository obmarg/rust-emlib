use super::bindings;

pub fn handle_ldma() {
    unsafe {
        bindings::LDMA_IRQHandler();
    }
}
