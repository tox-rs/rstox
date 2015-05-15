use std::sync::mpsc::{channel, Receiver, Sender};
use std::{slice, mem, ffi, ptr, fmt};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::{sleep_ms};
use std::str::FromStr;

use libc::{c_uint, c_void};

pub use self::ll::Tox as Tox_Struct;
pub use self::Event::*;
pub use self::errors::*;

mod ll;
mod errors;

pub const MAX_NAME_LENGTH:              usize = 128;
pub const MAX_MESSAGE_LENGTH:           usize = 1368;
pub const MAX_STATUSMESSAGE_LENGTH:     usize = 1007;
pub const TOX_MAX_FRIENDREQUEST_LENGTH: usize = 1016;
pub const ID_CLIENT_SIZE:               usize = 32;
pub const ADDRESS_SIZE:                 usize = ID_CLIENT_SIZE + 6;
pub const AVATAR_MAX_DATA_LENGTH:       usize = 16384;
pub const HASH_LENGTH:                  usize = 32;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum AvatarFormat {
    None = 0,
    PNG,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum GroupchatType {
    Text = ll::TOX_GROUPCHAT_TYPE_TEXT as u8,
    Av = ll::TOX_GROUPCHAT_TYPE_AV as u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Connection {
    None = 0,
    Tcp,
    Udp,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ConnectionStatus {
    Online,
    Offline,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UserStatus {
    None = 0,
    Away,
    Busy,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MessageType {
    Normal = 0,
    Action,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ChatChange {
    PeerAdd = 0,
    PeerDel,
    PeerName,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TransferType {
    Receiving = 0,
    Sending,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum FileKind {
    Data = 0,
    Avatar,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum FileControl {
    Resume = 0,
    Pause,
    Cancel,
}

/// A Tox address consist of `PublicKey`, nospam and checksum
#[repr(C)]
#[derive(PartialEq, Clone, Debug)]
pub struct Address {
    id: PublicKey,
    nospam: [u8; 4],
    // #[allow(dead_code)]
    checksum: [u8; 2],
}

impl Address {
    pub fn client_id(&self) -> &PublicKey {
        &self.id
    }
    fn checksum(&self) -> [u8; 2] {
        let mut check = [0u8, 0u8];
        for (i, &x) in self.id.raw.iter().enumerate() {
            check[i % 2] ^= x;
        }
        for i in 0..4 {
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

        if parse_hex(&s[0..2 * ID_CLIENT_SIZE], &mut id[..]).is_err() {
            return Err(());
        }
        if parse_hex(&s[2 * ID_CLIENT_SIZE..2 * ID_CLIENT_SIZE + 8],
           &mut nospam[..]).is_err() {
            return Err(());
        }
        if parse_hex(&s[2 * ID_CLIENT_SIZE + 8..2 * ADDRESS_SIZE],
           &mut check[..]).is_err() {
            return Err(());
        }

        let addr = Address { id: PublicKey { raw: id }, nospam: nospam, checksum: check };
        if &addr.checksum() != &check {
            return Err(());
        }
        Ok(addr)
    }
}

fn parse_hex(s: &str, buf: &mut [u8]) -> Result<(),()> {
    if s.len() != 2*buf.len() {
        return Err(());
    }
    for i in 0..buf.len() {
        for j in 0..2 {
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

/// `PublicKey` is the main part of tox `Address`. Other two are nospam and checksum.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct PublicKey {
    pub raw: [u8; ID_CLIENT_SIZE],
}

impl fmt::Display for PublicKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for &n in self.raw.iter() {
            try!(write!(fmt, "{:02X}", n));
        }
        Ok(())
    }
}

impl FromStr for PublicKey {
    type Err = ();
    fn from_str(s: &str) -> Result<PublicKey, ()> {
        if s.len() != 2 * ID_CLIENT_SIZE {
            return Err(());
        }

        let mut id = [0u8; ID_CLIENT_SIZE];

        try!(parse_hex(s, &mut id[..]));
        Ok(PublicKey { raw: id })
    }
}

/// Locally-calculated cryptographic hash of the avatar data
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[allow(missing_copy_implementations)]
pub struct Hash {
    pub hash: [u8; HASH_LENGTH]
}
/*
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
*/
/// Tox events enum
#[derive(Clone, Debug)]
pub enum Event {
    GroupInvite(i32, GroupchatType, Vec<u8>),
    /// `(gnum, pnum, msg)` where `gnum` is the group number, `pnum` is the peer number
    /// and `msg` is the message
    GroupMessage(i32, i32, String),
    /// `(gnum, pnum, ChatChange)`
    GroupNamelistChange(i32, i32, ChatChange),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ProxyType {
    None = 0,
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ToxOptions {
    txo: ll::Tox_Options
}

impl ToxOptions {
    /// Create a default ToxOptions struct
    pub fn new() -> ToxOptions {
        ToxOptions {
            txo: ::std::default::Default::default(),
        }
    }

    /// Enable ipv6
    pub fn ipv6(mut self) -> ToxOptions {
        self.txo.ipv6_enabled = 1;
        self
    }

    /// Disable UDP
    pub fn no_udp(mut self) -> ToxOptions {
        self.txo.udp_enabled = 0;
        self
    }

    /*
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
    }*/
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
            -1 => return None,
            _ => Some($val),
        }
    };
    ($e:expr) => {
        match $e {
            -1 => return None,
            v => Some(v),
        }
    }
}

macro_rules! ok_or_minus {
    ($e:expr, $val:expr) => {
        match $e {
            -1 => return Err(()),
            _ => Ok($val),
        }
    };
    ($e:expr) => {
        match $e {
            -1 => return Err(()),
            v => Ok(v),
        }
    }
}

macro_rules! tox_try {
    ($err:ident, $exp:expr) => {{
        let mut $err = ::std::mem::zeroed();
        let res = $exp;
        match $err as u32 {
            0 => {},
            _ => return Err($err),
        };
        res
    }};
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
    /// Create a new tox instance
    pub fn new(opts: ToxOptions, data: Option<&[u8]>) -> Result<Tox, InitError> {
        let tox = unsafe {
            match data {
                Some(data) => tox_try!(err, ll::tox_new(&opts.txo, data.as_ptr(), data.len(), &mut err)),
                None => tox_try!(err, ll::tox_new(&opts.txo, ptr::null(), 0, &mut err)),
            }
        };

        let (tx, rx) = channel::<Event>();
        let mut btx = box tx;
        let rrrx = Rc::new(RefCell::new(rx)); // too much bloat to just get a channel, eh?
        unsafe {
            let chan: *mut c_void = mem::transmute(&mut *btx);

        }

        Ok(Tox {
            raw: tox,
            event_rx: rrrx,
            event_tx: btx,
        })
    }

    /// Ticks the Tox and returns an iterator to the Tox events
    pub fn iter(&mut self) -> ToxIter {
        self.tick();

        ToxIter::new(self.event_rx.clone())
    }

    /// This function animates tox by calling `tox_do()` It function should be called
    /// at least several times per second. Use `wait()` method to get optimal delays
    pub fn tick(&mut self) {
        unsafe { ll::tox_iterate(self.raw); }
    }

    /// This function makes thread sleep for a some time, optimal for `tick()` method
    pub fn wait(&self) {
        unsafe {
            let delay = ll::tox_iteration_interval(self.raw);
            sleep_ms(delay);
        }
    }

    /**
        Sends a "get nodes" request to the given bootstrap node with IP, port, and
        public key to setup connections.
        
        This function will attempt to connect to the node using UDP and TCP at the
        same time.
        
        Tox will use the node as a TCP relay in case Tox_Options.udp_enabled was
        false, and also to connect to friends that are in TCP-only mode. Tox will
        also use the TCP connection when NAT hole punching is slow, and later switch
        to UDP if hole punching succeeds.
        
        ## Panics
        Panics if `host` strubg contains `\0`.
    */
    pub fn bootstrap(&mut self, host: &str, port: u16, public_key: PublicKey) -> Result<(), BootstrapError> {
        unsafe {
            let c_host = ffi::CString::new(host).unwrap();
            let c_pk: *const u8 = mem::transmute(&public_key);
            tox_try!(err, ll::tox_bootstrap(self.raw, c_host.as_ptr(), port, c_pk, &mut err));
        }
        Ok(())
    }

    /// Get self connection status
    pub fn get_connection_status(&self) -> Connection {
        unsafe { ll::tox_self_get_connection_status(self.raw) }
    }

    /// Get self tox address
    pub fn get_address(&self) -> Address {
        unsafe {
            let mut addr: Address = mem::uninitialized();
            ll::tox_self_get_address(self.raw, &mut addr as *mut _ as *mut u8);
            addr
        }
    }

    /// Get self nospam
    pub fn get_nospam(&self) -> [u8; 4] {
        unsafe {
            let nospam = ll::tox_self_get_nospam(self.raw);
            mem::transmute(nospam)
        }
    }

    /// Set self nospam
    pub fn set_nospam(&mut self, nospam: [u8; 4]) {
        unsafe {
            let nospam: u32 = mem::transmute(nospam);
            ll::tox_self_set_nospam(self.raw, nospam);
        }
    }

    /// Get self public key
    pub fn get_public_key(&self) -> PublicKey {
        unsafe {
            let mut pk: PublicKey = mem::uninitialized();
            ll::tox_self_get_public_key(self.raw, &mut pk as *mut _ as *mut u8);
            pk
        }
    }
    // SELF INFO
    // END OF SELF INFO

    pub fn add_friend(&mut self, address: &Address, message: &str) -> Result<(), FriendAddError> {
        unsafe {
            let c_addr = address as *const _ as *const u8;
            tox_try!(
                err,
                ll::tox_friend_add(self.raw, c_addr, message.as_ptr(), message.len(), &mut err)
            );
        }
        Ok(())
    }

    pub fn add_friend_norequest(&mut self, address: &Address) -> Result<(), FriendAddError> {
        unsafe {
            let c_addr = address as *const _ as *const u8;
            tox_try!(err, ll::tox_friend_add_norequest(self.raw, c_addr, &mut err));
        }
        Ok(())
    }

    // Groupchats
/*
    pub fn add_groupchat(&mut self) -> Result<i32, ()> {
        unsafe {
            ok_or_minus!(ll::tox_add_groupchat(self.raw))
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
        some_or_minus!(len);
        String::from_utf8(vec).ok()
    }

    pub fn invite_friend(&mut self, friendnumber: i32, groupnumber: i32) -> Result<(), ()> {
        unsafe { ok_or_minus!(ll::tox_invite_friend(self.raw, friendnumber, groupnumber), ()) }
    }

    pub fn join_groupchat(&mut self, friendnumber: i32,
                      data: &[u8]) -> Result<i32, ()> {
        let res = unsafe {
            ll::tox_join_groupchat(self.raw, friendnumber, data.as_ptr(), data.len() as u16)
        };
        ok_or_minus!(res)
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

    pub fn group_get_type(&self, groupnumber: i32) -> Option<GroupchatType> {
        unsafe {
            some_or_minus!(ll::tox_group_get_type(self.raw, groupnumber))
                .map(|k| mem::transmute(k as u8))
        }
    }
*/
    pub fn save(&mut self) -> Vec<u8> {
        unimplemented!()
    }

    pub fn load(&mut self, data: &[u8]) -> Result<(), ()> {
        unimplemented!()
    }

    #[inline]
    #[doc(hidden)]
    pub unsafe fn from_raw_tox(raw: *mut ll::Tox) -> Tox {
        let mut tox: Tox = mem::zeroed();
        tox.raw = raw;
        tox
    }

    #[inline]
    #[doc(hidden)]
    pub unsafe fn raw(&mut self) -> *mut ll::Tox {
        self.raw
    }
}


macro_rules! parse_string {
    ($p:expr, $l:ident) => {
        unsafe {
            let slice = slice::from_raw_parts($p, $l as usize);
            match ::std::str::from_utf8(slice) {
                Ok(s) => s.to_string(),
                _ => return,
            }
        }
    }
}

// BEGIN: Callback pack
/*
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
    let msg = parse_string!(message, len);
    tx.send(GroupMessage(groupnumber, frindgroupnumber, msg)).unwrap();
}

extern fn on_group_action(_: *mut ll::Tox, groupnumber: i32, frindgroupnumber: i32,
        action: *const u8, len: u16, chan: *mut c_void) {
    let tx = unsafe { mem::transmute::<_, &mut Sender<Event>>(chan) };    
    let action = parse_string!(action, len);
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
*/
// END: Callback pack
