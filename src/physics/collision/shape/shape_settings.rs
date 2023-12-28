use crate::ShapeRef;
use jolt_sys::JPC_ShapeSettings;

pub trait ShapeSettings {
    fn as_shape_settings(&self) -> *const JPC_ShapeSettings;

    fn create_shape(&self) -> Result<ShapeRef, ()> {
        unsafe {
            let shape = jolt_sys::JPC_ShapeSettings_CreateShape(self.as_shape_settings());
            if shape.is_null() {
                Err(())
            } else {
                Ok(ShapeRef::from_inner(shape))
            }
        }
    }
}
