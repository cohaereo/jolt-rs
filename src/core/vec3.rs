use mint::Vector3;

pub trait Vec3Ext {
    fn to_fixed_vec3(self) -> [f32; 4];
}

impl<V> Vec3Ext for V
where
    V: Into<Vector3<f32>>,
{
    fn to_fixed_vec3(self) -> [f32; 4] {
        let v = self.into();
        [v.x, v.y, v.z, v.z]
    }
}
