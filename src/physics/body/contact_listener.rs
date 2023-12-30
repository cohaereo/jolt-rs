use crate::{Body, CollideShapeResult, ContactManifold, ContactSettings, SubShapeIDPair};
use jolt_sys::{
    JPC_Body, JPC_CollideShapeResult, JPC_ContactManifold, JPC_ContactSettings, JPC_Real,
    JPC_SubShapeIDPair, JPC_ValidateResult,
    JPC_ValidateResult_JPC_VALIDATE_RESULT_ACCEPT_ALL_CONTACTS,
    JPC_ValidateResult_JPC_VALIDATE_RESULT_ACCEPT_CONTACT,
    JPC_ValidateResult_JPC_VALIDATE_RESULT_REJECT_ALL_CONTACTS,
    JPC_ValidateResult_JPC_VALIDATE_RESULT_REJECT_CONTACT,
};
use mint::Vector3;
use std::ffi::c_void;

#[repr(u32)]
pub enum ValidateResult {
    AcceptAllContactsForThisBodyPair =
        JPC_ValidateResult_JPC_VALIDATE_RESULT_ACCEPT_ALL_CONTACTS as _,
    AcceptContact = JPC_ValidateResult_JPC_VALIDATE_RESULT_ACCEPT_CONTACT as _,
    RejectContact = JPC_ValidateResult_JPC_VALIDATE_RESULT_REJECT_CONTACT as _,
    RejectAllContactsForThisBodyPair =
        JPC_ValidateResult_JPC_VALIDATE_RESULT_REJECT_ALL_CONTACTS as _,
}

// JPC_ContactListenerVTable
pub trait ContactListener {
    fn on_contact_validate(
        &self,
        body1: &Body,
        body2: &Body,
        base_offset: Vector3<f32>,
        collision_result: &CollideShapeResult,
    ) -> ValidateResult;

    fn on_contact_added(
        &self,
        body1: &Body,
        body2: &Body,
        manifold: &ContactManifold,
        io_settings: &mut ContactSettings,
    );

    fn on_contact_persisted(
        &self,
        body1: &Body,
        body2: &Body,
        manifold: &ContactManifold,
        io_settings: &mut ContactSettings,
    );

    fn on_contact_removed(&self, sub_shape_pair: &SubShapeIDPair);
}

#[repr(C)]
pub struct ContactListenerWrapper {
    pub(crate) vtable: *const jolt_sys::JPC_ContactListenerVTable,
    pub(crate) inner: Box<dyn ContactListener>,
}

impl ContactListenerWrapper {
    pub fn new(inner: Box<dyn ContactListener>) -> Self {
        let vtable = Box::new(jolt_sys::JPC_ContactListenerVTable {
            OnContactValidate: Some(Self::on_contact_validate),
            OnContactAdded: Some(Self::on_contact_added),
            OnContactPersisted: Some(Self::on_contact_persisted),
            OnContactRemoved: Some(Self::on_contact_removed),
        });

        Self {
            vtable: Box::into_raw(vtable),
            inner,
        }
    }

    unsafe extern "C" fn on_contact_validate(
        wrapper: *mut c_void,
        in_body1: *const JPC_Body,
        in_body2: *const JPC_Body,
        in_base_offset: *const JPC_Real,
        in_collision_result: *const JPC_CollideShapeResult,
    ) -> JPC_ValidateResult {
        (*(wrapper as *const Self)).inner.on_contact_validate(
            &*in_body1,
            &*in_body2,
            *(in_base_offset as *const Vector3<f32>),
            &*in_collision_result,
        ) as _
    }

    unsafe extern "C" fn on_contact_added(
        wrapper: *mut c_void,
        in_body1: *const JPC_Body,
        in_body2: *const JPC_Body,
        in_manifold: *const JPC_ContactManifold,
        io_settings: *mut JPC_ContactSettings,
    ) {
        (*(wrapper as *const Self)).inner.on_contact_added(
            &*in_body1,
            &*in_body2,
            &*in_manifold,
            &mut *io_settings,
        )
    }

    unsafe extern "C" fn on_contact_persisted(
        wrapper: *mut c_void,
        in_body1: *const JPC_Body,
        in_body2: *const JPC_Body,
        in_manifold: *const JPC_ContactManifold,
        io_settings: *mut JPC_ContactSettings,
    ) {
        (*(wrapper as *const Self)).inner.on_contact_persisted(
            &*in_body1,
            &*in_body2,
            &*in_manifold,
            &mut *io_settings,
        )
    }

    unsafe extern "C" fn on_contact_removed(
        wrapper: *mut c_void,
        in_sub_shape_pair: *const JPC_SubShapeIDPair,
    ) {
        (*(wrapper as *const Self))
            .inner
            .on_contact_removed(&*in_sub_shape_pair)
    }
}
