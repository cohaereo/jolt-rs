pub struct JobSystem(*mut jolt_sys::JPC_JobSystem);

impl JobSystem {
    pub const MAX_PHYSICS_JOBS: u32 = jolt_sys::JPC_JobSystemConstants_JPC_MAX_PHYSICS_JOBS as _;
    pub const MAX_PHYSICS_BARRIERS: u32 =
        jolt_sys::JPC_JobSystemConstants_JPC_MAX_PHYSICS_BARRIERS as _;

    pub fn create(max_jobs: u32, max_barriers: u32, num_threads: i32) -> Self {
        unsafe {
            JobSystem(jolt_sys::JPC_JobSystem_Create(
                max_jobs,
                max_barriers,
                num_threads,
            ))
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut jolt_sys::JPC_JobSystem {
        self.0
    }
}

impl Drop for JobSystem {
    fn drop(&mut self) {
        unsafe {
            jolt_sys::JPC_JobSystem_Destroy(self.0);
        }
    }
}
