use std::sync::mpsc::{channel, Receiver, Sender};
use std::{slice, mem, ptr, fmt};
use std::cell::RefCell;
use std::rc::Rc;
use std::old_io::timer::{sleep};
use std::time::duration::Duration;
use rust_core::str::FromStr;
use libc::{c_uint, c_void};
pub use self::ll::Tox as Tox_Struct;
pub use self::Event::*;

mod ll;

pub const MAX_NAME_LENGTH:              usize = 128us;
pub const MAX_MESSAGE_LENGTH:           usize = 1368us;
pub const MAX_STATUSMESSAGE_LENGTH:     usize = 1007us;
pub const TOX_MAX_FRIENDREQUEST_LENGTH: usize = 1016us;
pub const ID_CLIENT_SIZE:               usize = 32us;
pub const ADDRESS_SIZE:                 usize = ID_CLIENT_SIZE + 6us;
pub const AVATAR_MAX_DATA_LENGTH:       usize = 16384us;
pub const HASH_LENGTH:                  usize = 32us;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum AvatarFormat {
    None = ll::TOX_AVATAR_FORMAT_NONE as u8,
    PNG = ll::TOX_AVATAR_FORMAT_PNG as u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum GroupchatType {
    Text = ll::TOX_GROUPCHAT_TYPE_TEXT as u8,
    Av = ll::TOX_GROUPCHAT_TYPE_AV as u8,
}

/// A Tox address consist of `ClientId`, nospam and checksum
#[repr(C)]
#[derive(PartialEq, Clone)]
pub struct Address {
    id: ClientId,
    nospam: [u8; 4],
    // #[allow(dead_code)]
    checksum: [u8; 2],
}

impl Address {
    pub fn client_id(&self) -> &ClientId {
        &self.id
    }
    fn checksum(&self) -> [u8; 2] {
        let mut check = [0u8, 0u8];
        for (i, &x) in self.id.raw.iter().enumerate() {
            check[i % 2] ^= x;
        }
        for i in range(0us, 4) {
            check[(ID_CLIENT_SIZE + i) % 2] ^= self.nospam[i];
        }
        check
    }
}

impl fmt::Display for Address {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(self.id.fmt(fmt));
        try!(write!(fmt, "{:02X}", self.nospam[0]));
        try!(write!(fmt, "{:02X}", self.nospam[1]));
        try!(write!(fmt, "{:02X}", self.nospam[2]));
        try!(write!(fmt, "{:02X}", self.nospam[3]));
        let check = self.checksum();
        try!(write!(fmt, "{:02X}", check[0]));
        try!(write!(fmt, "{:02X}", check[1]));
        Ok(())
    }
}

impl FromStr for Address {
    type Err = ();
    fn from_str(s: &str) -> Result<Address, ()> {
        if s.len() != 2 * ADDRESS_SIZE {
            return Err(());
        }

        let mut id     = [0u8; 32];
        let mut nospam = [0u8; 4];
        let mut check  = [0u8; 2];

        if parse_hex(&s[0..2 * ID_CLIENT_SIZE], id.as_mut_slice()).is_err() {
            return Err(());
        }
        if parse_hex(&s[2 * ID_CLIENT_SIZE..2 * ID_CLIENT_SIZE + 8],
           nospam.as_mut_slice()).is_err() {
            return Err(());
        }
        if parse_hex(&s[2 * ID_CLIENT_SIZE + 8..2 * ADDRESS_SIZE],
           check.as_mut_slice()).is_err() {
            return Err(());
        }

        let addr = Address { id: ClientId { raw: id }, nospam: nospam, checksum: check };
        if addr.checksum().as_slice() != check.as_slice() {
            return Err(());
        }
        Ok(addr)
    }
}

fn parse_hex(s: &str, buf: &mut [u8]) -> Result<(),()> {
    if s.len() != 2*buf.len() {
        return Err(());
    }
    for i in range(0us, buf.len()) {
        for j in range(0us, 2) {
            buf[i] = (buf[i] << 4) + match s.as_bytes()[2*i + j] as char {
                c @ '0' ... '9' => (c as u8) - ('0' as u8),
                c @ 'a' ... 'f' => (c as u8) - ('a' as u8) + 10,
                c @ 'A' ... 'F' => (c as u8) - ('A' as u8) + 10,
                _              => return Err(()),
            }
        }
    }
    return Ok(());
}

/// `ClientId` is the main part of tox `Address`. Other two are nospam and checksum.
#[repr(C)]
#[derive(PartialEq, Clone)]
#[allow(missing_copy_implementations)]
pub struct ClientId {
    pub raw: [u8; ID_CLIENT_SIZE],
}

impl fmt::Display for ClientId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for &n in self.raw.iter() {
            try!(write!(fmt, "{:02X}", n));
        }
        Ok(())
    }
}

impl FromStr for ClientId {
    type Err = ();
    fn from_str(s: &str) -> Result<ClientId, ()> {
        if s.len() != 2 * ID_CLIENT_SIZE {
            return Err(());
        }

        let mut id = [0u8; ID_CLIENT_SIZE];

        try!(parse_hex(s, id.as_mut_slice()));
        Ok(ClientId { raw: id })
    }
}

/// Locally-calculated cryptographic hash of the avatar data
#[derive(Clone, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct Hash {
    pub hash: [u8; HASH_LENGTH]
}

impl Hash {
    pub fn new(data: &[u8]) -> Result<Hash, ()> {
        let mut hash: Hash = unsafe { mem::uninitialized() };
        let res = unsafe {
            ll::tox_hash(hash.hash.as_mut_ptr(), data.as_ptr(), data.len() as u32)
        };
        match res {
            0 => Ok(hash),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    Online,
    Offline,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UserStatus {
    None = ll::TOX_USERSTATUS_NONE,
    Away = ll::TOX_USERSTATUS_AWAY,
    Busy = ll::TOX_USERSTATUS_BUSY,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ChatChange {
    PeerAdd  = ll::TOX_CHAT_CHANGE_PEER_ADD,
    PeerDel  = ll::TOX_CHAT_CHANGE_PEER_DEL,
    PeerName = ll::TOX_CHAT_CHANGE_PEER_NAME,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ControlType {
    Accept       = ll::TOX_FILECONTROL_ACCEPT,
    Pause        = ll::TOX_FILECONTROL_PAUSE,
    Kill         = ll::TOX_FILECONTROL_KILL,
    Finished     = ll::TOX_FILECONTROL_FINISHED,
    ResumeBroken = ll::TOX_FILECONTROL_RESUME_BROKEN,
}

/// Faerr - Friend Add Error
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Faerr {
    Toolong      = ll::TOX_FAERR_TOOLONG,
    Nomessage    = ll::TOX_FAERR_NOMESSAGE,
    Ownkey       = ll::TOX_FAERR_OWNKEY,
    Alreadysent  = ll::TOX_FAERR_ALREADYSENT,
    Unknown      = ll::TOX_FAERR_UNKNOWN,
    Badchecksum  = ll::TOX_FAERR_BADCHECKSUM,
    Setnewnospam = ll::TOX_FAERR_SETNEWNOSPAM,
    Nomem        = ll::TOX_FAERR_NOMEM,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TransferType {
    Receiving,
    Sending,
}

/// Tox events enum
#[derive(Clone)]
pub enum Event {
    /// The first value is the client id, the second is the friend request message
    FriendRequest(Box<ClientId>, String),
    /// `(fnum, msg)` where `fnum` is the friend number and `msg` is the received message
    FriendMessage(i32, String),
    /// `(fnum, msg)` where `fnum` is the friend number and `msg` is the action message
    FriendAction(i32, String),
    /// `(fnum, name)` where `fnum` is the friend number and `name` is the new friend name
    NameChange(i32, String),
    /// `(fnum, status)` where `fnum` is the friend number and `status` is the status
    /// message
    StatusMessage(i32, String),
    /// `(fnum, usrst)` where `fnum` is the friend number and `usrst` is the friend status
    UserStatusVar(i32, UserStatus),
    /// `(fnum, is_typing)`. `true` value of is_typing means that friend is typing. `fnum`
    /// is the friend number
    TypingChange(i32, bool),
    // ?
    ReadReceipt(i32, u32),
    /// `(fnum, ConnectionStatus)`. `fnum` is the friend number
    ConnectionStatusVar(i32, ConnectionStatus),
    /// `(fnum, ty, data)` where `data` is special data what needs
    /// to be passed to Tox::join_group method, `fnum` is the friend number, and `ty` is
    /// the type of the group.
    GroupInvite(i32, GroupchatType, Vec<u8>),
    /// `(gnum, pnum, msg)` where `gnum` is the group number, `pnum` is the peer number
    /// and `msg` is the message
    GroupMessage(i32, i32, String),
    /// `(gnum, pnum, ChatChange)`
    GroupNamelistChange(i32, i32, ChatChange),
    /// `(fnum, fid, fisize, finame)`
    FileSendRequest(i32, u8, u64, Vec<u8>),
    /// `(fnum, TranserType, fid, ControlType, data)`
    FileControl(i32, TransferType, u8, ControlType, Vec<u8>),
    /// `(fnum, fid, data)`
    FileData(i32, u8, Vec<u8>),
    /// `(fnum, AvatarFormat, Hash)`
    AvatarInfo(i32, AvatarFormat, Hash),
    /// `(fnum, AvatarFormat, Hash, data)`
    AvatarData(i32, AvatarFormat, Hash, Vec<u8>),
}

#[repr(u8)]
#[derive(Copy)]
pub enum ProxyType {
    None,
    Socks5,
    HTTP,
}

/**
    ToxOptions provides options that tox will be initalized with.

    Usage:
    ```
        let txo = ToxOptions::new().ipv6().proxy("[proxy address]", port);
        let tox = Tox::new(txo);
    ```
*/
#[derive(Copy)]
pub struct ToxOptions {
    txo: ll::Tox_Options
}

impl ToxOptions {
    /// Create a default ToxOptions struct
    pub fn new() -> ToxOptions {
        ToxOptions {
            txo: ll::Tox_Options {
                ipv6enabled: 0,
                udp_disabled: 0,
                proxy_type: 0,
                proxy_address: [0; 256us],
                proxy_port: 0,
            }
        }
    }

    /// Enable ipv6
    pub fn ipv6(mut self) -> ToxOptions {
        self.txo.ipv6enabled = 1;
        self
    }

    /// Disable UDP
    pub fn no_udp(mut self) -> ToxOptions {
        self.txo.udp_disabled = 1;
        self
    }

    /// Use a proxy
    pub fn proxy(mut self, ty: ProxyType, addr: &str, port: u16) -> ToxOptions {
        if addr.len() >= 256 {
            panic!("proxy address is too long");
        }

        self.txo.proxy_address.as_mut_slice()
        .clone_from_slice(addr.as_bytes());
        self.txo.proxy_type = ty as u8;
        self.txo.proxy_port = port;
        self
    }
}

pub struct ToxIter {
    rx: Rc<RefCell<Receiver<Event>>>,
}

impl ToxIter {
    fn new(rx: Rc<RefCell<Receiver<Event>>>) -> ToxIter {
        ToxIter { rx: rx }
    }
}

impl Iterator for ToxIter {
    type Item = Event;
    fn next(&mut self) -> Option<Event> { self.rx.borrow_mut().try_recv().ok() }
}


macro_rules! some_or_minus {
    ($e:expr, $val:expr) => {
        match $e {
            -1 => None,
            _ => Some($val),
        }
    };
    ($e:expr) => {
        match $e {
            -1 => None,
            v => Some(v),
        }
    }
}

macro_rules! ok_or_minus {
    ($e:expr, $val:expr) => {
        match $e {
            -1 => Err(()),
            _ => Ok($val),
        }
    };
    ($e:expr) => {
        match $e {
            -1 => Err(()),
            v => Ok(v),
        }
    }
}

pub struct Tox {
    raw: *mut ll::Tox,
    event_rx: Rc<RefCell<Receiver<Event>>>,
    #[allow(dead_code)]
    event_tx: Box<Sender<Event>>,
}

impl Drop for Tox {
    fn drop(&mut self) {
        unsafe { ll::tox_kill(self.raw); }
    }
}

impl Tox {
    /// Create a new tox instance. Panics on failure
    pub fn new(mut opts: ToxOptions) -> Tox {
        let tox = unsafe { ll::tox_new(&mut opts.txo) };
        if tox.is_null() { panic!("couldn't initalize tox") }

        let (tx, rx) = channel::<Event>();
        let mut btx = box tx;
        let rrrx = Rc::new(RefCell::new(rx)); // too much bloat to just get a channel, eh?
        let chan = &mut *btx as *mut _ as *mut c_void;
        unsafe {
            ll::tox_callback_friend_request(        tox, on_friend_request,        chan);
            ll::tox_callback_friend_message(        tox, on_friend_message,        chan);
            ll::tox_callback_friend_action(         tox, on_friend_action,         chan);
            ll::tox_callback_name_change(           tox, on_name_change,           chan);
            ll::tox_callback_status_message(        tox, on_status_message,        chan);
            ll::tox_callback_user_status(           tox, on_user_status,           chan);
            ll::tox_callback_typing_change(         tox, on_typing_change,         chan);
            ll::tox_callback_read_receipt(          tox, on_read_receipt,          chan);
            ll::tox_callback_connection_status(     tox, on_connection_status,     chan);
            ll::tox_callback_group_invite(          tox, on_group_invite,          chan);
            ll::tox_callback_group_message(         tox, on_group_message,         chan);
            ll::tox_callback_group_action(          tox, on_group_action,          chan);
            ll::tox_callback_group_namelist_change( tox, on_group_namelist_change, chan);
            ll::tox_callback_file_send_request(     tox, on_file_send_request,     chan);
            ll::tox_callback_file_control(          tox, on_file_control,          chan);
            ll::tox_callback_file_data(             tox, on_file_data,             chan);
            ll::tox_callback_avatar_info(           tox, on_avatar_info,           chan);
            ll::tox_callback_avatar_data(           tox, on_avatar_data,           chan);
        }

        Tox {
            raw: tox,
            event_rx: rrrx,
            event_tx: btx,
        }
    }

    /// Ticks the Tox and returns an iterator to the Tox events
    pub fn iter(&mut self) -> ToxIter {
        self.tick();

        ToxIter::new(self.event_rx.clone())
    }

    /// This function animates tox by calling `tox_do()` It function should be called
    /// at least several times per second. Use `wait()` method to get optimal delays
    pub fn tick(&mut self) {
        unsafe { ll::tox_do(self.raw); }
    }

    /// This function makes thread sleep for a some time, optimal for `tick()` method
    pub fn wait(&mut self) {
        unsafe {
            let delay = Duration::milliseconds(ll::tox_do_interval(self.raw) as i64);
            sleep(delay);
        }
    }

    pub fn bootstrap_from_address(&mut self, mut address: String, port: u16,
      public_key: ClientId) -> Result<(), ()> {
        let res = unsafe {
            address.as_mut_vec().push(0);
            ll::tox_bootstrap_from_address(self.raw, address.as_bytes().as_ptr() as *const _,
                port, public_key.raw.as_ptr())
        };
        match res {
            1 => Ok(()),
            _ => Err(()),
        }
    }

    pub fn is_connected(&self) -> bool {
        match unsafe { ll::tox_isconnected(&*self.raw) } {
            0 => false,
            _ => true,
        }
    }

    pub fn get_address(&self) -> Address {
        let mut adr: Address = unsafe { mem::uninitialized() };
        unsafe { ll::tox_get_address(&*self.raw, &mut adr as *mut _ as *mut _); }
        adr
    }

    pub fn add_friend(&mut self, address: Address, msg: &str) -> Result<i32, Faerr> {
        let res = unsafe {
            ll::tox_add_friend(self.raw, &address as *const _ as *const _,
             msg.as_bytes().as_ptr(), msg.len() as u16)
        };
        match res {
            ll::TOX_FAERR_TOOLONG => Err(Faerr::Toolong),
            ll::TOX_FAERR_NOMESSAGE => Err(Faerr::Nomessage),
            ll::TOX_FAERR_OWNKEY => Err(Faerr::Ownkey),
            ll::TOX_FAERR_ALREADYSENT => Err(Faerr::Alreadysent),
            ll::TOX_FAERR_UNKNOWN => Err(Faerr::Unknown),
            ll::TOX_FAERR_BADCHECKSUM => Err(Faerr::Badchecksum),
            ll::TOX_FAERR_SETNEWNOSPAM => Err(Faerr::Setnewnospam),
            ll::TOX_FAERR_NOMEM => Err(Faerr::Nomem),
            n if n >= 0 => Ok(n),
            _ => Err(Faerr::Unknown),
        }
    }

    pub fn add_friend_norequest(&mut self, client_id: ClientId) -> Result<i32, ()> {
        match unsafe { ll::tox_add_friend_norequest(self.raw, client_id.raw.as_ptr()) } {
            -1 => Err(()),
            n => Ok(n),
        }
    }

    pub fn get_friend_number(&self, client_id: &ClientId) -> Option<i32> {
        let res = unsafe {
            ll::tox_get_friend_number(&*self.raw, client_id.raw.as_ptr())
        };
        
        some_or_minus!(res)
    }

    pub fn get_client_id(&self, friendnumber: i32) -> Option<ClientId> {
        let mut client: ClientId = unsafe { mem::uninitialized() };
        let res = unsafe {
            ll::tox_get_client_id(&*self.raw, friendnumber, client.raw.as_mut_ptr())
        };
        some_or_minus!(res, client)
    }

    pub fn get_friend_connection_status(&self, friendnumber: i32) -> Option<ConnectionStatus> {
        match unsafe { ll::tox_get_friend_connection_status(&*self.raw, friendnumber) } {
            1 => Some(ConnectionStatus::Online),
            0 => Some(ConnectionStatus::Offline),
            _ => None,
        }
    }

    pub fn del_friend(&mut self, friendnumber: i32) -> Option<()> {
        some_or_minus!(unsafe { ll::tox_del_friend(self.raw, friendnumber) }, ())
    }

    pub fn friend_exists(&self, friendnumber: i32) -> bool {
        match unsafe { ll::tox_friend_exists(&*self.raw, friendnumber) } {
            1 => true,
            _ => false,
        }
    }

    pub fn send_message(&mut self, friendnumber: i32, msg: &str) -> Result<u32, ()> {
        let res = unsafe {
            ll::tox_send_message(self.raw, friendnumber, msg.as_bytes().as_ptr(), msg.len() as u32)
        };
        ok_or_minus!(res)
    }

    pub fn send_action(&mut self, friendnumber: i32, act: &str) -> Result<u32, ()> {
        let res = unsafe {
            ll::tox_send_action(self.raw, friendnumber, act.as_bytes().as_ptr(), act.len() as u32)
        };
        ok_or_minus!(res)
    }

    pub fn set_name(&mut self, name: &str) -> Result<(),()> {
        unsafe {
            let bytes = name.as_bytes();
            ok_or_minus!(ll::tox_set_name(self.raw, bytes.as_ptr(), bytes.len() as u16), ())
        }
    }

    pub fn get_self_name(&self) -> Option<String> {
        let mut name = Vec::with_capacity(MAX_NAME_LENGTH);
        let res = unsafe {
            let len = ll::tox_get_self_name(&*self.raw, name.as_mut_ptr());
            name.set_len(len as usize);
            len
        };
        match res {
            0 => None,
            _ => match String::from_utf8(name) {
                Ok(name) => Some(name),
                _ => None,
            },
        }
    }

    pub fn get_name(&self, friendnumber: i32) -> Option<String> {
        let mut name = Vec::with_capacity(MAX_NAME_LENGTH);
        let res = unsafe {
            let len = ll::tox_get_name(&*self.raw, friendnumber, name.as_mut_ptr());
            // len might be -1 but it doesn't matter if we don't return name.
            name.set_len(len as usize);
            len
        };
        match res {
            -1 => None,
            _ => match String::from_utf8(name) {
                Ok(name) => Some(name),
                _ => None,
            },
        }
    }

    pub fn set_status_message(&mut self, status: &str) -> Result<(),()> {
        let res = unsafe {
            ll::tox_set_status_message(self.raw, status.as_bytes().as_ptr(),
             status.len() as u16)
        };
        ok_or_minus!(res, ())
    }

    pub fn set_user_status(&mut self, userstatus: UserStatus) -> Result<(), ()> {
        ok_or_minus!( unsafe { ll::tox_set_user_status(self.raw, userstatus as u8) }, () )
    }

    pub fn get_status_message(&self, friendnumber: i32) -> Option<String> {
        let size = unsafe { ll::tox_get_status_message_size(&*self.raw, friendnumber) };
        let size = match size {
            -1 => return None,
            _ => size,
        };
        let mut status = Vec::with_capacity(size as usize);
        let size = unsafe {
            let len = ll::tox_get_status_message(&*self.raw, friendnumber, status.as_mut_ptr(),
               size as u32);
            status.set_len(len as usize);
            len
        };
        match size {
            -1 => return None,
            _ => match String::from_utf8(status) {
                Ok(status) => Some(status),
                _ => return None,
            },
        }
    }

    pub fn get_self_status_message(&self) -> Option<String> {
        let size = unsafe { ll::tox_get_self_status_message_size(&*self.raw) };
        let size = match size {
            -1 => return None,
            _ => size as u32,
        };
        let mut status = Vec::with_capacity(size as usize);
        let size = unsafe {
            let len = ll::tox_get_self_status_message(&*self.raw, status.as_mut_ptr(), size);
            status.set_len(len as usize);
            len
        };
        match size {
            -1 => return None,
            _ => match String::from_utf8(status) {
                Ok(status) => Some(status),
                _ => return None,
            },
        }
    }

    pub fn get_user_status(&self, friendnumber: i32) -> Option<UserStatus> {
        match unsafe { ll::tox_get_user_status(&*self.raw, friendnumber) as u32 } {
            ll::TOX_USERSTATUS_AWAY => Some(UserStatus::Away),
            ll::TOX_USERSTATUS_NONE => Some(UserStatus::None),
            ll::TOX_USERSTATUS_BUSY => Some(UserStatus::Busy),
            _ => None
        }
    }

    pub fn get_self_user_status(&self) -> Option<UserStatus> {
        match unsafe { ll::tox_get_self_user_status(&*self.raw) as u32 } {
            ll::TOX_USERSTATUS_AWAY => Some(UserStatus::Away),
            ll::TOX_USERSTATUS_NONE => Some(UserStatus::None),
            ll::TOX_USERSTATUS_BUSY => Some(UserStatus::Busy),
            _ => None
        }
    }

    pub fn get_last_online(&self, friendnumber: i32) -> Option<u64> {
        some_or_minus!( unsafe { ll::tox_get_last_online(&*self.raw, friendnumber) } )
    }

    pub fn set_user_is_typing(&mut self, friendnumber: i32, is_typing: bool) -> Result<(), ()> {
        let raw = unsafe {
            ll::tox_set_user_is_typing(self.raw, friendnumber, is_typing as u8)
        };
        ok_or_minus!(raw, ())
    }

    pub fn get_is_typing(&self, friendnumber: i32) -> bool {
        match unsafe { ll::tox_get_is_typing(&*self.raw, friendnumber) } {
            0 => false,
            _ => true,
        }
    }

    pub fn count_friendlist(&self) -> u32 {
        unsafe { ll::tox_count_friendlist(&*self.raw) }
    }

    pub fn get_num_online_friends(&self) -> u32 {
        unsafe { ll::tox_get_num_online_friends(&*self.raw) }
    }

    pub fn get_friendlist(&self) -> Vec<i32> {
        let size = self.count_friendlist();
        let mut vec = Vec::with_capacity(size as usize);
        unsafe {
            let len = ll::tox_get_friendlist(&*self.raw, vec.as_mut_ptr(), size);
            vec.set_len(len as usize);
        }
        vec
    }
/*
    pub fn get_nospam(&self) -> [u8; 4] {
        unsafe { mem::transmute(ll::tox_get_nospam(&*self.raw).to_be()) }
    }

    pub fn set_nospam(&mut self, nospam: [u8; 4]) {
        unsafe { ll::tox_set_nospam(self.raw, ::std::num::Int::from_be(mem::transmute(nospam))); }
    }
*/
    // Groupchats

    pub fn add_groupchat(&mut self) -> Result<i32, ()> {
        match unsafe { ll::tox_add_groupchat(self.raw) } {
            -1 => Err(()),
            n => Ok(n),
        }
    }

    pub fn del_groupchat(&mut self, groupnumber: i32) -> Result<(),()> {
        unsafe { ok_or_minus!(ll::tox_del_groupchat(self.raw, groupnumber), ()) }
    }

    pub fn group_peername(&self, groupnumber: i32, peernumber: i32) -> Option<String> {
        let mut vec = Vec::with_capacity(MAX_NAME_LENGTH);
        let len = unsafe {
            let len = ll::tox_group_peername(&*self.raw, groupnumber, peernumber,
                                         vec.as_mut_ptr());
            vec.set_len(len as usize);
            len
        };
        match len {
            -1 => None,
            _ => match String::from_utf8(vec) {
                Ok(name) => Some(name),
                _ => None,
            }
        }
    }

    pub fn invite_friend(&mut self, friendnumber: i32, groupnumber: i32) -> Result<(), ()> {
        unsafe { ok_or_minus!(ll::tox_invite_friend(self.raw, friendnumber, groupnumber), ()) }
    }

    pub fn join_groupchat(&mut self, friendnumber: i32,
                      data: Vec<u8>) -> Result<i32, ()> {
        let res = unsafe {
            ll::tox_join_groupchat(self.raw, friendnumber, data.as_ptr(), data.len() as u16)
        };
        match res {
            -1 => Err(()),
            n => Ok(n),
        }
    }

    pub fn group_message_send(&mut self, groupnumber: i32, msg: &str) -> Result<(), ()> {
        let res = unsafe {
            ll::tox_group_message_send(self.raw, groupnumber, msg.as_bytes().as_ptr(),
                                   msg.len() as u16)
        };
        ok_or_minus!(res, ())
    }

    pub fn group_action_send(&mut self, groupnumber: i32, act: &str) -> Result<(), ()> {
        let res = unsafe {
            ll::tox_group_action_send(self.raw, groupnumber, act.as_bytes().as_ptr(),
                                  act.len() as u16)
        };
        ok_or_minus!(res, ())
    }

    pub fn group_number_peers(&self, groupnumber: i32) -> Option<i32> {
        some_or_minus!( unsafe { ll::tox_group_number_peers(&*self.raw, groupnumber) } )
    }

    pub fn group_get_names(&self, groupnumber: i32) -> Result<Vec<Option<String>>, ()> {
        let num = match self.group_number_peers(groupnumber) {
            Some(n) => n as usize,
            _ => return Err(()),
        };
        let mut names = Vec::with_capacity(num);
        let mut lengths = Vec::with_capacity(num);
        let len = unsafe {
            let len = ll::tox_group_get_names(&*self.raw, groupnumber, names.as_mut_ptr(),
                                          lengths.as_mut_ptr(), num as u16);
            names.set_len(len as usize);
            lengths.set_len(len as usize);
            len
        };
        if len == -1 {
            return Err(());
        }
        let mut real_names = Vec::with_capacity(len as usize);
        for (name, &length) in names.iter().zip(lengths.iter()) {
            match ::std::str::from_utf8(&name[..length as usize]) {
                Ok(s) => real_names.push(Some(s.to_string())),
                _ => real_names.push(None),
            }
        }
        Ok(real_names)
    }

    pub fn count_chatlist(&self) -> u32 {
        unsafe { ll::tox_count_chatlist(&*self.raw) }
    }

    pub fn get_chatlist(&self) -> Vec<i32> {
        let num = unsafe { ll::tox_count_chatlist(&*self.raw) };
        let mut vec = Vec::with_capacity(num as usize);
        unsafe {
            let num = ll::tox_get_chatlist(&*self.raw, vec.as_mut_ptr(), num);
            vec.set_len(num as usize);
        }
        vec
    }

    // Avatars

    pub fn set_avatar(&mut self, format: AvatarFormat, data: &[u8]) -> Result<(), ()> {
        let res = unsafe {
            ll::tox_set_avatar(self.raw, format as u8, data.as_ptr(), data.len() as u32)
        };
        ok_or_minus!(res, ())
    }

    pub fn unset_avatar(&mut self) {
        unsafe { ll::tox_unset_avatar(self.raw); }
    }

    pub fn get_self_avatar(&self) -> Option<(AvatarFormat, Vec<u8>, Hash)> {
        let mut data = Vec::with_capacity(AVATAR_MAX_DATA_LENGTH);
        let mut hash: Hash = unsafe { mem::uninitialized() };
        let mut format = 0;
        let mut length = 0;
        let res = unsafe {
            ll::tox_get_self_avatar(self.raw, &mut format, data.as_mut_ptr(), &mut length,
                                AVATAR_MAX_DATA_LENGTH as u32, hash.hash.as_mut_ptr())
        };
        if res == -1 {
            return None;
        }
        unsafe { data.set_len(length as usize); }
        data.shrink_to_fit();
        let format = match format as c_uint {
            ll::TOX_AVATAR_FORMAT_NONE => AvatarFormat::None,
            ll::TOX_AVATAR_FORMAT_PNG => AvatarFormat::PNG,
            _ => return None,
        };
        Some((format, data, hash))
    }

    pub fn request_avatar_info(&self, friendnumber: i32) -> Result<(), ()> {
        let res = unsafe {
            ll::tox_request_avatar_info(self.raw, friendnumber)
        };
        ok_or_minus!(res, ())
    }

    pub fn request_avatar_data(&self, friendnumber: i32) -> Result<(), ()> {
        let res = unsafe {
            ll::tox_request_avatar_data(self.raw, friendnumber)
        };
        ok_or_minus!(res, ())
    }

    pub fn send_avatar_info(&mut self, friendnumber: i32) -> Result<(), ()> {
        let res = unsafe {
            ll::tox_send_avatar_info(self.raw, friendnumber)
        };
        ok_or_minus!(res, ())
    }

    // File sending

    pub fn new_file_sender(&mut self, friendnumber: i32, filesize: u64, filename: &Path)
            -> Result<i32, ()> {
        let filename = filename.as_vec();
        let res = unsafe {
            ll::tox_new_file_sender(self.raw, friendnumber, filesize,
                                filename.as_ptr(), filename.len() as u16)
        };
        match res {
            -1 => Err(()),
            n => Ok(n)
        }
    }

    pub fn file_send_control(&mut self, friendnumber: i32, send_receive: TransferType,
            filenumber: u8, message_id: u8, pos: u64) -> Result<(), ()> {
        unsafe {
            let mut data: [u8; 8] = mem::transmute(pos);
            let len = match message_id as u32 {
                ll::TOX_FILECONTROL_RESUME_BROKEN => data.len(),
                _ => 0,
            };
            let res = ll::tox_file_send_control(self.raw, friendnumber, 1 - send_receive as u8,
                                  filenumber, message_id, data.as_ptr(),
                                  len as u16);
            ok_or_minus!(res, ())
        }
    }

    pub fn file_send_data(&mut self, friendnumber: i32, filenumber: u8,
                      data: Vec<u8>) -> Result<(), ()> {
        let res = unsafe {
            ll::tox_file_send_data(self.raw, friendnumber, filenumber, data.as_ptr(),
                               data.len() as u16)
        };
        ok_or_minus!(res, ())
    }

    pub fn file_data_size(&self, friendnumber: i32) -> Option<i32> {
        some_or_minus!( unsafe { ll::tox_file_data_size(&*self.raw, friendnumber) } )
    }

    pub fn file_data_remaining(&mut self, friendnumber: i32, filenumber: u8, send_receive: TransferType)
            -> Option<u64> {
        let res = unsafe {
            ll::tox_file_data_remaining(&*self.raw, friendnumber, filenumber,
                                    send_receive as u8)
        };
        match res {
            0 => None,
            n => Some(n),
        }
    }

    pub fn save(&mut self) -> Vec<u8> {
        unsafe {
            let size = ll::tox_size(self.raw) as usize;
            let mut buf: Vec<u8> = Vec::with_capacity(size);
            buf.set_len(size);
            ll::tox_save(self.raw, buf.as_mut_ptr());
            buf
        }
    }

    pub fn load(&mut self, data: &[u8]) -> Result<(), ()> {
        unsafe {
            match ll::tox_load(self.raw, data.as_ptr(), data.len() as u32) {
                0 => Ok(()),
                _ => Err(()),
            }
        }
    }

    pub unsafe fn raw(&mut self) -> *mut ll::Tox {
        self.raw
    }
}


macro_rules! parse_string {
    ($p:expr, $l:ident) => {
        unsafe {
            let slice = slice::from_raw_buf($p, $l as usize);
            match ::std::str::from_utf8(slice) {
                Ok(s) => s.to_string(),
                _ => return,
            }
        }
    }
}

// BEGIN: Callback pack

extern fn on_friend_request(_: *mut ll::Tox, public_key: *const u8, data: *const u8,
        length: u16, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };
    let msg = parse_string!(&data, length);
    let id = ClientId { raw: unsafe { ptr::read(public_key as *const _) } };
    tx.send(FriendRequest(box id, msg)).unwrap();
}

extern fn on_friend_message(_: *mut ll::Tox, friendnumber: i32, msg: *const u8, length: u16,
        chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };
    let msg = parse_string!(&msg, length);
    tx.send(FriendMessage(friendnumber, msg)).unwrap();
}

extern fn on_friend_action(_: *mut ll::Tox, friendnumber: i32, act: *const u8, length: u16,
        chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };
    let act = parse_string!(&act, length);
    tx.send(FriendAction(friendnumber, act)).unwrap();
}

extern fn on_name_change(_: *mut ll::Tox, friendnumber: i32, new: *const u8, length: u16,
        chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };
    let new = parse_string!(&new, length);
    tx.send(NameChange(friendnumber, new)).unwrap();
}

extern fn on_status_message(_: *mut ll::Tox, friendnumber: i32, new: *const u8, length: u16,
        chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };
    let new = parse_string!(&new, length);
    tx.send(StatusMessage(friendnumber, new)).unwrap();
}

extern fn on_user_status(_: *mut ll::Tox, friendnumber: i32, status: u8,
        chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };
    let status = match status as u32 {
        ll::TOX_USERSTATUS_NONE => UserStatus::None,
        ll::TOX_USERSTATUS_AWAY => UserStatus::Away,
        ll::TOX_USERSTATUS_BUSY => UserStatus::Busy,
        _ => return,
    };
    tx.send(UserStatusVar(friendnumber, status)).unwrap();
}

extern fn on_typing_change(_: *mut ll::Tox, friendnumber: i32, is_typing: u8, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };
    tx.send(TypingChange(friendnumber, is_typing != 0)).unwrap();
}

extern fn on_read_receipt(_: *mut ll::Tox, friendnumber: i32, receipt: u32, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };
    tx.send(ReadReceipt(friendnumber, receipt)).unwrap();
}

extern fn on_connection_status(_: *mut ll::Tox, friendnumber: i32, status: u8, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };    
    let status = match status {
        1 => ConnectionStatus::Online,
        _ => ConnectionStatus::Offline,
    };
    tx.send(ConnectionStatusVar(friendnumber, status)).unwrap();
}

extern fn on_group_invite(_: *mut ll::Tox, friendnumber: i32, ty: u8, data: *const u8, 
        length: u16, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };
    let data = unsafe {
        Vec::from_raw_buf(data, length as usize)
    };
    let ty = match ty as c_uint {
        ll::TOX_GROUPCHAT_TYPE_TEXT => GroupchatType::Text,
        ll::TOX_GROUPCHAT_TYPE_AV => GroupchatType::Av,
        _ => return,
    };
    tx.send(GroupInvite(friendnumber, ty, data)).unwrap();
}

extern fn on_group_message(_: *mut ll::Tox, groupnumber: i32, frindgroupnumber: i32,
        message: *const u8, len: u16, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };
    let msg = parse_string!(&message, len);
    tx.send(GroupMessage(groupnumber, frindgroupnumber, msg)).unwrap();
}

extern fn on_group_action(_: *mut ll::Tox, groupnumber: i32, frindgroupnumber: i32,
        action: *const u8, len: u16, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };    
    let action = parse_string!(&action, len);
    tx.send(GroupMessage(groupnumber, frindgroupnumber, action)).unwrap();
}

extern fn on_group_namelist_change(_: *mut ll::Tox, groupnumber: i32, peernumber: i32,
        change: u8, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };    
    let change = match change as u32 {
        ll::TOX_CHAT_CHANGE_PEER_ADD => ChatChange::PeerAdd,
        ll::TOX_CHAT_CHANGE_PEER_DEL => ChatChange::PeerDel,
        ll::TOX_CHAT_CHANGE_PEER_NAME => ChatChange::PeerName,
        _ => return,
    };
    tx.send(GroupNamelistChange(groupnumber, peernumber, change)).unwrap();
}

extern fn on_file_send_request(_: *mut ll::Tox, friendnumber: i32, filenumber: u8,
        filesize: u64, filename: *const u8, len: u16, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };    
    let slice = unsafe { slice::from_raw_buf(&filename, len as usize) };
    let path = match Path::new_opt(slice) {
        Some(p) => match p.filename() {
            Some(f) => f.to_vec(),
            None => b"\xbf\xef".to_vec(),
        },
        None => b"\xbf\xef".to_vec(),
    };
    tx.send(FileSendRequest(friendnumber, filenumber, filesize, path)).unwrap();
}

extern fn on_file_control(_: *mut ll::Tox, friendnumber: i32, receive_send: u8,
        filenumber: u8, control_type: u8, data: *const u8, len: u16, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };    
    let ty = match control_type as u32 {
        ll::TOX_FILECONTROL_ACCEPT        => ControlType::Accept,
        ll::TOX_FILECONTROL_PAUSE         => ControlType::Pause,
        ll::TOX_FILECONTROL_KILL          => ControlType::Kill,
        ll::TOX_FILECONTROL_FINISHED      => ControlType::Finished,
        ll::TOX_FILECONTROL_RESUME_BROKEN => ControlType::ResumeBroken,
        _ => return,
    };
    let tt = match receive_send {
        1 => TransferType::Sending,
        0 => TransferType::Receiving,
        _ => return,
    };
    let data = unsafe { slice::from_raw_buf(&data, len as usize).to_vec() };
    tx.send(FileControl(friendnumber, tt, filenumber, ty, data)).unwrap();
}

extern fn on_file_data(_: *mut ll::Tox, friendnumber: i32, filenumber: u8, data: *const u8,
        len: u16, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };    
    let data = unsafe { slice::from_raw_buf(&data, len as usize).to_vec() };
    tx.send(FileData(friendnumber, filenumber, data)).unwrap();
}

extern fn on_avatar_info(_: *mut ll::Tox, friendnumber: i32, format: u8, hash: *mut u8,
        chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };    
    let format = match format as c_uint {
        ll::TOX_AVATAR_FORMAT_NONE => AvatarFormat::None,
        ll::TOX_AVATAR_FORMAT_PNG  => AvatarFormat::PNG,
        _ => return,
    };
    let hash = unsafe { ptr::read(hash as *const u8 as *const _) };
    tx.send(AvatarInfo(friendnumber, format, hash)).unwrap();
}

extern fn on_avatar_data(_: *mut ll::Tox, friendnumber: i32, format: u8, hash: *mut u8,
        data: *mut u8, datalen: u32, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };    
    let format = match format as c_uint {
        ll::TOX_AVATAR_FORMAT_NONE => AvatarFormat::None,
        ll::TOX_AVATAR_FORMAT_PNG  => AvatarFormat::PNG,
        _ => return,
    };
    let hash = unsafe { ptr::read(hash as *const u8 as *const _) };
    let data = unsafe { Vec::from_raw_buf(data, datalen as usize) };
    tx.send(AvatarData(friendnumber, format, hash, data)).unwrap();
}

// END: Callback pack
