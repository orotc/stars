use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::Crc;
use crc::CRC_32_ISCSI;
use std::{
    error::Error,
    fmt, io,
    io::{Read, Seek, SeekFrom, Write},
};

fn mask_crc(crc: u32) -> u32 {
    ((crc >> 15) | (crc << 17)).wrapping_add(0xa282_ead8u32)
}
