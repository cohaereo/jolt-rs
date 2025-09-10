pub trait PointerConstExt {
    type Target;
    unsafe fn as_ref_expect(&self, field: &str) -> &Self::Target;
}

pub trait PointerMutExt {
    type Target;
    unsafe fn as_mut_expect(&self, field: &str) -> &mut Self::Target;
}

impl<T> PointerConstExt for *const T {
    type Target = T;

    unsafe fn as_ref_expect(&self, field: &str) -> &Self::Target {
        if let Some(r) = self.as_ref() {
            r
        } else {
            panic!("{field} is null!");
        }
    }
}

impl<T> PointerConstExt for *mut T {
    type Target = T;

    unsafe fn as_ref_expect(&self, field: &str) -> &Self::Target {
        if let Some(r) = self.as_ref() {
            r
        } else {
            panic!("{field} is null!");
        }
    }
}

impl<T> PointerMutExt for *mut T {
    type Target = T;

    unsafe fn as_mut_expect(&self, field: &str) -> &mut Self::Target {
        if let Some(r) = self.as_mut() {
            r
        } else {
            panic!("{field} is null!");
        }
    }
}
