use basic_comm::decode;
use basic_comm::encode;

use heapless::Vec;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    SetRotation(i32),
    SetLED(bool),
}

fn main() {
    const INPUT_1: &[u8] = b"{\"SetRotation\": 5}";
    const INPUT_2: &[u8] = b"{\"SetLED\": false}";
    let mut output: [u8; 64] = [0; 64];
    let mut offset = encode(INPUT_1, &mut output).unwrap();
    offset += encode(INPUT_2, &mut output[offset..]).unwrap();

    let input = &output[..offset];
    const CMD_VEC_LEN: usize = 4;
    const TMP_BUF_LEN: usize = 64;
    let mut messages = Vec::<Command, CMD_VEC_LEN>::new();
    decode::<Command, CMD_VEC_LEN, TMP_BUF_LEN>(&input, &mut messages).unwrap();

    println!("{:?}", messages);
}
