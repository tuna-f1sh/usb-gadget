mod common;
use common::*;

use usb_gadget::function::audio::Uac2;

#[test]
fn audio() {
    init();

    let builder = Uac2::builder();
    let (audio, func) = builder.build();

    let reg = reg(func);

    println!("UAC2 audio device at {}", audio.status().path().unwrap().display());

    unreg(reg).unwrap();
}
