#![allow(improper_ctypes, non_camel_case_types)]

use super::errors;
use super::{Connection, UserStatus, MessageType, FileControl, };

use libc::{c_int, c_uint, c_void};

pub enum Struct_Tox { }
pub type Tox = Struct_Tox;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Tox_Options {
    pub ipv6_enabled: u8,
    pub udp_enabled: u8,
    pub proxy_type: super::ProxyType,
    pub proxy_host: *const ::libc::c_char,
    pub proxy_port: u16,
    pub start_port: u16,
    pub end_port: u16,
    pub savedata_type: super::SavedataType,
    pub savedata_data: *const u8,
    pub savedata_length: usize,
}

impl ::std::default::Default for Tox_Options {
    fn default() -> Tox_Options {
        unsafe {
            let mut opts = ::std::mem::uninitialized();
            tox_options_default(&mut opts);
            opts
        }
    }
}

pub type Enum_TOX_ERR_OPTIONS_NEW = ::libc::c_uint;
pub const TOX_ERR_OPTIONS_NEW_OK: ::libc::c_uint = 0;
pub const TOX_ERR_OPTIONS_NEW_MALLOC: ::libc::c_uint = 1;
pub type TOX_ERR_OPTIONS_NEW = Enum_TOX_ERR_OPTIONS_NEW;

pub type tox_self_connection_status_cb =
    extern "C" fn(tox: *mut Tox, connection_status: Connection,
                  user_data: *mut ::libc::c_void) -> ();


pub type Enum_TOX_ERR_FRIEND_DELETE = ::libc::c_uint;
pub const TOX_ERR_FRIEND_DELETE_OK: ::libc::c_uint = 0;
pub const TOX_ERR_FRIEND_DELETE_FRIEND_NOT_FOUND: ::libc::c_uint = 1;
pub type TOX_ERR_FRIEND_DELETE = Enum_TOX_ERR_FRIEND_DELETE;


pub type Enum_TOX_ERR_FRIEND_BY_PUBLIC_KEY = ::libc::c_uint;
pub const TOX_ERR_FRIEND_BY_PUBLIC_KEY_OK: ::libc::c_uint = 0;
pub const TOX_ERR_FRIEND_BY_PUBLIC_KEY_NULL: ::libc::c_uint = 1;
pub const TOX_ERR_FRIEND_BY_PUBLIC_KEY_NOT_FOUND: ::libc::c_uint = 2;
pub type TOX_ERR_FRIEND_BY_PUBLIC_KEY = Enum_TOX_ERR_FRIEND_BY_PUBLIC_KEY;

pub type Enum_TOX_ERR_FRIEND_GET_PUBLIC_KEY = ::libc::c_uint;
pub const TOX_ERR_FRIEND_GET_PUBLIC_KEY_OK: ::libc::c_uint = 0;
pub const TOX_ERR_FRIEND_GET_PUBLIC_KEY_FRIEND_NOT_FOUND: ::libc::c_uint = 1;
pub type TOX_ERR_FRIEND_GET_PUBLIC_KEY = Enum_TOX_ERR_FRIEND_GET_PUBLIC_KEY;

pub type Enum_TOX_ERR_FRIEND_GET_LAST_ONLINE = ::libc::c_uint;
pub const TOX_ERR_FRIEND_GET_LAST_ONLINE_OK: ::libc::c_uint = 0;
pub const TOX_ERR_FRIEND_GET_LAST_ONLINE_FRIEND_NOT_FOUND: ::libc::c_uint = 1;
pub type TOX_ERR_FRIEND_GET_LAST_ONLINE = Enum_TOX_ERR_FRIEND_GET_LAST_ONLINE;

pub type Enum_TOX_ERR_FRIEND_QUERY = ::libc::c_uint;
pub const TOX_ERR_FRIEND_QUERY_OK: ::libc::c_uint = 0;
pub const TOX_ERR_FRIEND_QUERY_NULL: ::libc::c_uint = 1;
pub const TOX_ERR_FRIEND_QUERY_FRIEND_NOT_FOUND: ::libc::c_uint = 2;
pub type TOX_ERR_FRIEND_QUERY = Enum_TOX_ERR_FRIEND_QUERY;

pub type tox_friend_name_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  name: *const u8, length: usize,
                  user_data: *mut ::libc::c_void) -> ();
pub type tox_friend_status_message_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  message: *const u8, length: usize,
                  user_data: *mut ::libc::c_void) -> ();
pub type tox_friend_status_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  status: UserStatus, user_data: *mut ::libc::c_void)
        -> ();
pub type tox_friend_connection_status_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  connection_status: Connection,
                  user_data: *mut ::libc::c_void) -> ();
pub type tox_friend_typing_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32, is_typing: u8,
                  user_data: *mut ::libc::c_void) -> ();

pub type Enum_TOX_ERR_SET_TYPING = ::libc::c_uint;
pub const TOX_ERR_SET_TYPING_OK: ::libc::c_uint = 0;
pub const TOX_ERR_SET_TYPING_FRIEND_NOT_FOUND: ::libc::c_uint = 1;
pub type TOX_ERR_SET_TYPING = Enum_TOX_ERR_SET_TYPING;


pub type tox_friend_read_receipt_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  message_id: u32, user_data: *mut ::libc::c_void) -> ();
pub type tox_friend_request_cb =
    extern "C" fn(tox: *mut Tox, public_key: *const u8,
                  message: *const u8, length: usize,
                  user_data: *mut ::libc::c_void) -> ();
pub type tox_friend_message_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  _type: MessageType, message: *const u8,
                  length: usize, user_data: *mut ::libc::c_void) -> ();


pub type tox_file_recv_control_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  file_number: u32, control: FileControl,
                  user_data: *mut ::libc::c_void) -> ();



pub type tox_file_chunk_request_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  file_number: u32, position: u64, length: usize,
                  user_data: *mut ::libc::c_void) -> ();

pub type tox_file_recv_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  file_number: u32, kind: u32, file_size: u64,
                  filename: *const u8, filename_length: usize,
                  user_data: *mut ::libc::c_void) -> ();

pub type tox_file_recv_chunk_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  file_number: u32, position: u64,
                  data: *const u8, length: usize,
                  user_data: *mut ::libc::c_void) -> ();


pub type tox_friend_lossy_packet_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  data: *const u8, length: usize,
                  user_data: *mut ::libc::c_void) -> ();
pub type tox_friend_lossless_packet_cb =
    extern "C" fn(tox: *mut Tox, friend_number: u32,
                  data: *const u8, length: usize,
                  user_data: *mut ::libc::c_void) -> ();

pub type Enum_TOX_ERR_GET_PORT = ::libc::c_uint;
pub const TOX_ERR_GET_PORT_OK: ::libc::c_uint = 0;
pub const TOX_ERR_GET_PORT_NOT_BOUND: ::libc::c_uint = 1;
pub type TOX_ERR_GET_PORT = Enum_TOX_ERR_GET_PORT;

pub type Enum_Unnamed1 = ::libc::c_uint;
pub const TOX_GROUPCHAT_TYPE_TEXT: ::libc::c_uint = 0;
pub const TOX_GROUPCHAT_TYPE_AV: ::libc::c_uint = 1;
pub type Enum_Unnamed2 = ::libc::c_uint;

pub const TOX_CHAT_CHANGE_PEER_ADD: ::libc::c_uint = 0;
pub const TOX_CHAT_CHANGE_PEER_DEL: ::libc::c_uint = 1;
pub const TOX_CHAT_CHANGE_PEER_NAME: ::libc::c_uint = 2;
pub type TOX_CHAT_CHANGE = Enum_Unnamed2;

#[link(name = "toxcore")]
extern "C" {
    pub fn tox_version_major() -> u32;
    pub fn tox_version_minor() -> u32;
    pub fn tox_version_patch() -> u32;
    pub fn tox_version_is_compatible(major: u32, minor: u32,
                                     patch: u32) -> u8;
    pub fn tox_options_default(options: *mut Tox_Options) -> ();
    pub fn tox_options_new(error: *mut TOX_ERR_OPTIONS_NEW)
     -> *mut Tox_Options;
    pub fn tox_options_free(options: *mut Tox_Options) -> ();
    pub fn tox_new(options: *const Tox_Options, error: *mut errors::InitError) -> *mut Tox;
    pub fn tox_kill(tox: *mut Tox) -> ();
    pub fn tox_get_savedata_size(tox: *const Tox) -> usize;
    pub fn tox_get_savedata(tox: *const Tox, data: *mut u8) -> ();
    pub fn tox_bootstrap(tox: *mut Tox, host: *const ::libc::c_char,
                         port: u16, public_key: *const u8,
                         error: *mut errors::BootstrapError) -> u8;
    pub fn tox_add_tcp_relay(tox: *mut Tox, host: *const ::libc::c_char,
                             port: u16, public_key: *const u8,
                             error: *mut errors::BootstrapError) -> u8;
    pub fn tox_self_get_connection_status(tox: *const Tox) -> Connection;
    pub fn tox_callback_self_connection_status(tox: *mut Tox,
                                               function: tox_self_connection_status_cb,
                                               user_data: *mut ::libc::c_void)
     -> ();
    pub fn tox_iteration_interval(tox: *const Tox) -> u32;
    pub fn tox_iterate(tox: *mut Tox) -> ();
    pub fn tox_self_get_address(tox: *const Tox, address: *mut u8) -> ();
    pub fn tox_self_set_nospam(tox: *mut Tox, nospam: u32) -> ();
    pub fn tox_self_get_nospam(tox: *const Tox) -> u32;
    pub fn tox_self_get_public_key(tox: *const Tox, public_key: *mut u8)
     -> ();
    pub fn tox_self_get_secret_key(tox: *const Tox, secret_key: *mut u8)
     -> ();
    pub fn tox_self_set_name(tox: *mut Tox, name: *const u8,
                             length: usize, error: *mut errors::SetInfoError)
     -> u8;
    pub fn tox_self_get_name_size(tox: *const Tox) -> usize;
    pub fn tox_self_get_name(tox: *const Tox, name: *mut u8) -> ();
    pub fn tox_self_set_status_message(tox: *mut Tox, status: *const u8,
                                       length: usize,
                                       error: *mut errors::SetInfoError) -> u8;
    pub fn tox_self_get_status_message_size(tox: *const Tox) -> usize;
    pub fn tox_self_get_status_message(tox: *const Tox, status: *mut u8)
     -> ();
    pub fn tox_self_set_status(tox: *mut Tox, user_status: UserStatus)
     -> ();
    pub fn tox_self_get_status(tox: *const Tox) -> UserStatus;
    pub fn tox_friend_add(tox: *mut Tox, address: *const u8,
                          message: *const u8, length: usize,
                          error: *mut errors::FriendAddError) -> u32;
    pub fn tox_friend_add_norequest(tox: *mut Tox, public_key: *const u8,
                                    error: *mut errors::FriendAddError)
     -> u32;
    pub fn tox_friend_delete(tox: *mut Tox, friend_number: u32,
                             error: *mut TOX_ERR_FRIEND_DELETE) -> u8;
    pub fn tox_friend_by_public_key(tox: *const Tox,
                                    public_key: *const u8,
                                    error: *mut TOX_ERR_FRIEND_BY_PUBLIC_KEY)
     -> u32;
    pub fn tox_friend_get_public_key(tox: *const Tox, friend_number: u32,
                                     public_key: *mut u8,
                                     error:
                                         *mut TOX_ERR_FRIEND_GET_PUBLIC_KEY)
     -> u8;
    pub fn tox_friend_exists(tox: *const Tox, friend_number: u32) -> u8;
    pub fn tox_friend_get_last_online(tox: *const Tox,
                                      friend_number: u32,
                                      error:
                                          *mut TOX_ERR_FRIEND_GET_LAST_ONLINE)
     -> u64;
    pub fn tox_self_get_friend_list_size(tox: *const Tox) -> usize;
    pub fn tox_self_get_friend_list(tox: *const Tox, list: *mut u32)
     -> ();
    pub fn tox_friend_get_name_size(tox: *const Tox, friend_number: u32,
                                    error: *mut TOX_ERR_FRIEND_QUERY)
     -> usize;
    pub fn tox_friend_get_name(tox: *const Tox, friend_number: u32,
                               name: *mut u8,
                               error: *mut TOX_ERR_FRIEND_QUERY) -> u8;
    pub fn tox_callback_friend_name(tox: *mut Tox,
                                    function:
                                        *mut tox_friend_name_cb,
                                    user_data: *mut ::libc::c_void) -> ();
    pub fn tox_friend_get_status_message_size(tox: *const Tox,
                                              friend_number: u32,
                                              error:
                                                  *mut TOX_ERR_FRIEND_QUERY)
     -> usize;
    pub fn tox_friend_get_status_message(tox: *const Tox,
                                         friend_number: u32,
                                         message: *mut u8,
                                         error: *mut TOX_ERR_FRIEND_QUERY)
     -> u8;
    pub fn tox_callback_friend_status_message(tox: *mut Tox,
                                              function: *mut tox_friend_status_message_cb,
                                              user_data: *mut ::libc::c_void)
     -> ();
    pub fn tox_friend_get_status(tox: *const Tox, friend_number: u32,
                                 error: *mut TOX_ERR_FRIEND_QUERY)
     -> UserStatus;
    pub fn tox_callback_friend_status(tox: *mut Tox,
                                      function: *mut tox_friend_status_message_cb,
                                      user_data: *mut ::libc::c_void) -> ();
    pub fn tox_friend_get_connection_status(tox: *const Tox,
                                            friend_number: u32,
                                            error: *mut TOX_ERR_FRIEND_QUERY)
     -> Connection;
    pub fn tox_callback_friend_connection_status(tox: *mut Tox,
                                                 function: *mut tox_friend_connection_status_cb,
                                                 user_data:
                                                     *mut ::libc::c_void)
     -> ();
    pub fn tox_friend_get_typing(tox: *const Tox, friend_number: u32,
                                 error: *mut TOX_ERR_FRIEND_QUERY) -> u8;
    pub fn tox_callback_friend_typing(tox: *mut Tox,
                                      function: *mut tox_friend_typing_cb,
                                      user_data: *mut ::libc::c_void) -> ();
    pub fn tox_self_set_typing(tox: *mut Tox, friend_number: u32,
                               is_typing: u8, error: *mut TOX_ERR_SET_TYPING)
     -> u8;
    pub fn tox_friend_send_message(tox: *mut Tox, friend_number: u32,
                                   _type: MessageType,
                                   message: *const u8, length: usize,
                                   error: *mut errors::FriendSendMessageError)
     -> u32;
    pub fn tox_callback_friend_read_receipt(tox: *mut Tox,
                                            function: tox_friend_read_receipt_cb,
                                            user_data: *mut ::libc::c_void)
     -> ();
    pub fn tox_callback_friend_request(tox: *mut Tox,
                                       function: tox_friend_request_cb,
                                       user_data: *mut ::libc::c_void) -> ();
    pub fn tox_callback_friend_message(tox: *mut Tox,
                                       function: tox_friend_message_cb,
                                       user_data: *mut ::libc::c_void) -> ();
    pub fn tox_hash(hash: *mut u8, data: *const u8, length: usize)
     -> u8;
    pub fn tox_file_control(tox: *mut Tox, friend_number: u32,
                            file_number: u32, control: FileControl,
                            error: *mut errors::FileControlError) -> u8;
    pub fn tox_callback_file_recv_control(tox: *mut Tox,
                                          function: tox_file_recv_cb,
                                          user_data: *mut ::libc::c_void)
     -> ();
    pub fn tox_file_seek(tox: *mut Tox, friend_number: u32,
                         file_number: u32, position: u64,
                         error: *mut errors::FileSeekError) -> u8;
    pub fn tox_file_get_file_id(tox: *const Tox, friend_number: u32,
                                file_number: u32, file_id: *mut u8,
                                error: *mut errors::FileGetError) -> u8;
    pub fn tox_file_send(tox: *mut Tox, friend_number: u32,
                         kind: u32, file_size: u64,
                         file_id: *const u8, filename: *const u8,
                         filename_length: usize,
                         error: *mut errors::FileSendError) -> u32;
    pub fn tox_file_send_chunk(tox: *mut Tox, friend_number: u32,
                               file_number: u32, position: u64,
                               data: *const u8, length: usize,
                               error: *mut errors::FileSendChunkError) -> u8;
    pub fn tox_callback_file_chunk_request(tox: *mut Tox,
                                           function: tox_file_chunk_request_cb,
                                           user_data: *mut ::libc::c_void)
     -> ();
    pub fn tox_callback_file_recv(tox: *mut Tox,
                                  function: tox_file_recv_cb,
                                  user_data: *mut ::libc::c_void) -> ();
    pub fn tox_callback_file_recv_chunk(tox: *mut Tox,
                                        function: tox_file_recv_chunk_cb,
                                        user_data: *mut ::libc::c_void) -> ();
    pub fn tox_friend_send_lossy_packet(tox: *mut Tox,
                                        friend_number: u32,
                                        data: *const u8, length: usize,
                                        error:
                                            *mut errors::FriendCustomPacketError)
     -> u8;
    pub fn tox_callback_friend_lossy_packet(tox: *mut Tox,
                                            function: tox_friend_lossy_packet_cb,
                                            user_data: *mut ::libc::c_void)
     -> ();
    pub fn tox_friend_send_lossless_packet(tox: *mut Tox,
                                           friend_number: u32,
                                           data: *const u8,
                                           length: usize,
                                           error:
                                               *mut errors::FriendCustomPacketError)
     -> u8;
    pub fn tox_callback_friend_lossless_packet(tox: *mut Tox,
                                               function: tox_friend_lossless_packet_cb,
                                               user_data: *mut ::libc::c_void)
     -> ();
    pub fn tox_self_get_dht_id(tox: *const Tox, dht_id: *mut u8) -> ();
    pub fn tox_self_get_udp_port(tox: *const Tox,
                                 error: *mut TOX_ERR_GET_PORT) -> u16;
    pub fn tox_self_get_tcp_port(tox: *const Tox,
                                 error: *mut TOX_ERR_GET_PORT) -> u16;

    // +-----------------------------------------------------+
    // | KLUDGE ALERT!!! This will be removed in the future. |
    // +-----------------------------------------------------+
    pub fn tox_callback_group_invite(tox: *mut Tox,
                                     function:
                                         /*Option<*/extern fn
                                                  (arg1: *mut Tox,
                                                   arg2: i32,
                                                   arg3: u8,
                                                   arg4: *const u8,
                                                   arg5: u16,
                                                   arg6: *mut c_void)/*>*/,
                                     userdata: *mut c_void);
    pub fn tox_callback_group_message(tox: *mut Tox,
                                      function:
                                          /*Option<*/extern fn
                                                   (arg1: *mut Tox,
                                                    arg2: c_int,
                                                    arg3: c_int,
                                                    arg4: *const u8,
                                                    arg5: u16,
                                                    arg6: *mut c_void)/*>*/,
                                      userdata: *mut c_void);
    pub fn tox_callback_group_action(tox: *mut Tox,
                                     function:
                                         /*Option<*/extern fn
                                                  (arg1: *mut Tox,
                                                   arg2: c_int,
                                                   arg3: c_int,
                                                   arg4: *const u8,
                                                   arg5: u16,
                                                   arg6: *mut c_void)/*>*/,
                                     userdata: *mut c_void);
    pub fn tox_callback_group_title(tox: *mut Tox,
                                    function:
                                        /*Option<*/extern fn
                                                 (arg1: *mut Tox,
                                                  arg2: c_int,
                                                  arg3: c_int,
                                                  arg4: *const u8,
                                                  arg5: u8,
                                                  arg6: *mut c_void)/*>*/,
                                    userdata: *mut c_void);
    pub fn tox_callback_group_namelist_change(tox: *mut Tox,
                                              function:
                                                  /*Option<*/extern fn
                                                           (arg1: *mut Tox,
                                                            arg2: c_int,
                                                            arg3: c_int,
                                                            arg4: u8,
                                                            arg5: *mut c_void)/*>*/,
                                              userdata: *mut c_void);
    pub fn tox_add_groupchat(tox: *mut Tox) -> c_int;
    pub fn tox_del_groupchat(tox: *mut Tox, groupnumber: c_int) -> c_int;
    pub fn tox_group_peername(tox: *const Tox, groupnumber: c_int, peernumber: c_int,
                              name: *mut u8) -> c_int;
    pub fn tox_group_peer_pubkey(tox: *const Tox, groupnumber: c_int, peernumber: c_int,
                                 pk: *mut u8) -> c_int;
    pub fn tox_invite_friend(tox: *mut Tox, friendnumber: i32,
                             groupnumber: c_int) -> c_int;
    pub fn tox_join_groupchat(tox: *mut Tox, friendnumber: i32, data: *const u8,
                              length: u16) -> c_int;
    pub fn tox_group_message_send(tox: *mut Tox, groupnumber: c_int, message: *const u8,
                                  length: u16) -> c_int;
    pub fn tox_group_get_title(tox: *const Tox, groupnumber: c_int,
                               title: *mut u8, max_length: u32) -> c_int;
    pub fn tox_group_set_title(tox: *mut Tox, groupnumber: c_int, title: *const u8,
                               length: u8) -> c_int;
    pub fn tox_group_action_send(tox: *mut Tox, groupnumber: c_int, action: *const u8,
                                 length: u16) -> c_int;
    pub fn tox_group_peernumber_is_ours(tox: *const Tox, groupnumber: c_int,
                                        peernumber: c_int) -> c_uint;
    pub fn tox_group_number_peers(tox: *const Tox, groupnumber: c_int) -> c_int;
    pub fn tox_group_get_names(tox: *const Tox, groupnumber: c_int,
                               names: *mut [u8; 128], lengths: *mut u16,
                               length: u16) -> c_int;
    pub fn tox_count_chatlist(tox: *const Tox) -> u32;
    pub fn tox_get_chatlist(tox: *const Tox, out_list: *mut i32, list_size: u32) -> u32;
    pub fn tox_group_get_type(tox: *const Tox, groupnumber: c_int) -> c_int;
    // ================================
    // END OF NECESSARY DERPECATED CODE
    // ================================
}
