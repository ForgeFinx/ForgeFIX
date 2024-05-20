use forgefix::fix::encode::SerializedInt;
use forgefix::fix::generated::Tags;
use forgefix::{SessionSettingsBuilder, SessionSettings, ApplicationError, FixApplicationHandle, FixApplicationInitiator};

use std::ffi::{c_char, c_ulong, CStr, c_int};
use std::net::SocketAddr;
use std::time::Duration;

const TIME_FORMAT: &str = "%H:%M:%S";

#[repr(C)]
#[derive(Debug)]
pub enum CFixError {
    OK,
    IoError,
    SessionEnded,
    LogonFailed,
    LogoutFailed,
    SendMessageFailed,
    NullPointer,
    BadString,
    SettingRequired, 
    Unknown,
}

impl<T> From<Result<T, ApplicationError>> for CFixError {
    fn from(res: Result<T, ApplicationError>) -> CFixError {
        match res {
            Ok(_) => CFixError::OK,
            Err(ApplicationError::IoError(_)) => CFixError::IoError,
            Err(ApplicationError::SessionEnded) => CFixError::SessionEnded,
            Err(ApplicationError::LogonFailed) => CFixError::LogonFailed,
            Err(ApplicationError::LogoutFailed) => CFixError::LogoutFailed,
            Err(ApplicationError::SendMessageFailed) => CFixError::SendMessageFailed,
            Err(ApplicationError::SettingRequired(..)) => CFixError::SettingRequired,
        }
    }
}

#[allow(non_camel_case_types)]
pub type fix_app_client_t = *mut BlockingFixApplicationClient;

/// # Safety
///
/// This function should be called with Utf-8 valid strings.
#[no_mangle]
pub unsafe extern "C" fn fix_app_client_build(settings: session_settings_t) -> fix_app_client_t {
    if settings.is_null() {
        return std::ptr::null_mut();
    }

    let fix_app = match BlockingFixApplicationClient::build((*settings).clone()) {
        Ok(app) => app,
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(fix_app))
}

/// # Safety
///
/// The fix_app_client_t will be mutated using this function.
#[no_mangle]
pub unsafe extern "C" fn fix_app_client_start(client: fix_app_client_t) -> CFixError {
    if client.is_null() {
        return CFixError::NullPointer;
    }
    (*client).start().into()
}

/// # Safety
///
/// fix_app_client_t should not be NULL.
#[no_mangle]
pub unsafe extern "C" fn fix_app_client_end(client: fix_app_client_t) -> CFixError {
    if client.is_null() {
        return CFixError::NullPointer;
    }
    (*client).end().into()
}

/// # Safety
///
/// Only pass in a fix_app_client_t that came from fix_app_client_new function.
#[no_mangle]
pub unsafe extern "C" fn fix_app_client_free(client: fix_app_client_t) {
    if !client.is_null() {
        drop(Box::from_raw(client));
    }
}

/// # Safety
///
/// Neither fix_app_client_t nor message_builder_t should be NULL when passed in. message_builder_t
/// will be NULL if the function retures OK.
#[no_mangle]
pub unsafe extern "C" fn fix_app_client_send_message(
    client: fix_app_client_t,
    builder: message_builder_t,
) -> CFixError {
    if client.is_null() || builder.is_null() {
        return CFixError::NullPointer;
    }
    let builder: MessageBuilder = *Box::from_raw(builder);
    (*client).send_message(builder).into()
}

pub struct BlockingFixApplicationClient {
    inner: FixApplicationHandle,
}

impl BlockingFixApplicationClient {
    #[allow(clippy::too_many_arguments)]
    pub fn build(settings: SessionSettings) -> Result<BlockingFixApplicationClient, ApplicationError> {
        let fix_app_initiator = FixApplicationInitiator::build(settings)?;
        let (inner, mut event_receiver) = fix_app_initiator.initiate_sync()?; 
        event_receiver.close();

        Ok(BlockingFixApplicationClient { inner })
    }

    pub fn start(&mut self) -> Result<(), ApplicationError> {
        self.inner.start_sync()
    }

    pub fn end(&mut self) -> Result<(), ApplicationError> {
        self.inner.end_sync()
    }

    pub fn send_message(&mut self, builder: MessageBuilder) -> Result<(), ApplicationError> {
        self.inner.send_message_sync(builder)
    }
}

pub type MessageBuilder = forgefix::fix::encode::MessageBuilder;

#[allow(non_camel_case_types)]
pub type message_builder_t = *mut MessageBuilder;

/// # Safety
///
/// Only Utf-8 valid strings and valid FIX MsgTypes should be passed to this function.
#[no_mangle]
pub unsafe extern "C" fn message_builder_new(
    begin_string: *const c_char,
    msg_type: c_char,
) -> message_builder_t {
    if begin_string.is_null() {
        return std::ptr::null_mut();
    }
    let begin_string = match CStr::from_ptr(begin_string).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let msg_type = msg_type as u8 as char;
    let builder = MessageBuilder::new(begin_string, msg_type);
    Box::into_raw(Box::new(builder))
}

/// # Safety
///
/// The message_builder_t should not be NULL.
#[no_mangle]
pub unsafe extern "C" fn message_builder_push_str(
    builder: message_builder_t,
    tag_param: Tags,
    value: *const c_char,
) -> CFixError {
    if builder.is_null() || value.is_null() {
        return CFixError::NullPointer;
    }
    let value = CStr::from_ptr(value).to_bytes();
    (*builder).push_mut(tag_param as u32, value);
    CFixError::OK
}

/// # Safety
///
/// The message_builder_t should not be NULL.
#[no_mangle]
pub unsafe extern "C" fn message_builder_push_int(
    builder: message_builder_t,
    tag_param: Tags,
    value: isize,
) -> CFixError {
    if builder.is_null() {
        return CFixError::NullPointer;
    }
    (*builder).push_mut(
        tag_param as u32,
        SerializedInt::from(value as u32).as_bytes(),
    );
    CFixError::OK
}

/// # Safety
///
/// The message_builder_t should not be NULL.
#[no_mangle]
pub unsafe extern "C" fn message_builder_push_field(
    builder: message_builder_t,
    tag_param: Tags,
    value: isize,
) -> CFixError {
    if builder.is_null() {
        return CFixError::NullPointer;
    }
    (*builder).push_mut(tag_param as u32, &[value as u8]);
    CFixError::OK
}

/// # Safety
///
/// The message_builder_t should not be NULL.
#[no_mangle]
pub unsafe extern "C" fn message_builder_push_current_time(
    builder: message_builder_t,
    tag_param: Tags,
) -> CFixError {
    if builder.is_null() {
        return CFixError::NullPointer;
    }
    (*builder).push_mut(
        tag_param as u32,
        forgefix::fix::encode::formatted_time().as_bytes(),
    );
    CFixError::OK
}

/// # Safety
///
/// Any pointers passed in should only be from the message_builder_new() function.
#[no_mangle]
pub unsafe extern "C" fn message_builder_free(builder: message_builder_t) {
    if !builder.is_null() {
        drop(Box::from_raw(builder));
    }
}

#[allow(non_camel_case_types)]
pub type session_settings_t = *mut SessionSettings; 

#[allow(non_camel_case_types)]
pub type session_settings_builder_t = *mut SessionSettingsBuilder; 

#[no_mangle]
pub extern "C" fn session_settings_builder_new() -> session_settings_builder_t {
    let settings_builder = SessionSettingsBuilder::new();
    Box::into_raw(Box::new(settings_builder))
}

/// # Safety
///
/// This function should be called with Utf-8 valid strings.
#[no_mangle]
pub unsafe extern "C" fn ssb_set_sender_comp_id(
    builder: session_settings_builder_t,
    sender_comp_id: *const c_char,
) -> CFixError {
    if builder.is_null() || sender_comp_id.is_null() {
        return CFixError::NullPointer;
    }
    let sender_comp_id = match CStr::from_ptr(sender_comp_id).to_str() {
        Ok(s) => s,
        Err(_) => return CFixError::BadString,
    };
    (*builder).set_sender_comp_id(sender_comp_id);
    CFixError::OK
}

/// # Safety
///
/// This function should be called with Utf-8 valid strings.
#[no_mangle]
pub unsafe extern "C" fn ssb_set_target_comp_id(
    builder: session_settings_builder_t,
    target_comp_id: *const c_char,
) -> CFixError {
    if builder.is_null() || target_comp_id.is_null() {
        return CFixError::NullPointer;
    }
    let target_comp_id = match CStr::from_ptr(target_comp_id).to_str() {
        Ok(s) => s,
        Err(_) => return CFixError::BadString,
    };
    (*builder).set_target_comp_id(target_comp_id);
    CFixError::OK
}

/// # Safety
///
/// This function should be called with Utf-8 valid strings.
#[no_mangle]
pub unsafe extern "C" fn ssb_set_socket_addr(
    builder: session_settings_builder_t,
    addr: *const c_char,
) -> CFixError {
    if builder.is_null() || addr.is_null() {
        return CFixError::NullPointer;
    }

    let addr = match CStr::from_ptr(addr)
        .to_str()
        .ok()
        .and_then(|s| s.parse::<SocketAddr>().ok())
    {
        Some(a) => a,
        None => return CFixError::BadString, 
    };
    (*builder).set_socket_addr(addr);
    CFixError::OK
}

/// # Safety
///
/// The pointers should not be NULL.
#[no_mangle]
pub unsafe extern "C" fn ssb_set_begin_string(
    builder: session_settings_builder_t,
    begin_string: *const c_char, 
) -> CFixError {
    if builder.is_null() || begin_string.is_null() {
        return CFixError::NullPointer;
    }
    let begin_string = match CStr::from_ptr(begin_string).to_str() {
        Ok(s) => s,
        Err(_) => return CFixError::BadString,
    };
    (*builder).set_begin_string(begin_string);
    CFixError::OK
}

/// # Safety
///
/// The pointers should not be NULL.
#[no_mangle]
pub unsafe extern "C" fn ssb_set_epoch(
    builder: session_settings_builder_t,
    epoch: *const c_char,
) -> CFixError {
    if builder.is_null() || epoch.is_null() {
        return CFixError::NullPointer;
    }
    let epoch = match CStr::from_ptr(epoch).to_str() {
        Ok(s) => s,
        Err(_) => return CFixError::BadString,
    };
    (*builder).set_epoch(epoch);
    CFixError::OK
}

/// # Safety
///
/// The pointers should not be NULL.
#[no_mangle]
pub unsafe extern "C" fn ssb_set_store_path(
    builder: session_settings_builder_t,
    store_path: *const c_char,
) -> CFixError {
    if builder.is_null() || store_path.is_null() {
        return CFixError::NullPointer;
    }
    let store_path = match CStr::from_ptr(store_path)
        .to_str()
        .map(std::path::PathBuf::from)
    {
        Ok(p) => p,
        Err(_) => return CFixError::BadString,
    };
    (*builder).set_store_path(store_path);
    CFixError::OK
}

/// # Safety
///
/// The pointers should not be NULL.
#[no_mangle]
pub unsafe extern "C" fn ssb_set_log_dir(
    builder: session_settings_builder_t,
    log_dir: *const c_char,
) -> CFixError {
    if builder.is_null() || log_dir.is_null() {
        return CFixError::NullPointer;
    }
    let log_dir = match CStr::from_ptr(log_dir)
        .to_str()
        .map(std::path::PathBuf::from)
    {
        Ok(p) => p, 
        Err(_) => return CFixError::BadString,
    };
    (*builder).set_log_dir(log_dir);
    CFixError::OK
}

/// # Safety
///
/// The pointers should not be NULL.
#[no_mangle]
pub unsafe extern "C" fn ssb_set_heartbeat_timeout(
    builder: session_settings_builder_t,
    heartbeat_timeout: c_ulong,
) -> CFixError {
    if builder.is_null() {
        return CFixError::NullPointer; 
    }
    let heartbeat_timeout = Duration::from_secs(heartbeat_timeout);
    (*builder).set_heartbeat_timeout(heartbeat_timeout);
    CFixError::OK
}

/// # Safety
///
/// The pointers should not be NULL and start_time_str should be a UTC time in format `HH:MM:SS`. 
#[no_mangle]
pub unsafe extern "C" fn ssb_set_start_time(
    builder: session_settings_builder_t,
    start_time_str: *const c_char,
) -> CFixError {
    if builder.is_null() | start_time_str.is_null() {
        return CFixError::NullPointer; 
    }

    let start_time_str = match CStr::from_ptr(start_time_str).to_str() {
        Ok(s) => s,
        Err(_) => return CFixError::BadString, 
    }; 

    let start_time = match chrono::naive::NaiveTime::parse_from_str(start_time_str, TIME_FORMAT) {
        Ok(t) => t,
        Err(_) => return CFixError::BadString,
    };

    (*builder).set_start_time(start_time); 
    CFixError::OK
}

#[no_mangle]
pub extern "C" fn ssb_set_reset_seq_num(
    builder: session_settings_builder_t,
    reset: c_int,
) -> CFixError {
    if builder.is_null() {
        return CFixError::NullPointer;
    }

    let reset = reset != 0; 
    unsafe { (*builder).set_reset_seq_num(reset); }
    CFixError::OK
}

#[no_mangle]
pub extern "C" fn ssb_set_reset_flag_on_initial_logon(
    builder: session_settings_builder_t,
    use_flag: c_int,
) -> CFixError {
    if builder.is_null() {
        return CFixError::NullPointer; 
    }

    let use_flag = use_flag != 0; 
    unsafe { (*builder).set_reset_flag_on_initial_logon(use_flag); }
    CFixError::OK
}

/// # Safety
///
/// The pointer should not be NULL.
#[no_mangle]
pub unsafe extern "C" fn ssb_build(builder: session_settings_builder_t) -> session_settings_t {
    if builder.is_null() {
        return std::ptr::null_mut();
    }
    let builder = Box::from_raw(builder);
    match builder.build() {
        Ok(settings) => Box::into_raw(Box::new(settings)),
        Err(_) => std::ptr::null_mut(),
    }
}

/// # Safety
///
/// Any pointers passed in should only be from the session_settings_builder_new() function.
#[no_mangle]
pub unsafe extern "C" fn session_settings_builder_free(builder: session_settings_builder_t) {
    if !builder.is_null() {
        drop(Box::from_raw(builder));
    }
}
