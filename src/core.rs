use std::sync::mpsc::{channel, Receiver, Sender};
use std::{slice, mem, ffi, fmt};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use std::thread::sleep;
use std::str::FromStr;

use libc::{c_uint, c_void};

// pub use self::ll::Tox as Tox_Struct;
pub use self::Event::*;
use self::errors::*;

mod ll;
pub mod errors;

pub const PUBLIC_KEY_SIZE:              usize = 32;
pub const SECRET_KEY_SIZE:              usize = 32;
pub const ADDRESS_SIZE:                 usize = PUBLIC_KEY_SIZE + 6;
// pub const MAX_NAME_LENGTH:              usize = 128;
// pub const MAX_STATUSMESSAGE_LENGTH:     usize = 1007;
// pub const MAX_FRIENDREQUEST_LENGTH:     usize = 1016;
// pub const MAX_MESSAGE_LENGTH:           usize = 1372;
// pub const MAX_CUSTOM_PACKET_SIZE:       usize = 1367;
// pub const HASH_LENGTH:                  usize = 32;
// pub const FILE_ID_LENGTH:               usize = 32;
// pub const MAX_FILENAME_LENGTH:          usize = 255;
pub const CONFERENCE_ID_SIZE:       usize = 32;
pub const FILE_ID_LENGTH:           usize = 32;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UserStatus {
    None = 0,
    Away = 1,
    Busy = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MessageType {
    Normal = 0,
    Action = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ProxyType {
    None = 0,
    Http = 1,
    Socks5 = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SavedataType {
    None = 0,
    ToxSave = 1,
    SecretKey = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Connection {
    None = 0,
    Tcp = 1,
    Udp = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FileKind {
    Data = 0,
    Avatar = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FileControl {
    Resume = 0,
    Pause = 1,
    Cancel = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FileId {
    raw: [u8; FILE_ID_LENGTH],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConferenceType {
    Text = 0,
    Av = 1,
}

pub struct ConferenceId {
    raw: [u8; CONFERENCE_ID_SIZE]
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cookie {
    raw: Vec<u8>
}

impl Cookie {
    pub fn into_bytes(self) -> Vec<u8> {
        self.raw
    }

    pub fn from_bytes(bytes: &[u8]) -> Cookie {
        Cookie {
            raw: bytes.to_owned()
        }
    }
}

/// A Tox address consist of `PublicKey`, nospam and checksum
#[repr(C)]
#[derive(PartialEq, Clone, Debug)]
pub struct Address {
    key: PublicKey,
    nospam: [u8; 4],
    // #[allow(dead_code)]
    checksum: [u8; 2],
}

impl Address {
    pub fn public_key(&self) -> &PublicKey {
        &self.key
    }
    fn checksum(&self) -> [u8; 2] {
        let mut check = [0u8, 0u8];
        for (i, &x) in self.key.raw.iter().enumerate() {
            check[i % 2] ^= x;
        }
        for i in 0..4 {
            check[(PUBLIC_KEY_SIZE + i) % 2] ^= self.nospam[i];
        }
        check
    }
}

impl fmt::Display for Address {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.key.fmt(fmt)?;
        write!(fmt, "{:02X}", self.nospam[0])?;
        write!(fmt, "{:02X}", self.nospam[1])?;
        write!(fmt, "{:02X}", self.nospam[2])?;
        write!(fmt, "{:02X}", self.nospam[3])?;
        let check = self.checksum();
        write!(fmt, "{:02X}", check[0])?;
        write!(fmt, "{:02X}", check[1])?;
        Ok(())
    }
}

impl FromStr for Address {
    type Err = ();
    fn from_str(s: &str) -> Result<Address, ()> {
        if s.len() != 2 * ADDRESS_SIZE {
            return Err(());
        }

        let mut key    = [0u8; 32];
        let mut nospam = [0u8; 4];
        let mut check  = [0u8; 2];

        if parse_hex(&s[0..2 * PUBLIC_KEY_SIZE], &mut key[..]).is_err() {
            return Err(());
        }
        if parse_hex(&s[2 * PUBLIC_KEY_SIZE..2 * PUBLIC_KEY_SIZE + 8],
           &mut nospam[..]).is_err() {
            return Err(());
        }
        if parse_hex(&s[2 * PUBLIC_KEY_SIZE + 8..2 * ADDRESS_SIZE],
           &mut check[..]).is_err() {
            return Err(());
        }

        let addr = Address { key: PublicKey { raw: key }, nospam: nospam, checksum: check };
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
pub struct PublicKey {
    pub raw: [u8; PUBLIC_KEY_SIZE],
}

impl fmt::Display for PublicKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for &n in self.raw.iter() {
            write!(fmt, "{:02X}", n)?;
        }
        Ok(())
    }
}

impl FromStr for PublicKey {
    type Err = ();
    fn from_str(s: &str) -> Result<PublicKey, ()> {
        if s.len() != 2 * PUBLIC_KEY_SIZE {
            return Err(());
        }

        let mut id = [0u8; PUBLIC_KEY_SIZE];

        parse_hex(s, &mut id[..])?;
        Ok(PublicKey { raw: id })
    }
}

pub struct SecretKey {
    pub raw: [u8; SECRET_KEY_SIZE],
}

impl fmt::Display for SecretKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for &n in self.raw.iter() {
            write!(fmt, "{:02X}", n)?;
        }

        Ok(())
    }
}

impl FromStr for SecretKey {
    type Err = ();
    fn from_str(s: &str) -> Result<SecretKey, ()> {
        if s.len() != 2 * SECRET_KEY_SIZE {
            return Err(());
        }

        let mut id = [0u8; SECRET_KEY_SIZE];

        parse_hex(s, &mut id[..])?;
        Ok(SecretKey { raw: id })
    }
}

/// Tox events enum
#[derive(Clone, Debug)]
pub enum Event {
    ConnectionStatus(Connection),
    FriendRequest(PublicKey, String),
    FriendMessage(u32, MessageType, String),
    FriendName(u32, String),
    FriendStatusMessage(u32, String),
    FriendStatus(u32, UserStatus),
    FriendConnectionStatus(u32, Connection),
    FriendTyping(u32, bool),
    FriendReadReceipt {
        friend: u32,
        message_id: u32,
    },

    FileControlReceive {
        friend: u32,
        file_number: u32,
        control: FileControl,
    },
    FileChunkRequest {
        friend: u32,
        file_number: u32,
        position: usize,
        length: usize,
    },
    FileReceive {
        friend: u32,
        file_number: u32,
        kind: u32,
        file_size: usize,
        file_name: String,
    },
    FileChunkReceive {
        friend: u32,
        file_number: u32,
        position: usize,
        data: Vec<u8>,
    },

    ConferenceInvite {
        friend: u32,
        kind: ConferenceType,
        cookie: Cookie,
    },
    ConferenceConnected {
        conference: u32
    },
    ConferenceMessage {
        conference: u32,
        peer: u32,
        kind: MessageType,
        message: String,
    },
    ConferenceTitle {
        conference: u32,
        peer: u32,
        title: String,
    },
    ConferencePeerName {
        conference: u32,
        peer: u32,
        name: String,
    },
    ConferencePeerListChanged {
        conference: u32
    },

    LossyPackage(u32, Vec<u8>),
    LosslessPackage(u32, Vec<u8>),
    /// ToxAV Event
    Call(u32, bool, bool),
    CallState(u32, u32),
    BitRateStatus(u32, u32, u32),
    AudioReceiveFrame(u32, Vec<i16>, usize, u8, u32),
    VideoReceiveFrame(u32, u16, u16, Vec<u8>, Vec<u8>, Vec<u8>, i32, i32, i32),
}

// #[repr(C)]
// #[derive(Copy, Clone, PartialEq, Eq, Debug)]
// pub enum ProxyType {
//     None = 0,
//     Socks5,
//     HTTP,
// }

// #[repr(C)]
// #[derive(Copy, Clone, PartialEq, Eq, Debug)]
// pub enum SavedataType {
//     None = 0,
//     ToxSave,
//     SecretKey,
// }

/**
    ToxOptions provides options that tox will be initalized with.
*/

//#[derive(Clone, Copy)]
pub struct ToxOptions {
    raw: ll::Tox_Options,
    sk_ptr: Option<*mut SecretKey>,
}

impl ToxOptions {
    /// Create a default ToxOptions struct
    pub fn new() -> ToxOptions {
        let raw_options = unsafe {
            let mut raw: ll::Tox_Options = std::mem::uninitialized();
            ll::tox_options_default(&mut raw);

            raw
        };

        ToxOptions {
            raw: raw_options,
            sk_ptr: None,
        }
    }

    /// Set the given SecretKey so you can restore your Tox
    pub fn set_secret_key(mut self, secret_key: SecretKey) -> ToxOptions {
        if self.sk_ptr.is_some() {
            panic!("SK has already been set");
        }
        let sk_ptr = Box::into_raw(Box::new(secret_key));
        self.sk_ptr = Some(sk_ptr);
        self.raw.savedata_type = SavedataType::SecretKey;
        self.raw.savedata_data = sk_ptr as *const _;
        self.raw.savedata_length = SECRET_KEY_SIZE;
        self
    }

    /// Enable ipv6
    pub fn ipv6(mut self) -> ToxOptions {
        self.raw.ipv6_enabled = true;
        self
    }

    /// Disable UDP
    pub fn no_udp(mut self) -> ToxOptions {
        self.raw.udp_enabled = false;
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

impl Drop for ToxOptions {
    fn drop(&mut self) {
        if let Some(sk_ptr) = self.sk_ptr {
            unsafe { Box::from_raw(sk_ptr) };
        }
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

macro_rules! tox_try {
    ($err:ident, $exp:expr) => {{
        let mut $err = ::std::mem::uninitialized();
        let res = $exp;
        match $err as c_uint {
            0 => {},
            _ => return Err($err),
        };
        res
    }};
}

macro_rules! tox_option {
    ($err:ident, $exp:expr) => {{
        let mut $err = ::std::mem::uninitialized();
        let res = $exp;
        match $err as c_uint {
            0 => {},
            _ => return None,
        };
        res
    }};
}

pub struct Tox {
    pub raw: *mut ll::Tox,
    pub event_tx: Box<Sender<Event>>,
    event_rx: Rc<RefCell<Receiver<Event>>>,
}

impl Drop for Tox {
    fn drop(&mut self) {
        unsafe { ll::tox_kill(self.raw); }
    }
}

impl Tox {
    /// Create a new tox instance
    pub fn new(mut opts: ToxOptions, data: Option<&[u8]>) -> Result<Tox, InitError> {
        let tox = unsafe {
            match data {
                Some(data) => {
                    opts.raw.savedata_type = SavedataType::ToxSave;
                    opts.raw.savedata_data = data.as_ptr();
                    opts.raw.savedata_length = data.len();
                    tox_try!(err, ll::tox_new(&opts.raw, &mut err))
                },
                None => {
                    tox_try!(err, ll::tox_new(&opts.raw, &mut err))
                }
            }
        };

        let (tx, rx) = channel::<Event>();
        let event_tx = Box::new(tx);
        let event_rx = Rc::new(RefCell::new(rx));

        unsafe {
            ll::tox_callback_self_connection_status(tox, Some(on_connection_status));
            ll::tox_callback_friend_request(tox, Some(on_friend_request));
            ll::tox_callback_friend_message(tox, Some(on_friend_message));

            ll::tox_callback_friend_name(tox, Some(on_friend_name));
            ll::tox_callback_friend_status_message(tox, Some(on_friend_status_message));
            ll::tox_callback_friend_status(tox, Some(on_friend_status));
            ll::tox_callback_friend_connection_status(tox, Some(on_friend_connection_status));
            ll::tox_callback_friend_typing(tox, Some(on_friend_typing));
            ll::tox_callback_friend_read_receipt(tox, Some(on_friend_read_receipt));

            ll::tox_callback_file_recv_control(tox, Some(on_file_control));
            ll::tox_callback_file_chunk_request(tox, Some(on_file_chunk_request));
            ll::tox_callback_file_recv(tox, Some(on_file_receive));
            ll::tox_callback_file_recv_chunk(tox, Some(on_file_chunk_receive));

            ll::tox_callback_conference_invite(tox, Some(on_conference_invite));
            ll::tox_callback_conference_connected(tox, Some(on_conference_connected));
            ll::tox_callback_conference_message(tox, Some(on_conference_message));
            ll::tox_callback_conference_title(tox, Some(on_conference_title));
            ll::tox_callback_conference_peer_name(tox, Some(on_conference_peer_name));
            ll::tox_callback_conference_peer_list_changed(
                tox,
                Some(on_conference_peer_list_changed)
            );

            ll::tox_callback_friend_lossy_packet(tox, Some(on_lossy_package));
            ll::tox_callback_friend_lossless_packet(tox, Some(on_lossless_package));
        }

        Ok(Tox {
            raw: tox,
            event_tx,
            event_rx,
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
        unsafe {
            let chan = (&mut *self.event_tx) as *mut _ as *mut _;
            ll::tox_iterate(self.raw, chan);
        }
    }

    /// This function makes thread sleep for a some time, optimal for `tick()` method
    pub fn wait(&self) {
        unsafe {
            let delay = ll::tox_iteration_interval(self.raw);
            sleep(Duration::from_millis(delay as u64));
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
        Panics if `host` string contains `\0`.
    */
    pub fn bootstrap(&mut self, host: &str, port: u16, public_key: PublicKey) -> Result<(), BootstrapError> {
        unsafe {
            let c_host = ffi::CString::new(host).unwrap();
            let c_pk: *const u8 = &public_key as *const _ as *const _;
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

    /// Get secret_key
    pub fn get_secret_key(&self) -> SecretKey {
        unsafe {
            let mut sk: SecretKey = mem::uninitialized();
            ll::tox_self_get_secret_key(self.raw, &mut sk as *mut _ as *mut u8);
            sk
        }
    }

    /// Set the nickname for the Tox client
    pub fn set_name(&mut self, name: &str) -> Result<(), SetInfoError> {
        unsafe {
            tox_try!(err, ll::tox_self_set_name(self.raw, name.as_ptr(), name.len(), &mut err));
        }
        Ok(())
    }

    /// Get self nickname
    pub fn get_name(&self) -> String {
        unsafe {
            let len = ll::tox_self_get_name_size(self.raw);
            let mut bytes: Vec<u8> = Vec::with_capacity(len);
            bytes.set_len(len);
            ll::tox_self_get_name(self.raw, bytes.as_mut_ptr());
            String::from_utf8_unchecked(bytes)
        }
    }

    /// Set self status message
    pub fn set_status_message(&mut self, message: &str) -> Result<(), SetInfoError> {
        unsafe {
            tox_try!(err, ll::tox_self_set_status_message(self.raw, message.as_ptr(), message.len(), &mut err));
        }
        Ok(())
    }

    /// Get self status message
    pub fn get_status_message(&self) -> String {
        unsafe {
            let len = ll::tox_self_get_status_message_size(self.raw);
            let mut bytes: Vec<u8> = Vec::with_capacity(len);
            bytes.set_len(len);
            ll::tox_self_get_status_message(self.raw, bytes.as_mut_ptr());
            String::from_utf8_unchecked(bytes)
        }
    }

    /// Set self status
    pub fn set_status(&mut self, status: UserStatus) {
        unsafe { ll::tox_self_set_status(self.raw, status); }
    }

    /// Get self status
    pub fn get_status(&self) -> UserStatus {
        unsafe { ll::tox_self_get_status(self.raw) }
    }


    /**
        Add a friend to the friend list and send a friend request.

        A friend request message must be at least 1 byte long and at most
        `TOX_MAX_FRIEND_REQUEST_LENGTH`.

        Friend numbers are unique identifiers used in all functions that operate on
        friends. Once added, a friend number is stable for the lifetime of the Tox
        object. After saving the state and reloading it, the friend numbers may not
        be the same as before. Deleting a friend creates a gap in the friend number
        set, which is filled by the next adding of a friend. Any pattern in friend
        numbers should not be relied on.
    */
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

    /**
        Add a friend without sending a friend request.

        This function is used to add a friend in response to a friend request. If the
        client receives a friend request, it can be reasonably sure that the other
        client added this client as a friend, eliminating the need for a friend
        request.

        This function is also useful in a situation where both instances are
        controlled by the same entity, so that this entity can perform the mutual
        friend adding. In this case, there is no need for a friend request, either.
    */
    pub fn add_friend_norequest(&mut self, address: &PublicKey) -> Result<(), FriendAddError> {
        unsafe {
            let c_addr = address as *const _ as *const u8;
            tox_try!(err, ll::tox_friend_add_norequest(self.raw, c_addr, &mut err));
        }
        Ok(())
    }

    /**
        Remove a friend from the friend list.

        This does not notify the friend of their deletion. After calling this
        function, this client will appear offline to the friend and no communication
        can occur between the two.
    */
    pub fn delete_friend(&mut self, fnum: u32) -> Result<(), ()> {
        unsafe {
            let mut err: ll::TOX_ERR_FRIEND_DELETE = mem::uninitialized();
            if !ll::tox_friend_delete(self.raw, fnum, &mut err as *mut _) {
                return Err(())
            }
        }
        Ok(())
    }

    // FRIEND STUFF
    pub fn friend_by_public_key(&self, public_key: PublicKey) -> Option<u32> {
        unsafe {
            let pk: *const u8 = &public_key as *const _ as *const _;
            let fnum = tox_option!(err, ll::tox_friend_by_public_key(self.raw, pk, &mut err));
            Some(fnum)
        }
    }

    pub fn friend_exists(&self, fnum: u32) -> bool {
        unsafe {
            ll::tox_friend_exists(self.raw, fnum)
        }
    }

    pub fn get_friend_list(&self) -> Vec<u32> {
        unsafe {
            let len = ll::tox_self_get_friend_list_size(self.raw);
            let mut list = Vec::with_capacity(len);
            list.set_len(len);
            ll::tox_self_get_friend_list(self.raw, list.as_mut_ptr());
            list
        }
    }

    pub fn get_friend_public_key(&self, fnum: u32) -> Option<PublicKey> {
        unsafe {
            let mut public_key: PublicKey = mem::uninitialized();
            let pk: *mut u8 = &mut public_key as *mut _ as *mut _;
            tox_option!(err, ll::tox_friend_get_public_key(self.raw, fnum, pk, &mut err));
            Some(public_key)
        }
    }

    /**
        Returns an `Option<u64>` with number of seconds since January 1, 1970
        0:00:00 UTC (aka "UNIX timestamp").

        In case where there is no friend with supplied `fnum`, `None` is
        returned.
    */
    pub fn get_friend_last_online(&self, fnum: u32) -> Option<u64> {
        unsafe {
            Some(tox_option!(err, ll::tox_friend_get_last_online(self.raw, fnum, &mut err)))
        }
    }

    /// Returns friend name, or, if friend doesn't exist, `None`.
    pub fn get_friend_name(&self, fnum: u32) -> Option<String> {
        unsafe {
            let len = tox_option!(err, ll::tox_friend_get_name_size(self.raw,
                                fnum, &mut err));
            let mut bytes: Vec<u8> = Vec::with_capacity(len);
            bytes.set_len(len);
            tox_option!(err, ll::tox_friend_get_name(self.raw, fnum,
                    bytes.as_mut_ptr(), &mut err));
            Some(String::from_utf8_unchecked(bytes))
        }
    }

    /// Returns status message of a friend, or, if friend doesn't exist, `None`.
    pub fn get_friend_status_message(&self, fnum: u32) -> Option<String> {
        unsafe {
            let len = tox_option!(err, ll::tox_friend_get_status_message_size(self.raw,
                                fnum, &mut err));
            let mut bytes: Vec<u8> = Vec::with_capacity(len);
            bytes.set_len(len);
            tox_option!(err, ll::tox_friend_get_status_message(self.raw, fnum,
                    bytes.as_mut_ptr(), &mut err));
            Some(String::from_utf8_unchecked(bytes))
        }
    }

    /// Returns friend status, or, if there is an error, `None`.
    pub fn get_friend_status(&self, fnum: u32) -> Option<UserStatus> {
        unsafe {
            Some(tox_option!(err, ll::tox_friend_get_status(self.raw, fnum, &mut err)))
        }
    }

    /// Return status of connection of friend, or if friend doesn't exist, `None`.
    pub fn get_friend_connection_status(&self, fnum: u32) -> Option<Connection> {
        unsafe {
            Some(tox_option!(err, ll::tox_friend_get_connection_status(self.raw, fnum, &mut err)))
        }
    }
    // END OF FRIEND STUFF
    /**
        Send a text chat message to an online friend.

        This function creates a chat message packet and pushes it into the send
        queue.

        The message length may not exceed `TOX_MAX_MESSAGE_LENGTH`. Larger messages
        must be split by the client and sent as separate messages. Other clients can
        then reassemble the fragments. Messages may not be empty.

        The return value of this function is the message ID. If a read receipt is
        received, the triggered `friend_read_receipt` event will be passed this message ID.

        Message IDs are unique per friend. The first message ID is 0. Message IDs are
        incremented by 1 each time a message is sent. If `UINT32_MAX` messages were
        sent, the next message ID is 0.
    */
    pub fn send_friend_message(
        &mut self, fnum: u32, kind: MessageType, message: &str
    ) -> Result<u32, FriendSendMessageError> {
        let msg_id = unsafe {
            tox_try!(
                err,
                ll::tox_friend_send_message(self.raw, fnum, kind, message.as_ptr(), message.len(), &mut err)
            )
        };
        Ok(msg_id)
    }

    pub fn control_file(
        &mut self,
        friend: u32,
        file_number: u32,
        control: FileControl
    ) -> Result<(), FileControlError> {
        unsafe {
            tox_try!(err, ll::tox_file_control(
                self.raw,
                friend,
                file_number,
                control,
                &mut err
            ));

            Ok(())
        }
    }

    pub fn seek_file(
        &mut self,
        friend: u32,
        file_number: u32,
        postition: usize
    ) -> Result<(), FileSeekError> {
        unsafe {
            tox_try!(err, ll::tox_file_seek(
                self.raw,
                friend,
                file_number,
                postition as u64,
                &mut err
            ));

            Ok(())
        }
    }

    pub fn get_file_id(
        &mut self,
        friend: u32,
        file_number: u32,
    ) -> Result<FileId, FileGetError> {
        unsafe {
            let mut raw: [u8; FILE_ID_LENGTH] = mem::uninitialized();

            tox_try!(err, ll::tox_file_get_file_id(
                self.raw,
                friend,
                file_number,
                raw.as_mut_ptr(),
                &mut err
            ));

            Ok(FileId {
                raw
            })
        }
    }

    pub fn send_file(
        &mut self,
        friend: u32,
        kind: FileKind,
        file_size: usize,
        file_name: &str,
    ) -> Result<u32, FileSendError> {
        unsafe {
            let file_number = tox_try!(err, ll::tox_file_send(
                self.raw,
                friend,
                kind as u32,
                file_size as u64,
                std::ptr::null(),
                file_name.as_ptr(),
                file_name.len(),
                &mut err
            ));

            Ok(file_number)
        }
    }

    pub fn send_file_chunk(
        &mut self,
        friend: u32,
        file_number: u32,
        position: usize,
        data: &[u8]
    ) -> Result<(), FileSendChunkError> {
        unsafe {
            tox_try!(err, ll::tox_file_send_chunk(
                self.raw,
                friend,
                file_number,
                position as u64,
                data.as_ptr(),
                data.len(),
                &mut err
            ));

            Ok(())
        }
    }

    // Conference stuff

    pub fn new_conference(&mut self) -> Result<u32, ()> {
        unsafe {
            let mut err = ::std::mem::uninitialized();
            let res = ll::tox_conference_new(self.raw, &mut err);

            match err as c_uint {
                0 => return Ok(res),
                _ => return Err(()),
            };
        }
    }

    pub fn delete_conference(&mut self, conference_number: u32) -> Option<()> {
        unsafe {
            tox_option!(err, ll::tox_conference_delete(
                self.raw,
                conference_number,
                &mut err
            ));
            Some(())
        }
    }

    pub fn conference_peer_count(
        &mut self, conference_number: u32
    )-> Result<u32, ConferencePeerQueryError> {
        unsafe {
            let count = tox_try!(err, ll::tox_conference_peer_count(
                self.raw(),
                conference_number,
                &mut err
            ));

            Ok(count)
        }
    }

    pub fn get_peer_name(
        &mut self,
        conference_number: u32,
        peer_number: u32
    ) -> Result<String, ConferencePeerQueryError> {
        unsafe {
            let size = tox_try!(err, ll::tox_conference_peer_get_name_size(
                self.raw,
                conference_number,
                peer_number,
                &mut err
            ));

            let mut name = Vec::with_capacity(size);

            tox_try!(err, ll::tox_conference_peer_get_name(
                self.raw,
                conference_number,
                peer_number,
                name.as_mut_ptr(),
                &mut err
            ));

            Ok(String::from_utf8_unchecked(name))
        }
    }

    pub fn get_peer_public_key(
        &mut self,
        conference_number: u32,
        peer_number: u32,
    ) -> Result<PublicKey, ConferencePeerQueryError> {
        unsafe {
            let mut raw: [u8; PUBLIC_KEY_SIZE] = mem::uninitialized();

            tox_try!(err, ll::tox_conference_peer_get_public_key(
                self.raw,
                conference_number,
                peer_number,
                raw.as_mut_ptr(),
                &mut err
            ));

            Ok(PublicKey {
                raw
            })
        }
    }

    pub fn is_own_peer_number(
        &mut self,
        conference_number: u32,
        peer_number: u32
    ) -> Result<bool, ConferencePeerQueryError> {
        unsafe {
            let is_ours =
                tox_try!(err, ll::tox_conference_peer_number_is_ours(
                    self.raw,
                    conference_number,
                    peer_number,
                    &mut err
                ));

            Ok(is_ours)
        }
    }

    pub fn conference_offline_peer_count(
        &mut self,
        conference_number: u32
    ) -> Result<u32, ConferencePeerQueryError> {
        unsafe {
            let count = tox_try!(err, ll::tox_conference_offline_peer_count(
                self.raw,
                conference_number,
                &mut err
            ));

            Ok(count)
        }
    }

    pub fn get_offline_peer_name(
        &mut self,
        conference_number: u32,
        peer_number: u32,
    ) -> Result<String, ConferencePeerQueryError> {
        unsafe {
            let size = tox_try!(err, ll::tox_conference_offline_peer_get_name_size(
                self.raw,
                conference_number,
                peer_number,
                &mut err
            ));

            let mut name = Vec::with_capacity(size);

            tox_try!(err, ll::tox_conference_offline_peer_get_name(
                self.raw,
                conference_number,
                peer_number,
                name.as_mut_ptr(),
                &mut err
            ));

            Ok(String::from_utf8_unchecked(name))
        }
    }

    pub fn get_offline_peer_public_key(
        &mut self,
        conference_number: u32,
        peer_number: u32
    ) -> Result<PublicKey, ConferencePeerQueryError> {
        unsafe {
            let mut raw: [u8; PUBLIC_KEY_SIZE] = mem::uninitialized();

            tox_try!(err, ll::tox_conference_peer_get_public_key(
                self.raw,
                conference_number,
                peer_number,
                raw.as_mut_ptr(),
                &mut err
            ));

            Ok(PublicKey {
                raw
            })
        }
    }

    pub fn get_offline_peer_last_active(
        &mut self,
        conference_number: u32,
        peer_number: u32,
    ) -> Result<u64, ConferencePeerQueryError> {
        unsafe {
            let time = tox_try!(err, ll::tox_conference_offline_peer_get_last_active(
                self.raw,
                conference_number,
                peer_number,
                &mut err
            ));

            Ok(time)
        }
    }

    pub fn invite_to_conference(
        &mut self,
        friend_number: u32,
        conference_number: u32
    ) -> Result<(), ConferenceInviteError> {
        unsafe {
            tox_try!(err, ll::tox_conference_invite(
                self.raw,
                friend_number,
                conference_number,
                &mut err
            ));

            Ok(())
        }
    }

    pub fn join_conference(
        &mut self,
        friend_number: u32,
        cookie: &Cookie,
    ) -> Result<u32, ConferenceJoinError> {
        unsafe {
            let conference = tox_try!(err, ll::tox_conference_join(
                self.raw,
                friend_number,
                cookie.raw.as_ptr(),
                cookie.raw.len(),
                &mut err
            ));

            Ok(conference)
        }
    }

    pub fn send_conference_message(
        &mut self,
        conference_number: u32,
        kind: MessageType,
        message: &str
    ) -> Result<(), ConferenceSendError> {
        unsafe {
            let msg = message.as_ptr();
            let len = message.len();

            tox_try!(err, ll::tox_conference_send_message(
                self.raw,
                conference_number,
                kind,
                msg,
                len,
                &mut err
            ));

            Ok(())
        }
    }

    pub fn get_conference_title(
        &mut self,
        conference_number: u32
    ) -> Result<String, ConferenceTitleError> {
        unsafe {
            let len = tox_try!(err, ll::tox_conference_get_title_size(
                self.raw,
                conference_number,
                &mut err
            ));

            let mut title = Vec::with_capacity(len);

            tox_try!(err, ll::tox_conference_get_title(
                self.raw,
                conference_number,
                title.as_mut_ptr(),
                &mut err
            ));

            Ok(String::from_utf8_unchecked(title))
        }
    }

    pub fn set_conference_title(
        &mut self,
        conference_number: u32,
        title: &str
    ) -> Result<(), ConferenceTitleError> {
        unsafe {
            let len = title.len();

            tox_try!(err, ll::tox_conference_set_title(
                self.raw,
                conference_number,
                title.as_ptr(),
                len,
                &mut err
            ));

            Ok(())
        }
    }

    pub fn get_chatlist(&mut self) -> Vec<u32> {
        unsafe {
            let len = ll::tox_conference_get_chatlist_size(
                self.raw
            );

            let mut chatlist = Vec::with_capacity(len);

            ll::tox_conference_get_chatlist(
                self.raw,
                chatlist.as_mut_ptr()
            );

            chatlist
        }
    }

    pub fn get_conference_type(
        &mut self,
        conference_number: u32
    ) -> Option<ConferenceType> {
        unsafe {
            let kind = tox_option!(err, ll::tox_conference_get_type(
                self.raw,
                conference_number,
                &mut err
            ));

            Some(kind)
        }
    }

    pub fn get_conference_id(
        &mut self,
        conference_number: u32
    ) -> Option<ConferenceId> {
        unsafe {
            let mut raw = [0; CONFERENCE_ID_SIZE];

            let exists = ll::tox_conference_get_id(
                self.raw,
                conference_number,
                raw.as_mut_ptr(),
            );

            if exists {
                Some(ConferenceId{
                    raw,
                })
            }
            else {
                None
            }
        }
    }

    pub fn conference_by_id(
        &mut self,
        id: &ConferenceId
    ) -> Option<u32> {
        unsafe {
            let conf_num = tox_option!(err, ll::tox_conference_by_id(
                self.raw,
                id.raw.as_ptr(),
                &mut err
            ));

            Some(conf_num)
        }
    }

    /// Get all all information associated with the tox instance as a `Vec<u8>`
    pub fn save(&self) -> Vec<u8> {
        unsafe {
            let len = ll::tox_get_savedata_size(self.raw);
            let mut data = Vec::with_capacity(len);
            data.set_len(len);
            ll::tox_get_savedata(self.raw, data.as_mut_ptr());
            data
        }
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

// BEGIN: Callback pack

extern fn on_connection_status(_: *mut ll::Tox, status: Connection, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        tx.send(ConnectionStatus(status)).unwrap();
    }
}

extern fn on_friend_request(_: *mut ll::Tox, public_key: *const u8, message: *const u8, length: usize, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        let pk: &PublicKey = &*(public_key as *const _);
        let message = String::from_utf8_lossy(slice::from_raw_parts(message, length)).into_owned();
        tx.send(FriendRequest(*pk, message)).unwrap();
    }
}

extern fn on_friend_message(_: *mut ll::Tox, fnum: u32, kind: MessageType,
        message: *const u8, length: usize, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        let message = String::from_utf8_lossy(slice::from_raw_parts(message, length)).into_owned();
        tx.send(FriendMessage(fnum, kind, message)).unwrap();

    }
}

extern fn on_friend_name(_: *mut ll::Tox, fnum: u32, name: *const u8, length: usize, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        let name = String::from_utf8_lossy(slice::from_raw_parts(name, length)).into_owned();
        tx.send(FriendName(fnum, name)).unwrap();
    }
}

extern fn on_friend_status_message(_: *mut ll::Tox, fnum: u32, message: *const u8, length: usize, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        let message = String::from_utf8_lossy(slice::from_raw_parts(message, length)).into_owned();
        tx.send(FriendStatusMessage(fnum, message)).unwrap();
    }
}

extern fn on_friend_status(_: *mut ll::Tox, fnum: u32, status: UserStatus, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        tx.send(FriendStatus(fnum, status)).unwrap();
    }
}

extern fn on_friend_connection_status(_: *mut ll::Tox, fnum: u32, status: Connection, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        tx.send(FriendConnectionStatus(fnum, status)).unwrap();
    }
}

extern fn on_friend_typing(_: *mut ll::Tox, fnum: u32, is_typing: bool, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        tx.send(FriendTyping(fnum, is_typing)).unwrap();
    }
}

extern fn on_friend_read_receipt(
    _: *mut ll::Tox,
    friend: u32,
    message_id: u32,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        tx.send(FriendReadReceipt {
            friend, message_id
        }).unwrap();
    }
}

// File transfer

extern fn on_file_control(
    _: *mut ll::Tox,
    friend: u32,
    file_number: u32,
    control: FileControl,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        tx.send(FileControlReceive {
            friend,
            file_number,
            control
        }).unwrap()
    }
}

extern fn on_file_chunk_request(
    _: *mut ll::Tox,
    friend: u32,
    file_number: u32,
    position: u64,
    length: usize,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        tx.send(FileChunkRequest {
            friend,
            file_number,
            position: position as usize,
            length: length as usize,
        }).unwrap()
    }
}

extern fn on_file_receive(
    _: *mut ll::Tox,
    friend: u32,
    file_number: u32,
    kind: u32,
    file_size: u64,
    file_name: *const u8,
    file_name_size: usize,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        let file_name =
            String::from_utf8_lossy(
                slice::from_raw_parts(file_name, file_name_size)
            )
            .into_owned();
        tx.send(FileReceive {
            friend,
            file_number,
            kind,
            file_size: file_size as usize,
            file_name
        }).unwrap()
    }
}

extern fn on_file_chunk_receive(
    _: *mut ll::Tox,
    friend: u32,
    file_number: u32,
    position: u64,
    data: *const u8,
    data_len: usize,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        let data = Vec::from(slice::from_raw_parts(data, data_len));
        tx.send(FileChunkReceive {
            friend,
            file_number,
            position: position as usize,
            data
        }).unwrap()
    }
}

// Conference callbacks

extern fn on_conference_invite(
    _: *mut ll::Tox,
    friend: u32,
    kind: ConferenceType,
    cookie: *const u8,
    cookie_len: usize,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        let cookie = Cookie {
            raw: slice::from_raw_parts(cookie, cookie_len).into()
        };
        tx.send(ConferenceInvite {
            friend, kind, cookie,
        }).unwrap()
    }
}

extern fn on_conference_connected(
    _: *mut ll::Tox,
    conference: u32,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        tx.send(ConferenceConnected {
            conference
        }).unwrap();
    }
}

extern fn on_conference_message(
    _: *mut ll::Tox,
    conference: u32,
    peer: u32,
    kind: MessageType,
    message: *const u8,
    len: usize,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);

        let message =
            String::from_utf8_lossy(slice::from_raw_parts(message, len))
            .into_owned();

        tx.send(ConferenceMessage {
            conference, peer, kind, message
        }).unwrap();
    }
}

extern fn on_conference_title(
    _: *mut ll::Tox,
    conference: u32,
    peer: u32,
    title: *const u8,
    len: usize,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);

        let title =
            String::from_utf8_lossy(slice::from_raw_parts(title, len))
            .into_owned();

        tx.send(ConferenceTitle {
            conference, peer, title
        }).unwrap();
    }
}

extern fn on_conference_peer_name(
    _: *mut ll::Tox,
    conference: u32,
    peer: u32,
    name: *const u8,
    len: usize,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);

        let name =
            String::from_utf8_lossy(slice::from_raw_parts(name, len))
            .into_owned();

        tx.send(ConferencePeerName {
            conference, peer, name
        }).unwrap();
    }
}

extern fn on_conference_peer_list_changed(
    _: *mut ll::Tox,
    conference: u32,
    chan: *mut c_void
) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        tx.send(ConferencePeerListChanged {
            conference
        }).unwrap();
    }
}

extern fn on_lossy_package(_: *mut ll::Tox, fnum: u32, data: *const u8, length: usize, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        let data: Vec<u8> = From::from(slice::from_raw_parts(data, length as usize));
        tx.send(LossyPackage(fnum, data)).unwrap();
    }
}
extern fn on_lossless_package(_: *mut ll::Tox, fnum: u32, data: *const u8, length: usize, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = &mut *(chan as *mut _);
        let data: Vec<u8> = From::from(slice::from_raw_parts(data, length as usize));
        tx.send(LosslessPackage(fnum, data)).unwrap();
    }
}

// END: Callback pack
