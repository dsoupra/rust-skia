pub mod backend_semaphore {
    use skia_bindings as sb;
    use crate::gpu::vk;
    use crate::gpu::backend_semaphore::BackendSemaphore;
    pub fn new_vulkan(semaphore: vk::Semaphore) -> BackendSemaphore {
       BackendSemaphore::construct(|target| unsafe {
            sb::C_BackendSemaphores_ConstructVulkan(target, semaphore)
        })
    }
}
