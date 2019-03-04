extern crate rstox;

use rstox::core::*;

static BOOTSTRAP_IP: &'static str = "192.254.75.98";
static BOOTSTRAP_PORT: u16 = 33445;
static BOOTSTRAP_KEY: &'static str =
                    "951C88B7E75C867418ACDB5D273821372BB5BD652740BCDF623A4FA293E75D2F";
static BOT_NAME: &'static str = "yuppi";

fn main() {
    let mut tox = Tox::new(ToxOptions::new(), None).unwrap();
    tox.set_name(BOT_NAME).unwrap();
    let bootstrap_key = BOOTSTRAP_KEY.parse().unwrap();
    tox.bootstrap(BOOTSTRAP_IP, BOOTSTRAP_PORT, bootstrap_key).unwrap();

    println!("{}", tox.get_address());

    loop {
        for ev in tox.iter() {
            match ev {
                FriendRequest(cid, _) => {
                    tox.add_friend_norequest(&cid).unwrap();
                },
                ev => { println!("Tox event: {:?}", ev); },
            }
        }

        tox.wait();
    }
}
