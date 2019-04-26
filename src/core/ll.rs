#![allow(bad_style)]

use super::{
    UserStatus,
    MessageType,
    ProxyType,
    SavedataType,
    LogLevel,
    Connection,
    FileControl,
    ConferenceType
};

use super::errors::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tox {
    _unused: [u8; 0],
}

#[link(name = "toxcore")]
extern "C" {
    pub fn tox_version_major() -> u32;
    pub fn tox_version_minor() -> u32;
    pub fn tox_version_patch() -> u32;
    pub fn tox_version_is_compatible(major: u32, minor: u32, patch: u32) -> bool;
    pub fn tox_public_key_size() -> u32;
    pub fn tox_secret_key_size() -> u32;
    pub fn tox_conference_uid_size() -> u32;
    pub fn tox_conference_id_size() -> u32;
    pub fn tox_nospam_size() -> u32;
    pub fn tox_address_size() -> u32;
    pub fn tox_max_name_length() -> u32;
    pub fn tox_max_status_message_length() -> u32;
    pub fn tox_max_friend_request_length() -> u32;
    pub fn tox_max_message_length() -> u32;
    pub fn tox_max_custom_packet_size() -> u32;
    pub fn tox_hash_length() -> u32;
    pub fn tox_file_id_length() -> u32;
    pub fn tox_max_filename_length() -> u32;
    pub fn tox_max_hostname_length() -> u32;
}

pub type tox_log_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        level: LogLevel,
        file: *const ::std::os::raw::c_char,
        line: u32,
        func: *const ::std::os::raw::c_char,
        message: *const ::std::os::raw::c_char,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tox_Options {
    pub ipv6_enabled: bool,
    pub udp_enabled: bool,
    pub local_discovery_enabled: bool,
    pub proxy_type: ProxyType,
    pub proxy_host: *const ::std::os::raw::c_char,
    pub proxy_port: u16,
    pub start_port: u16,
    pub end_port: u16,
    pub tcp_port: u16,
    pub hole_punching_enabled: bool,
    pub savedata_type: SavedataType,
    pub savedata_data: *const u8,
    pub savedata_length: usize,
    pub log_callback: tox_log_cb,
    pub log_user_data: *mut ::std::os::raw::c_void,
}

#[test]
fn bindgen_test_layout_Tox_Options() {
    assert_eq!(
        ::std::mem::size_of::<Tox_Options>(),
        64usize,
        concat!("Size of: ", stringify!(Tox_Options))
    );
    assert_eq!(
        ::std::mem::align_of::<Tox_Options>(),
        8usize,
        concat!("Alignment of ", stringify!(Tox_Options))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).ipv6_enabled as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(ipv6_enabled)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).udp_enabled as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(udp_enabled)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Tox_Options>())).local_discovery_enabled as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(local_discovery_enabled)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).proxy_type as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(proxy_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).proxy_host as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(proxy_host)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).proxy_port as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(proxy_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).start_port as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(start_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).end_port as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(end_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).tcp_port as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(tcp_port)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Tox_Options>())).hole_punching_enabled as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(hole_punching_enabled)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).savedata_type as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(savedata_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).savedata_data as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(savedata_data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).savedata_length as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(savedata_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).log_callback as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(log_callback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Tox_Options>())).log_user_data as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(Tox_Options),
            "::",
            stringify!(log_user_data)
        )
    );
}

extern "C" {
    pub fn tox_options_get_ipv6_enabled(options: *const Tox_Options) -> bool;
    pub fn tox_options_set_ipv6_enabled(options: *mut Tox_Options, ipv6_enabled: bool);
    pub fn tox_options_get_udp_enabled(options: *const Tox_Options) -> bool;
    pub fn tox_options_set_udp_enabled(options: *mut Tox_Options, udp_enabled: bool);
    pub fn tox_options_get_local_discovery_enabled(options: *const Tox_Options) -> bool;
    pub fn tox_options_set_local_discovery_enabled(
        options: *mut Tox_Options,
        local_discovery_enabled: bool,
    );
    pub fn tox_options_get_proxy_type(options: *const Tox_Options) -> ProxyType;
    pub fn tox_options_set_proxy_type(options: *mut Tox_Options, type_: ProxyType);
    pub fn tox_options_get_proxy_host(options: *const Tox_Options)
        -> *const ::std::os::raw::c_char;
    pub fn tox_options_set_proxy_host(
        options: *mut Tox_Options,
        host: *const ::std::os::raw::c_char,
    );
    pub fn tox_options_get_proxy_port(options: *const Tox_Options) -> u16;
    pub fn tox_options_set_proxy_port(options: *mut Tox_Options, port: u16);
    pub fn tox_options_get_start_port(options: *const Tox_Options) -> u16;
    pub fn tox_options_set_start_port(options: *mut Tox_Options, start_port: u16);
    pub fn tox_options_get_end_port(options: *const Tox_Options) -> u16;
    pub fn tox_options_set_end_port(options: *mut Tox_Options, end_port: u16);
    pub fn tox_options_get_tcp_port(options: *const Tox_Options) -> u16;
    pub fn tox_options_set_tcp_port(options: *mut Tox_Options, tcp_port: u16);
    pub fn tox_options_get_hole_punching_enabled(options: *const Tox_Options) -> bool;
    pub fn tox_options_set_hole_punching_enabled(
        options: *mut Tox_Options,
        hole_punching_enabled: bool,
    );
    pub fn tox_options_get_savedata_type(options: *const Tox_Options) -> SavedataType;
    pub fn tox_options_set_savedata_type(options: *mut Tox_Options, type_: SavedataType);
    pub fn tox_options_get_savedata_data(options: *const Tox_Options) -> *const u8;
    pub fn tox_options_set_savedata_data(options: *mut Tox_Options, data: *const u8, length: usize);
    pub fn tox_options_get_savedata_length(options: *const Tox_Options) -> usize;
    pub fn tox_options_set_savedata_length(options: *mut Tox_Options, length: usize);
    pub fn tox_options_get_log_callback(options: *const Tox_Options) -> tox_log_cb;
    pub fn tox_options_set_log_callback(options: *mut Tox_Options, callback: tox_log_cb);
    pub fn tox_options_get_log_user_data(
        options: *const Tox_Options,
    ) -> *mut ::std::os::raw::c_void;
    pub fn tox_options_set_log_user_data(
        options: *mut Tox_Options,
        user_data: *mut ::std::os::raw::c_void,
    );
    pub fn tox_options_default(options: *mut Tox_Options);
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_OPTIONS_NEW {
    TOX_ERR_OPTIONS_NEW_OK = 0,
    TOX_ERR_OPTIONS_NEW_MALLOC = 1,
}

extern "C" {
    pub fn tox_options_new(error: *mut TOX_ERR_OPTIONS_NEW) -> *mut Tox_Options;
    pub fn tox_options_free(options: *mut Tox_Options);
}

extern "C" {
    pub fn tox_new(options: *const Tox_Options, error: *mut InitError) -> *mut Tox;
    pub fn tox_kill(tox: *mut Tox);
    pub fn tox_get_savedata_size(tox: *const Tox) -> usize;
    pub fn tox_get_savedata(tox: *const Tox, savedata: *mut u8);
}
extern "C" {
    pub fn tox_bootstrap(
        tox: *mut Tox,
        host: *const ::std::os::raw::c_char,
        port: u16,
        public_key: *const u8,
        error: *mut BootstrapError,
    ) -> bool;
    pub fn tox_add_tcp_relay(
        tox: *mut Tox,
        host: *const ::std::os::raw::c_char,
        port: u16,
        public_key: *const u8,
        error: *mut BootstrapError,
    ) -> bool;
}

extern "C" {
    pub fn tox_self_get_connection_status(tox: *const Tox) -> Connection;
}

pub type tox_self_connection_status_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        connection_status: Connection,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_self_connection_status(
        tox: *mut Tox,
        callback: tox_self_connection_status_cb,
    );
    pub fn tox_iteration_interval(tox: *const Tox) -> u32;
    pub fn tox_iterate(tox: *mut Tox, user_data: *mut ::std::os::raw::c_void);
    pub fn tox_self_get_address(tox: *const Tox, address: *mut u8);
    pub fn tox_self_set_nospam(tox: *mut Tox, nospam: u32);
    pub fn tox_self_get_nospam(tox: *const Tox) -> u32;
    pub fn tox_self_get_public_key(tox: *const Tox, public_key: *mut u8);
    pub fn tox_self_get_secret_key(tox: *const Tox, secret_key: *mut u8);
}

extern "C" {
    pub fn tox_self_set_name(
        tox: *mut Tox,
        name: *const u8,
        length: usize,
        error: *mut SetInfoError,
    ) -> bool;
    pub fn tox_self_get_name_size(tox: *const Tox) -> usize;
    pub fn tox_self_get_name(tox: *const Tox, name: *mut u8);
    pub fn tox_self_set_status_message(
        tox: *mut Tox,
        status_message: *const u8,
        length: usize,
        error: *mut SetInfoError,
    ) -> bool;
    pub fn tox_self_get_status_message_size(tox: *const Tox) -> usize;
    pub fn tox_self_get_status_message(tox: *const Tox, status_message: *mut u8);
    pub fn tox_self_set_status(tox: *mut Tox, status: UserStatus);
    pub fn tox_self_get_status(tox: *const Tox) -> UserStatus;
}

extern "C" {
    pub fn tox_friend_add(
        tox: *mut Tox,
        address: *const u8,
        message: *const u8,
        length: usize,
        error: *mut FriendAddError,
    ) -> u32;
    pub fn tox_friend_add_norequest(
        tox: *mut Tox,
        public_key: *const u8,
        error: *mut FriendAddError,
    ) -> u32;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_FRIEND_DELETE {
    TOX_ERR_FRIEND_DELETE_OK = 0,
    TOX_ERR_FRIEND_DELETE_FRIEND_NOT_FOUND = 1,
}

extern "C" {
    pub fn tox_friend_delete(
        tox: *mut Tox,
        friend_number: u32,
        error: *mut TOX_ERR_FRIEND_DELETE,
    ) -> bool;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_FRIEND_BY_PUBLIC_KEY {
    TOX_ERR_FRIEND_BY_PUBLIC_KEY_OK = 0,
    TOX_ERR_FRIEND_BY_PUBLIC_KEY_NULL = 1,
    TOX_ERR_FRIEND_BY_PUBLIC_KEY_NOT_FOUND = 2,
}

extern "C" {
    pub fn tox_friend_by_public_key(
        tox: *const Tox,
        public_key: *const u8,
        error: *mut TOX_ERR_FRIEND_BY_PUBLIC_KEY,
    ) -> u32;
    pub fn tox_friend_exists(tox: *const Tox, friend_number: u32) -> bool;
    pub fn tox_self_get_friend_list_size(tox: *const Tox) -> usize;
    pub fn tox_self_get_friend_list(tox: *const Tox, friend_list: *mut u32);
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_FRIEND_GET_PUBLIC_KEY {
    TOX_ERR_FRIEND_GET_PUBLIC_KEY_OK = 0,
    TOX_ERR_FRIEND_GET_PUBLIC_KEY_FRIEND_NOT_FOUND = 1,
}

extern "C" {
    pub fn tox_friend_get_public_key(
        tox: *const Tox,
        friend_number: u32,
        public_key: *mut u8,
        error: *mut TOX_ERR_FRIEND_GET_PUBLIC_KEY,
    ) -> bool;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_FRIEND_GET_LAST_ONLINE {
    TOX_ERR_FRIEND_GET_LAST_ONLINE_OK = 0,
    TOX_ERR_FRIEND_GET_LAST_ONLINE_FRIEND_NOT_FOUND = 1,
}

extern "C" {
    pub fn tox_friend_get_last_online(
        tox: *const Tox,
        friend_number: u32,
        error: *mut TOX_ERR_FRIEND_GET_LAST_ONLINE,
    ) -> u64;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_FRIEND_QUERY {
    TOX_ERR_FRIEND_QUERY_OK = 0,
    TOX_ERR_FRIEND_QUERY_NULL = 1,
    TOX_ERR_FRIEND_QUERY_FRIEND_NOT_FOUND = 2,
}

extern "C" {
    pub fn tox_friend_get_name_size(
        tox: *const Tox,
        friend_number: u32,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> usize;
    pub fn tox_friend_get_name(
        tox: *const Tox,
        friend_number: u32,
        name: *mut u8,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> bool;
}

pub type tox_friend_name_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        name: *const u8,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_friend_name(tox: *mut Tox, callback: tox_friend_name_cb);
    pub fn tox_friend_get_status_message_size(
        tox: *const Tox,
        friend_number: u32,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> usize;
    pub fn tox_friend_get_status_message(
        tox: *const Tox,
        friend_number: u32,
        status_message: *mut u8,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> bool;
}

pub type tox_friend_status_message_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        message: *const u8,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_friend_status_message(
        tox: *mut Tox,
        callback: tox_friend_status_message_cb,
    );
    pub fn tox_friend_get_status(
        tox: *const Tox,
        friend_number: u32,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> UserStatus;
}

pub type tox_friend_status_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        status: UserStatus,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_friend_status(tox: *mut Tox, callback: tox_friend_status_cb);
    pub fn tox_friend_get_connection_status(
        tox: *const Tox,
        friend_number: u32,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> Connection;
}

pub type tox_friend_connection_status_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        connection_status: Connection,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_friend_connection_status(
        tox: *mut Tox,
        callback: tox_friend_connection_status_cb,
    );
    pub fn tox_friend_get_typing(
        tox: *const Tox,
        friend_number: u32,
        error: *mut TOX_ERR_FRIEND_QUERY,
    ) -> bool;
}

pub type tox_friend_typing_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        is_typing: bool,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_friend_typing(tox: *mut Tox, callback: tox_friend_typing_cb);
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_SET_TYPING {
    TOX_ERR_SET_TYPING_OK = 0,
    TOX_ERR_SET_TYPING_FRIEND_NOT_FOUND = 1,
}

extern "C" {
    pub fn tox_self_set_typing(
        tox: *mut Tox,
        friend_number: u32,
        typing: bool,
        error: *mut TOX_ERR_SET_TYPING,
    ) -> bool;
}

extern "C" {
    pub fn tox_friend_send_message(
        tox: *mut Tox,
        friend_number: u32,
        type_: MessageType,
        message: *const u8,
        length: usize,
        error: *mut FriendSendMessageError,
    ) -> u32;
}

pub type tox_friend_read_receipt_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        message_id: u32,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_friend_read_receipt(tox: *mut Tox, callback: tox_friend_read_receipt_cb);
}

pub type tox_friend_request_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        public_key: *const u8,
        message: *const u8,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_friend_request(tox: *mut Tox, callback: tox_friend_request_cb);
}

pub type tox_friend_message_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        type_: MessageType,
        message: *const u8,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_friend_message(tox: *mut Tox, callback: tox_friend_message_cb);
    pub fn tox_hash(hash: *mut u8, data: *const u8, length: usize) -> bool;
}

extern "C" {
    pub fn tox_file_control(
        tox: *mut Tox,
        friend_number: u32,
        file_number: u32,
        control: FileControl,
        error: *mut FileControlError,
    ) -> bool;
}

pub type tox_file_recv_control_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        file_number: u32,
        control: FileControl,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_file_recv_control(tox: *mut Tox, callback: tox_file_recv_control_cb);
}

extern "C" {
    pub fn tox_file_seek(
        tox: *mut Tox,
        friend_number: u32,
        file_number: u32,
        position: u64,
        error: *mut FileSeekError,
    ) -> bool;
}

extern "C" {
    pub fn tox_file_get_file_id(
        tox: *const Tox,
        friend_number: u32,
        file_number: u32,
        file_id: *mut u8,
        error: *mut FileGetError,
    ) -> bool;
}

extern "C" {
    pub fn tox_file_send(
        tox: *mut Tox,
        friend_number: u32,
        kind: u32,
        file_size: u64,
        file_id: *const u8,
        filename: *const u8,
        filename_length: usize,
        error: *mut FileSendError,
    ) -> u32;
}

extern "C" {
    pub fn tox_file_send_chunk(
        tox: *mut Tox,
        friend_number: u32,
        file_number: u32,
        position: u64,
        data: *const u8,
        length: usize,
        error: *mut FileSendChunkError,
    ) -> bool;
}

pub type tox_file_chunk_request_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        file_number: u32,
        position: u64,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_file_chunk_request(tox: *mut Tox, callback: tox_file_chunk_request_cb);
}

pub type tox_file_recv_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        file_number: u32,
        kind: u32,
        file_size: u64,
        filename: *const u8,
        filename_length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_file_recv(tox: *mut Tox, callback: tox_file_recv_cb);
}

pub type tox_file_recv_chunk_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        file_number: u32,
        position: u64,
        data: *const u8,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_file_recv_chunk(tox: *mut Tox, callback: tox_file_recv_chunk_cb);
}

pub type tox_conference_invite_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        type_: ConferenceType,
        cookie: *const u8,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_conference_invite(tox: *mut Tox, callback: tox_conference_invite_cb);
}

pub type tox_conference_connected_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        conference_number: u32,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_conference_connected(tox: *mut Tox, callback: tox_conference_connected_cb);
}

pub type tox_conference_message_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        conference_number: u32,
        peer_number: u32,
        type_: MessageType,
        message: *const u8,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_conference_message(tox: *mut Tox, callback: tox_conference_message_cb);
}

pub type tox_conference_title_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        conference_number: u32,
        peer_number: u32,
        title: *const u8,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_conference_title(tox: *mut Tox, callback: tox_conference_title_cb);
}

pub type tox_conference_peer_name_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        conference_number: u32,
        peer_number: u32,
        name: *const u8,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_conference_peer_name(tox: *mut Tox, callback: tox_conference_peer_name_cb);
}

pub type tox_conference_peer_list_changed_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        conference_number: u32,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_conference_peer_list_changed(
        tox: *mut Tox,
        callback: tox_conference_peer_list_changed_cb,
    );
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_CONFERENCE_NEW {
    TOX_ERR_CONFERENCE_NEW_OK = 0,
    TOX_ERR_CONFERENCE_NEW_INIT = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_CONFERENCE_DELETE {
    TOX_ERR_CONFERENCE_DELETE_OK = 0,
    TOX_ERR_CONFERENCE_DELETE_CONFERENCE_NOT_FOUND = 1,
}

extern "C" {
    pub fn tox_conference_new(tox: *mut Tox, error: *mut TOX_ERR_CONFERENCE_NEW) -> u32;
    pub fn tox_conference_delete(
        tox: *mut Tox,
        conference_number: u32,
        error: *mut TOX_ERR_CONFERENCE_DELETE,
    ) -> bool;
    pub fn tox_conference_peer_count(
        tox: *const Tox,
        conference_number: u32,
        error: *mut ConferencePeerQueryError,
    ) -> u32;
    pub fn tox_conference_peer_get_name_size(
        tox: *const Tox,
        conference_number: u32,
        peer_number: u32,
        error: *mut ConferencePeerQueryError,
    ) -> usize;
    pub fn tox_conference_peer_get_name(
        tox: *const Tox,
        conference_number: u32,
        peer_number: u32,
        name: *mut u8,
        error: *mut ConferencePeerQueryError,
    ) -> bool;
    pub fn tox_conference_peer_get_public_key(
        tox: *const Tox,
        conference_number: u32,
        peer_number: u32,
        public_key: *mut u8,
        error: *mut ConferencePeerQueryError,
    ) -> bool;
    pub fn tox_conference_peer_number_is_ours(
        tox: *const Tox,
        conference_number: u32,
        peer_number: u32,
        error: *mut ConferencePeerQueryError,
    ) -> bool;
    pub fn tox_conference_offline_peer_count(
        tox: *const Tox,
        conference_number: u32,
        error: *mut ConferencePeerQueryError,
    ) -> u32;
    pub fn tox_conference_offline_peer_get_name_size(
        tox: *const Tox,
        conference_number: u32,
        offline_peer_number: u32,
        error: *mut ConferencePeerQueryError,
    ) -> usize;
    pub fn tox_conference_offline_peer_get_name(
        tox: *const Tox,
        conference_number: u32,
        offline_peer_number: u32,
        name: *mut u8,
        error: *mut ConferencePeerQueryError,
    ) -> bool;
    pub fn tox_conference_offline_peer_get_public_key(
        tox: *const Tox,
        conference_number: u32,
        offline_peer_number: u32,
        public_key: *mut u8,
        error: *mut ConferencePeerQueryError,
    ) -> bool;
    pub fn tox_conference_offline_peer_get_last_active(
        tox: *const Tox,
        conference_number: u32,
        offline_peer_number: u32,
        error: *mut ConferencePeerQueryError,
    ) -> u64;
    pub fn tox_conference_invite(
        tox: *mut Tox,
        friend_number: u32,
        conference_number: u32,
        error: *mut ConferenceInviteError,
    ) -> bool;
    pub fn tox_conference_join(
        tox: *mut Tox,
        friend_number: u32,
        cookie: *const u8,
        length: usize,
        error: *mut ConferenceJoinError,
    ) -> u32;
    pub fn tox_conference_send_message(
        tox: *mut Tox,
        conference_number: u32,
        type_: MessageType,
        message: *const u8,
        length: usize,
        error: *mut ConferenceSendError,
    ) -> bool;
    pub fn tox_conference_get_title_size(
        tox: *const Tox,
        conference_number: u32,
        error: *mut ConferenceTitleError,
    ) -> usize;
    pub fn tox_conference_get_title(
        tox: *const Tox,
        conference_number: u32,
        title: *mut u8,
        error: *mut ConferenceTitleError,
    ) -> bool;
    pub fn tox_conference_set_title(
        tox: *mut Tox,
        conference_number: u32,
        title: *const u8,
        length: usize,
        error: *mut ConferenceTitleError,
    ) -> bool;
    pub fn tox_conference_get_chatlist_size(tox: *const Tox) -> usize;
    pub fn tox_conference_get_chatlist(tox: *const Tox, chatlist: *mut u32);
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_CONFERENCE_GET_TYPE {
    TOX_ERR_CONFERENCE_GET_TYPE_OK = 0,
    TOX_ERR_CONFERENCE_GET_TYPE_CONFERENCE_NOT_FOUND = 1,
}

extern "C" {
    pub fn tox_conference_get_type(
        tox: *const Tox,
        conference_number: u32,
        error: *mut TOX_ERR_CONFERENCE_GET_TYPE,
    ) -> ConferenceType;
    pub fn tox_conference_get_id(
        tox: *const Tox,
        conference_number: u32,
        id: *mut u8
    ) -> bool;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_CONFERENCE_BY_ID {
    TOX_ERR_CONFERENCE_BY_ID_OK = 0,
    TOX_ERR_CONFERENCE_BY_ID_NULL = 1,
    TOX_ERR_CONFERENCE_BY_ID_NOT_FOUND = 2,
}

extern "C" {
    pub fn tox_conference_by_id(
        tox: *const Tox,
        id: *const u8,
        error: *mut TOX_ERR_CONFERENCE_BY_ID,
    ) -> u32;
    pub fn tox_conference_get_uid(tox: *const Tox, conference_number: u32, uid: *mut u8) -> bool;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_CONFERENCE_BY_UID {
    TOX_ERR_CONFERENCE_BY_UID_OK = 0,
    TOX_ERR_CONFERENCE_BY_UID_NULL = 1,
    TOX_ERR_CONFERENCE_BY_UID_NOT_FOUND = 2,
}

extern "C" {
    pub fn tox_conference_by_uid(
        tox: *const Tox,
        uid: *const u8,
        error: *mut TOX_ERR_CONFERENCE_BY_UID,
    ) -> u32;
}

extern "C" {
    pub fn tox_friend_send_lossy_packet(
        tox: *mut Tox,
        friend_number: u32,
        data: *const u8,
        length: usize,
        error: *mut FriendCustomPacketError,
    ) -> bool;
    pub fn tox_friend_send_lossless_packet(
        tox: *mut Tox,
        friend_number: u32,
        data: *const u8,
        length: usize,
        error: *mut FriendCustomPacketError,
    ) -> bool;
}

pub type tox_friend_lossy_packet_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        data: *const u8,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_friend_lossy_packet(tox: *mut Tox, callback: tox_friend_lossy_packet_cb);
}

pub type tox_friend_lossless_packet_cb = ::std::option::Option<
    unsafe extern "C" fn(
        tox: *mut Tox,
        friend_number: u32,
        data: *const u8,
        length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;

extern "C" {
    pub fn tox_callback_friend_lossless_packet(
        tox: *mut Tox,
        callback: tox_friend_lossless_packet_cb,
    );
}

extern "C" {
    pub fn tox_self_get_dht_id(tox: *const Tox, dht_id: *mut u8);
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TOX_ERR_GET_PORT {
    TOX_ERR_GET_PORT_OK = 0,
    TOX_ERR_GET_PORT_NOT_BOUND = 1,
}

extern "C" {
    pub fn tox_self_get_udp_port(tox: *const Tox, error: *mut TOX_ERR_GET_PORT) -> u16;
}

extern "C" {
    pub fn tox_self_get_tcp_port(tox: *const Tox, error: *mut TOX_ERR_GET_PORT) -> u16;
}
