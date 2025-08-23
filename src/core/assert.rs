use jolt_sys::{JPC_AssertFailedFunction, JPC_SetAssertFailedHandler};

pub fn set_assert_failed_handler(handler: JPC_AssertFailedFunction) {
    unsafe {
        JPC_SetAssertFailedHandler(handler);
    }
}
