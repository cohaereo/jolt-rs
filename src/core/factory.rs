pub fn create_factory() {
    unsafe {
        jolt_sys::JPC_CreateFactory();
    }
}

pub fn destroy_factory() {
    unsafe {
        jolt_sys::JPC_DestroyFactory();
    }
}
