extern crate rstox;

use rstox::core::{ Tox, ToxOptions, Event };
use rstox::av::{ ToxAv };

static BOOTSTRAP_IP: &'static str = "192.254.75.98";
static BOOTSTRAP_PORT: u16 = 33445;
static BOOTSTRAP_KEY: &'static str =
                    "951C88B7E75C867418ACDB5D273821372BB5BD652740BCDF623A4FA293E75D2F";
static BOT_NAME: &'static str = "testavbot";

fn main() {
    let mut tox = Tox::new(ToxOptions::new(), None).unwrap();
    let mut toxav = ToxAv::new(&mut tox).unwrap();
    tox.set_name(BOT_NAME).ok();
    tox.bootstrap(
        BOOTSTRAP_IP,
        BOOTSTRAP_PORT,
        BOOTSTRAP_KEY.parse().unwrap()
    ).ok();

    println!("{}", tox.get_address());

    loop {
        for ev in tox.iter(Some(&mut toxav)) {
            println!("Tox event: {:?}", ev.clone());

            match ev {
                Event::FriendRequest(pk, _) => {
                    tox.add_friend_norequest(&pk).ok();
                },
                Event::FriendMessage(fnum, _, msg) => {
                    if &msg == "call me" {
                        toxav.call(fnum, 48, 64).ok();
                    };
                },
                Event::Call(fnum, _, _) => {
                    toxav.answer(fnum, 48, 64).ok();
                },
                Event::AudioReceiveFrame(fnum, pcm, count, chan, rate) => {
                    toxav.send_audio(fnum, &pcm, count, chan, rate).ok();
                },
                _ => ()
            };
        }

        tox.wait();
    }
}
