#![no_std]

use core::str::Utf8Error;

use heapless::Vec;
use serde::de::DeserializeOwned;
use serial_line_ip::{Decoder, Encoder};

#[derive(Debug)]
pub enum DecodingError {
    NotEndOfPacket,
    UTF8Decode(Utf8Error),
    Deserialize,
}

pub fn decode<T, const N: usize, const TMP_BUF_LEN: usize>(
    src: &[u8],
    dst: &mut Vec<T, N>,
) -> Result<(), DecodingError>
where
    T: DeserializeOwned + core::fmt::Debug,
{
    let mut offset = 0;
    let mut output: [u8; TMP_BUF_LEN] = [0; TMP_BUF_LEN];
    while offset < src.len() {
        let (input_bytes_processed, output_slice, is_end_of_packet) =
            Decoder::new().decode(&src[offset..], &mut output).unwrap();
        if !is_end_of_packet {
            return Err(DecodingError::NotEndOfPacket);
        }
        offset += input_bytes_processed;
        let utf8 = core::str::from_utf8(output_slice).map_err(DecodingError::UTF8Decode)?;
        let (data, _): (T, _) =
            serde_json_core::from_str(utf8).map_err(|_| DecodingError::Deserialize)?;

        dst.push(data).unwrap();
    }
    Ok(())
}

pub fn encode(msg: &[u8], dst: &mut [u8]) -> Result<usize, serial_line_ip::Error> {
    let mut enc = Encoder::new();
    let mut totals = enc.encode(msg, dst)?;
    totals += enc.finish(&mut dst[totals.written..])?;
    Ok(totals.written)
}
