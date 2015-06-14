use std::sync::mpsc::{channel, Receiver, Sender};
use std::{slice, mem, ffi, ptr, fmt};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::{sleep_ms};
use std::str::FromStr;

use libc::{c_uint, c_void};

pub use self::ll::Tox as Tox_Struct;
pub use self::Event::*;
use self::errors::*;

mod ll;
pub mod errors;

pub const PUBLIC_KEY_SIZE:              usize = 32;
pub const SECRET_KEY_SIZE:              usize = 32;
pub const ADDRESS_SIZE:                 usize = PUBLIC_KEY_SIZE + 6;
pub const MAX_NAME_LENGTH:              usize = 128;
pub const MAX_STATUSMESSAGE_LENGTH:     usize = 1007;
pub const MAX_FRIENDREQUEST_LENGTH:     usize = 1016;
pub const MAX_MESSAGE_LENGTH:           usize = 1372;
pub const MAX_CUSTOM_PACKET_SIZE:       usize = 1367;
pub const HASH_LENGTH:                  usize = 32;
pub const FILE_ID_LENGTH:               usize = 32;
pub const MAX_FILENAME_LENGTH:          usize = 255;


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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UserStatus {
    None = 0,
    Away,
    Busy,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MessageType {
    Normal = 0,
    Action,
}

#[repr(u8)]
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
pub enum FileKind {
    Data = 0,
    Avatar,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FileControl {
    Resume = 0,
    Pause,
    Cancel,
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
        try!(self.key.fmt(fmt));
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

        let mut key     = [0u8; 32];
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
            try!(write!(fmt, "{:02X}", n));
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
    ConnectionStatus(Connection),
    FriendRequest(PublicKey, String),
    FriendMessage(u32, MessageType, String),
    LossyPackage(u32, Vec<u8>),
    LosslessPackage(u32, Vec<u8>),
    GroupInvite(i32, GroupchatType, Vec<u8>),
    /// `(gnum, pnum, msg)` where `gnum` is the group number, `pnum` is the peer number
    /// and `msg` is the message
    GroupMessage(i32, i32, String),
    GroupTitle(i32, i32, String),
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SavedataType {
    None = 0,
    ToxSave,
    SecretKey,
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
    pub fn new(mut opts: ToxOptions, data: Option<&[u8]>) -> Result<Tox, InitError> {
        let tox = unsafe {
            match data {
                Some(data) => {
                    opts.txo.savedata_type = SavedataType::ToxSave;
                    opts.txo.savedata_data = data.as_ptr();
                    opts.txo.savedata_length = data.len();
                    tox_try!(err, ll::tox_new(&opts.txo, &mut err))
                },
                None => {
                    opts.txo.savedata_type = SavedataType::None;
                    opts.txo.savedata_data = ptr::null();
                    opts.txo.savedata_length = 0;
                    tox_try!(err, ll::tox_new(&opts.txo, &mut err))
                }
            }
        };

        let (tx, rx) = channel::<Event>();
        let mut btx = Box::new(tx);
        let rrrx = Rc::new(RefCell::new(rx)); // too much bloat to just get a channel, eh?
        unsafe {
            let chan: *mut c_void = mem::transmute(&mut *btx);

            ll::tox_callback_self_connection_status(tox, on_connection_status, chan);
            ll::tox_callback_friend_request(tox, on_friend_request, chan);
            ll::tox_callback_friend_message(tox, on_friend_message, chan);

            ll::tox_callback_friend_lossy_packet(tox, on_lossy_package, chan);
            ll::tox_callback_friend_lossless_packet(tox, on_lossless_package, chan);

            ll::tox_callback_group_invite(tox, on_group_invite, chan);
            ll::tox_callback_group_message(tox, on_group_message, chan);
            ll::tox_callback_group_action(tox, on_group_action, chan);
            ll::tox_callback_group_title(tox, on_group_title, chan);
            ll::tox_callback_group_namelist_change(tox, on_group_namelist_change, chan);
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
        Panics if `host` string contains `\0`.
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
            let mut err: ::libc::c_uint = mem::uninitialized();
            if ll::tox_friend_delete(self.raw, fnum, &mut err as *mut _) == 0 {
                return Err(())
            }
        }
        Ok(())
    }

    // FRIEND STUFF
    pub fn friend_by_public_key(&self, public_key: PublicKey) -> Option<u32> {
        unsafe {
            let pk: *const u8 = mem::transmute(&public_key);
            let fnum = tox_option!(err, ll::tox_friend_by_public_key(self.raw, pk, &mut err));
            Some(fnum)
        }
    }
    pub fn friend_exists(&self, fnum: u32) -> bool {
        unsafe {
            ll::tox_friend_exists(self.raw, fnum) != 0
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
            let public_key: PublicKey = mem::uninitialized();
            let pk: *mut u8 = mem::transmute(&public_key);
            tox_option!(err, ll::tox_friend_get_public_key(self.raw, fnum, pk, &mut err));
            Some(public_key)
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
    pub fn send_friend_message(&mut self, fnum: u32, kind: MessageType, message: &str) -> Result<u32, FriendSendMessageError> {
        let msg_id = unsafe {
            tox_try!(
                err,
                ll::tox_friend_send_message(self.raw, fnum, kind, message.as_ptr(), message.len(), &mut err)
            )
        };
        Ok(msg_id)
    }

    // BEGIN of old ugly groupchat stuff

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
        unsafe {
            let len = ll::tox_group_peername(&*self.raw, groupnumber, peernumber, vec.as_mut_ptr());
            some_or_minus!(len);
            vec.set_len(len as usize);
        };
        String::from_utf8(vec).ok()
    }

    pub fn group_peer_pubkey(&self, groupnumber: i32, peernumber: i32) -> Option<PublicKey> {
        let res = unsafe {
            let mut pk: PublicKey = mem::uninitialized();
            some_or_minus!(ll::tox_group_peer_pubkey(&*self.raw, groupnumber, peernumber,
                                &mut pk as *mut _ as *mut u8));
            pk
        };
        Some(res)
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

    pub fn group_get_title(&self, groupnumber: i32) -> Option<String> {
        unsafe {
            let tox: *mut ll::Tox = mem::transmute(self.raw);
            let mut title: Vec<u8> = Vec::with_capacity(128);
            let len = some_or_minus!(ll::tox_group_get_title(tox, groupnumber, title.as_mut_ptr(), 128));
            title.set_len(len.unwrap() as usize);
            Some(String::from_utf8_unchecked(title))
        }
    }

    pub fn group_set_title(&mut self, groupnumber: i32, title: &str) -> Result<(),()> {
        if title.len() > 128 { return Err(()) }
        unsafe {
             ok_or_minus!(ll::tox_group_set_title(self.raw, groupnumber, title.as_ptr(), title.len() as u8), ())
        }
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

    // END of old ugly groupchat stuff

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

extern fn on_connection_status(_: *mut ll::Tox, status: Connection, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        tx.send(ConnectionStatus(status)).unwrap();
    }
}

extern fn on_friend_request(_: *mut ll::Tox, public_key: *const u8, message: *const u8, length: usize, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        let pk: &PublicKey = mem::transmute(public_key);
        let message = String::from_utf8_lossy(slice::from_raw_parts(message, length)).into_owned();
        tx.send(FriendRequest(*pk, message)).unwrap();
    }
}

extern fn on_friend_message(_: *mut ll::Tox, fnum: u32, kind: MessageType,
        message: *const u8, length: usize, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        let message = String::from_utf8_lossy(slice::from_raw_parts(message, length)).into_owned();
        tx.send(FriendMessage(fnum, kind, message)).unwrap();

    }
}

extern fn on_lossy_package(_: *mut ll::Tox, fnum: u32, data: *const u8, length: usize, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        let data: Vec<u8> = From::from(slice::from_raw_parts(data, length as usize));
        tx.send(LossyPackage(fnum, data)).unwrap();
    }
}
extern fn on_lossless_package(_: *mut ll::Tox, fnum: u32, data: *const u8, length: usize, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        let data: Vec<u8> = From::from(slice::from_raw_parts(data, length as usize));
        tx.send(LosslessPackage(fnum, data)).unwrap();
    }
}

extern fn on_group_invite(_: *mut ll::Tox, friendnumber: i32, kind: u8, data: *const u8,
        length: u16, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        let data: Vec<u8> = From::from(slice::from_raw_parts(data, length as usize));
        let kind: GroupchatType = mem::transmute(kind);
        tx.send(GroupInvite(friendnumber, kind, data)).unwrap();
    }
}

extern fn on_group_message(_: *mut ll::Tox, groupnumber: i32, frindgroupnumber: i32,
        message: *const u8, len: u16, chan: *mut c_void) {
    let tx: &mut Sender<Event> = unsafe { mem::transmute(chan) };
    let msg = parse_string!(message, len);
    tx.send(GroupMessage(groupnumber, frindgroupnumber, msg)).unwrap();
}

extern fn on_group_action(_: *mut ll::Tox, groupnumber: i32, frindgroupnumber: i32,
        action: *const u8, len: u16, chan: *mut c_void) {
    let tx: &mut Sender<Event> = unsafe { mem::transmute(chan) };
    let action = parse_string!(action, len);
    tx.send(GroupMessage(groupnumber, frindgroupnumber, action)).unwrap();
}

extern fn on_group_title(_: *mut ll::Tox, groupnumber: i32, frindgroupnumber: i32,
        message: *const u8, length: u8, chan: *mut c_void) {
    let tx: &mut Sender<Event> = unsafe { mem::transmute(chan) };
    let msg = parse_string!(message, length);
    tx.send(GroupTitle(groupnumber, frindgroupnumber, msg)).unwrap();
}

extern fn on_group_namelist_change(_: *mut ll::Tox, groupnumber: i32, peernumber: i32,
        change: u8, chan: *mut c_void) {
    unsafe {
        let tx: &mut Sender<Event> = mem::transmute(chan);
        let change: ChatChange = mem::transmute(change);
        tx.send(GroupNamelistChange(groupnumber, peernumber, change)).unwrap();
    }
}

// END: Callback pack
