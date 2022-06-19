use rand::Rng;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const PASSWORD_LEN: usize = 10;

pub fn gen_rand_str() -> String {
    let mut rng = rand::thread_rng();
    (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}