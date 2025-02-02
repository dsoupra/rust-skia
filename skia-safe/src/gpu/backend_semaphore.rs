use std::fmt;

use skia_bindings::{self as sb, GrBackendSemaphore};

use crate::prelude::*;

pub type BackendSemaphore = Handle<GrBackendSemaphore>;
unsafe_send_sync!(BackendSemaphore);
impl NativeDrop for GrBackendSemaphore {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendSemaphore_destruct(self) }
    }
}
impl NativeClone for GrBackendSemaphore {
    fn clone(&self) -> Self {
        construct(|f| unsafe { sb::C_GrBackendSemaphore_CopyConstruct(f, self) })
    }
}

impl fmt::Debug for BackendSemaphore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = f.debug_struct("BackendSemaphore");
        str.finish()
    }
}
