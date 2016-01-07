extern crate rstox;

use std::fs::File;
use std::io::{ Read, Write };
use rstox::core::{ Tox, ToxOptions, Event };
use rstox::av::{ ToxAv, CallControl };

static BOOTSTRAP_IP: &'static str = "192.254.75.98";
static BOOTSTRAP_PORT: u16 = 33445;
static BOOTSTRAP_KEY: &'static str =
                    "951C88B7E75C867418ACDB5D273821372BB5BD652740BCDF623A4FA293E75D2F";
static BOT_NAME: &'static str = "testavbot";

fn main() {
    let mut data = Vec::new();
    let mut tox = Tox::new(ToxOptions::new(), match File::open("./test.tox") {
        Ok(mut fd) => {
            fd.read_to_end(&mut data).ok();
            Some(&data)
        },
        Err(_) => None
    }).unwrap();
    let mut toxav = ToxAv::new(&mut tox).unwrap();
    tox.set_name(BOT_NAME).ok();
    tox.bootstrap(
        BOOTSTRAP_IP,
        BOOTSTRAP_PORT,
        BOOTSTRAP_KEY.parse().unwrap()
    ).ok();

    println!("{}", tox.get_address());

    loop {
        for ev in tox.iter() {
            match ev {
                Event::FriendRequest(pk, _) => {
                    tox.add_friend_norequest(&pk).ok();
                    match File::create("./test.tox") {
                        Ok(mut fd) => { fd.write(&tox.save()).ok(); },
                        Err(err) => println!("{:?}", err)
                    }
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
                    toxav.send_audio(fnum, &pcm, count, chan, rate).ok();
                },
                Event::VideoReceiveFrame(fnum, w, h, y, u, v, ys, us, vs) => {
                    let mut yy = Vec::new();
                    for i in 0..h {
                        let mut yyy = y[(i as usize * ys as usize)..((i as usize * ys as usize) + w as usize)].to_vec();
                        yy.append(&mut yyy);
                    }

                    let mut uu = Vec::new();
                    let mut vv = Vec::new();
                    for i in 0..(h as usize / 2) {
                        let mut uuu = u[(i as usize * us as usize)..((i as usize * us as usize) + w as usize / 2)].to_vec();
                        let mut vvv = v[(i as usize * vs as usize)..((i as usize * vs as usize) + w as usize / 2)].to_vec();
                        uu.append(&mut uuu);
                        vv.append(&mut vvv);
                    }

                    match toxav.send_video(fnum, w, h, &yy, &uu, &vv) {
                        Err(err) => println!("{:?}", err),
                        _ => ()
                    };
                }
                _ => println!("Tox event: {:?}", ev)
            };
        }

        toxav.tick();
        tox.wait();
    }
}
