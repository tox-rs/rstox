#![feature(box_syntax)]
#![feature(core, std_misc)]

extern crate rstox;

use std::sync::Arc;
use std::cell::{RefCell};
use std::thread::Thread;
use rstox::core::*;
use rstox::av::*;

static BOOTSTRAP_IP: &'static str = "192.254.75.98";
static BOOTSTRAP_PORT: u16 = 33445;
static BOOTSTRAP_KEY: &'static str =
                    "951C88B7E75C867418ACDB5D273821372BB5BD652740BCDF623A4FA293E75D2F";
static BOT_NAME: &'static str = "yuppi";

fn start_av(tox: Tox) -> (Arc<RefCell<Tox>>, GroupAudio) {
    let (tox_cell, mut av) = ToxAv::new(tox, 10);
    let gr_audio = av.group_audio(box |_, _, _, _| {});

    av.on_event(box |_, ev| println!("Av event: {:?}", ev));

    Thread::spawn(move || {
        let mut av = av;
        loop {
            av.tick();
            av.wait();
        }
    });
    (tox_cell, gr_audio)
}

fn main() {
    let (tox_cell, gr_audio) = start_av(Tox::new(ToxOptions::new()));

    let mut tox = tox_cell.borrow_mut();
    tox.set_name(BOT_NAME).unwrap();
    let bootstrap_key = BOOTSTRAP_KEY.parse().unwrap();
    tox.bootstrap_from_address(BOOTSTRAP_IP.to_string(), BOOTSTRAP_PORT, bootstrap_key).unwrap();

    println!("{}", tox.get_address());

    loop {
        for ev in tox.iter() {
            match ev {
                FriendRequest(cid, _) => {
                    tox.add_friend_norequest(*cid).unwrap();
                },
                GroupInvite(fid, kind, data) => {
                    match kind {
                        GroupchatType::Text => { tox.join_groupchat(fid, &data).unwrap(); },
                        GroupchatType::Av => { gr_audio.join_groupchat(&mut tox, fid, &data).unwrap(); },
                    }
                },
                ev => { println!("Tox event: {:?}", ev); },
            }
        }

        tox.wait();
    }
}
