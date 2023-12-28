pub fn register_default_allocator() {
    unsafe {
        jolt_sys::JPC_RegisterDefaultAllocator();
    }
}

pub struct TempAllocator(*mut jolt_sys::JPC_TempAllocator);

impl TempAllocator {
    pub fn create(size: u32) -> Self {
        unsafe { TempAllocator(jolt_sys::JPC_TempAllocator_Create(size)) }
    }

    pub(crate) fn as_ptr(&self) -> *mut jolt_sys::JPC_TempAllocator {
        self.0
    }
}
