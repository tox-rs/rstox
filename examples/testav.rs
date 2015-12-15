extern crate rstox;

use rstox::core::{ Tox, ToxOptions, Event };
use rstox::av::{ ToxAv, CallControl };

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
            match ev {
                Event::FriendRequest(pk, _) => {
                    tox.add_friend_norequest(&pk).ok();
                },
                Event::FriendMessage(fnum, _, msg) => {
                    if &msg == "call me" {
                        toxav.call(fnum, 48, 5000).ok();
                    };
                    if &msg == "bye" {
                        toxav.control(fnum, CallControl::Cancel).ok();
                    };
                },
                Event::Call(fnum, _, _) => {
                    toxav.answer(fnum, 48, 5000).ok();
                },
                Event::AudioReceiveFrame(fnum, pcm, count, chan, rate) => {
                    print!(".");
                    toxav.send_audio(fnum, &pcm, count, chan, rate).ok();
                },
                Event::VideoReceiveFrame(fnum, w, h, y, u, v, _, _, _) => {
                    print!("*");
                    println!(
                        "{} {} {}",
                        h as usize * w as usize >= y.len(),
                        (h/2) as usize * (w/2) as usize >= u.len(),
                        (h/2) as usize * (w/2) as usize >= v.len()
                    );
                    match toxav.send_video(fnum, w, h, &y, &u, &v) {
                        Err(err) => println!("{:?}", err),
                        _ => ()
                    };
                }
                _ => println!("Tox event: {:?}", ev)
            };
        }

        tox.wait();
    }
}
