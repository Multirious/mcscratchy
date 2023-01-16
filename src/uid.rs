use rand::prelude::*;
use rs_sb3::value::Uid;

const SOUP: &str =
    "!#%()*+,-./:;=?@[]^_`{|}~ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const SOUP_LEN: usize = SOUP.len();
const UID_LEN: usize = 20;

pub fn uid() -> Uid {
    let mut rng = thread_rng();
    let mut uid = (0..SOUP_LEN)
        .map(|_| SOUP.as_bytes()[rng.gen_range(0..SOUP_LEN)])
        .collect();
    // SAFETY: We're taking from 'SOUP' const and they're all valid utf8 characters
    unsafe { String::from_utf8_unchecked(uid) }
}