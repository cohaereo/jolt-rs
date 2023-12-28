use jolt_sys::JPC_BodyID;
use std::ffi::c_void;

// JPC_BodyActivationListenerVTable
pub trait BodyActivationListener {
    fn on_body_activated(&self, body_id: JPC_BodyID, user_data: u64);
    fn on_body_deactivated(&self, body_id: JPC_BodyID, user_data: u64);
}

#[repr(C)]
pub struct BodyActivationListenerWrapper {
    pub(crate) vtable: *const jolt_sys::JPC_BodyActivationListenerVTable,
    pub(crate) inner: Box<dyn BodyActivationListener>,
}

impl BodyActivationListenerWrapper {
    pub fn new(inner: Box<dyn BodyActivationListener>) -> Self {
        let vtable = Box::new(jolt_sys::JPC_BodyActivationListenerVTable {
            __vtable_header: unsafe { std::mem::zeroed() },
            OnBodyActivated: Some(Self::on_body_activated),
            OnBodyDeactivated: Some(Self::on_body_deactivated),
        });

        Self {
            vtable: Box::into_raw(vtable),
            inner,
        }
    }

    unsafe extern "C" fn on_body_activated(
        wrapper: *mut c_void,
        in_body_id: *const JPC_BodyID,
        in_user_data: u64,
    ) {
        (*(wrapper as *const Self))
            .inner
            .on_body_activated(*in_body_id, in_user_data)
    }

    unsafe extern "C" fn on_body_deactivated(
        wrapper: *mut c_void,
        in_body_id: *const JPC_BodyID,
        in_user_data: u64,
    ) {
        (*(wrapper as *const Self))
            .inner
            .on_body_deactivated(*in_body_id, in_user_data)
    }
}
