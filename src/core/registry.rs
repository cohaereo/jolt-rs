pub fn register_types() {
    unsafe {
        jolt_sys::JPC_RegisterTypes();
    }
}
