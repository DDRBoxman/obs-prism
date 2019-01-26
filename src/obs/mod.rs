use std::os::raw::{c_char, c_int, c_void};

#[repr(C)] // ensures that the memory layout of the struct is the same in Rust as in C.
pub struct obs_module_t {
    pub mod_name: *mut c_char,
    pub file: *const c_char,
    pub bin_path: *mut c_char,
    pub data_path: *mut c_char,
    pub module: *mut c_void,
    pub loaded: bool,

    load: extern "C" fn() -> bool,
    unload: extern "C" fn(),
    post_load: extern "C" fn(),
    set_locale: extern "C" fn(locale: *const c_char),
    free_locale: extern "C" fn(),
    ver: extern "C" fn() -> u32,
    set_pointer: extern "C" fn(module: *mut obs_module_t),
    name: extern "C" fn() -> *const c_char,
    description: extern "C" fn() -> *const c_char,
    author: extern "C" fn() -> *const c_char,
    next: *mut obs_module_t,
}

#[repr(C)]
pub enum obs_frontend_event {
    OBS_FRONTEND_EVENT_STREAMING_STARTING,
    OBS_FRONTEND_EVENT_STREAMING_STARTED,
    OBS_FRONTEND_EVENT_STREAMING_STOPPING,
    OBS_FRONTEND_EVENT_STREAMING_STOPPED,
    OBS_FRONTEND_EVENT_RECORDING_STARTING,
    OBS_FRONTEND_EVENT_RECORDING_STARTED,
    OBS_FRONTEND_EVENT_RECORDING_STOPPING,
    OBS_FRONTEND_EVENT_RECORDING_STOPPED,
    OBS_FRONTEND_EVENT_SCENE_CHANGED,
    OBS_FRONTEND_EVENT_SCENE_LIST_CHANGED,
    OBS_FRONTEND_EVENT_TRANSITION_CHANGED,
    OBS_FRONTEND_EVENT_TRANSITION_STOPPED,
    OBS_FRONTEND_EVENT_TRANSITION_LIST_CHANGED,
    OBS_FRONTEND_EVENT_SCENE_COLLECTION_CHANGED,
    OBS_FRONTEND_EVENT_SCENE_COLLECTION_LIST_CHANGED,
    OBS_FRONTEND_EVENT_PROFILE_CHANGED,
    OBS_FRONTEND_EVENT_PROFILE_LIST_CHANGED,
    OBS_FRONTEND_EVENT_EXIT,

    OBS_FRONTEND_EVENT_REPLAY_BUFFER_STARTING,
    OBS_FRONTEND_EVENT_REPLAY_BUFFER_STARTED,
    OBS_FRONTEND_EVENT_REPLAY_BUFFER_STOPPING,
    OBS_FRONTEND_EVENT_REPLAY_BUFFER_STOPPED,

    OBS_FRONTEND_EVENT_STUDIO_MODE_ENABLED,
    OBS_FRONTEND_EVENT_STUDIO_MODE_DISABLED,
    OBS_FRONTEND_EVENT_PREVIEW_SCENE_CHANGED,

    OBS_FRONTEND_EVENT_SCENE_COLLECTION_CLEANUP,
    OBS_FRONTEND_EVENT_FINISHED_LOADING,
}

#[link(name = "obs-frontend-api")]
extern "C" {
    pub fn obs_frontend_add_event_callback(
        callback: extern "C" fn(event: obs_frontend_event, private_dat: *mut c_void),
        private_data: *mut c_void,
    );
}
