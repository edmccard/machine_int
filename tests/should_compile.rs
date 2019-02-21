// Copyright 2018 Ed McCardell
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use machine_int::MachineInt;

#[allow(unused_must_use, unused_assignments)]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    let mut v_i8 = MachineInt(0i8);
    let mut v_u8 = MachineInt(0u8);
    let mut v_i16 = MachineInt(0i16);
    let mut v_u16 = MachineInt(0u16);
    let mut v_i32 = MachineInt(0i32);
    let mut v_u32 = MachineInt(0u32);
    let mut v_i64 = MachineInt(0i64);
    let mut v_u64 = MachineInt(0u64);

    (v_i8 + v_i8, v_i8 - v_i8, v_i8 & v_i8, v_i8 | v_i8, v_i8 ^ v_i8);
    (v_u8 + v_u8, v_u8 - v_u8, v_u8 & v_u8, v_u8 | v_u8, v_u8 ^ v_u8);
    (v_i16 + v_i16, v_i16 - v_i16, v_i16 & v_i16, v_i16 | v_i16, v_i16 ^ v_i16);
    (v_u16 + v_u16, v_u16 - v_u16, v_u16 & v_u16, v_u16 | v_u16, v_u16 ^ v_u16);
    (v_i32 + v_i32, v_i32 - v_i32, v_i32 & v_i32, v_i32 | v_i32, v_i32 ^ v_i32);
    (v_u32 + v_u32, v_u32 - v_u32, v_u32 & v_u32, v_u32 | v_u32, v_u32 ^ v_u32);
    (v_i64 + v_i64, v_i64 - v_i64, v_i64 & v_i64, v_i64 | v_i64, v_i64 ^ v_i64);
    (v_u64 + v_u64, v_u64 - v_u64, v_u64 & v_u64, v_u64 | v_u64, v_u64 ^ v_u64);

    v_i8 += 1; v_i8 += v_i8; v_i8 -= 1; v_i8 -= v_i8;
    v_i8 &= 1; v_i8 &= v_i8; v_i8 |= 1; v_i8 |= v_i8; v_i8 ^= 1; v_i8 ^= v_i8;

    v_u8 += 1; v_u8 += v_u8; v_u8 -= 1; v_u8 -= v_u8;
    v_u8 &= 1; v_u8 &= v_u8; v_u8 |= 1; v_u8 |= v_u8; v_u8 ^= 1; v_u8 ^= v_u8;

    v_i16 += 1; v_i16 += v_i16; v_i16 -= 1; v_i16 -= v_i16;
    v_i16 &= 1; v_i16 &= v_i16; v_i16 |= 1; v_i16 |= v_i16; v_i16 ^= 1; v_i16 ^= v_i16;

    v_u16 += 1; v_u16 += v_u16; v_u16 -= 1; v_u16 -= v_u16;
    v_u16 &= 1; v_u16 &= v_u16; v_u16 |= 1; v_u16 |= v_u16; v_u16 ^= 1; v_u16 ^= v_u16;

    v_i32 += 1; v_i32 += v_i32; v_i32 -= 1; v_i32 -= v_i32;
    v_i32 &= 1; v_i32 &= v_i32; v_i32 |= 1; v_i32 |= v_i32; v_i32 ^= 1; v_i32 ^= v_i32;

    v_u32 += 1; v_u32 += v_u32; v_u32 -= 1; v_u32 -= v_u32;
    v_u32 &= 1; v_u32 &= v_u32; v_u32 |= 1; v_u32 |= v_u32; v_u32 ^= 1; v_u32 ^= v_u32;

    v_i64 += 1; v_i64 += v_i64; v_i64 -= 1; v_i64 -= v_i64;
    v_i64 &= 1; v_i64 &= v_i64; v_i64 |= 1; v_i64 |= v_i64; v_i64 ^= 1; v_i64 ^= v_i64;

    v_u64 += 1; v_u64 += v_u64; v_u64 -= 1; v_u64 -= v_u64;
    v_u64 &= 1; v_u64 &= v_u64; v_u64 |= 1; v_u64 |= v_u64; v_u64 ^= 1; v_u64 ^= v_u64;

    (v_i8 + 1, v_u8 + 1, v_i16 + 1, v_u16 + 1, v_i32 + 1, v_u32 + 1,
     v_i64 + 1, v_u64 + 1);
    (1 + v_i8, 1 + v_u8, 1 + v_i16, 1 + v_u16, 1 + v_i32, 1 + v_u32,
     1 + v_i64, 1 + v_u64);
    (v_i8 - 1, v_u8 - 1, v_i16 - 1, v_u16 - 1, v_i32 - 1, v_u32 - 1,
     v_i64 - 1, v_u64 - 1);
    (1 - v_i8, 1 - v_u8, 1 - v_i16, 1 - v_u16, 1 - v_i32, 1 - v_u32,
     1 - v_i64, 1 - v_u64);
    (v_i8 & 1, v_u8 & 1, v_i16 & 1, v_u16 & 1, v_i32 & 1, v_u32 & 1,
     v_i64 & 1, v_u64 & 1);
    (1 & v_i8, 1 & v_u8, 1 & v_i16, 1 & v_u16, 1 & v_i32, 1 & v_u32,
     1 & v_i64, 1 & v_u64);
    (v_i8 | 1, v_u8 | 1, v_i16 | 1, v_u16 | 1, v_i32 | 1, v_u32 | 1,
     v_i64 | 1, v_u64 | 1);
    (0xff | v_i8, 1 | v_u8, 0xffff | v_i16, 1 | v_u16, 1 | v_i32, 1 | v_u32,
     1 | v_i64, 1 | v_u64);
    (v_i8 ^ 1, v_u8 ^ 1, v_i16 ^ 1, v_u16 ^ 1, v_i32 ^ 1, v_u32 ^ 1,
     v_i64 ^ 1, v_u64 ^ 1);
    (1 ^ v_i8, 1 ^ v_u8, 1 ^ v_i16, 1 ^ v_u16, 1 ^ v_i32, 1 ^ v_u32,
     1 ^ v_i64, 1 ^ v_u64);

    (v_u64 + v_u32, v_u64 + v_i32, v_u64 + v_u16, v_u64 + v_i16,
     v_u64 + v_u8, v_u64 + v_i8);
    (v_u32 + v_u64, v_i32 + v_u64, v_u16 + v_u64, v_i16 + v_u64,
     v_u8 + v_u64, v_i8 + v_u64);
    (v_u32 + v_u16, v_u32 + v_i16, v_u32 + v_u8, v_u32 + v_i8);
    (v_u16 + v_u32, v_i16 + v_u32, v_u8 + v_u32, v_i8 + v_u32);
    (v_u16 + v_u8, v_u16 + v_i8);
    (v_u8 + v_u16, v_i8 + v_u16);
    (v_i64 + v_u32, v_i64 + v_i32, v_i64 + v_u16, v_i64 + v_i16,
     v_i64 + v_u8, v_i64 + v_i8);
    (v_u32 + v_i64, v_i32 + v_i64, v_u16 + v_i64, v_i16 + v_i64,
     v_u8 + v_i64, v_i8 + v_i64);
    (v_i32 + v_u16, v_i32 + v_i16, v_i32 + v_u8, v_i32 + v_i8);
    (v_u16 + v_i32, v_i16 + v_i32, v_u8 + v_i32, v_i8 + v_i32);
    (v_i16 + v_u8, v_i16 + v_i8);
    (v_u8 + v_i16, v_i8 + v_i16);

    (v_u64 - v_u32, v_u64 - v_i32, v_u64 - v_u16, v_u64 - v_i16,
     v_u64 - v_u8, v_u64 - v_i8);
    (v_u32 - v_u64, v_i32 - v_u64, v_u16 - v_u64, v_i16 - v_u64,
     v_u8 - v_u64, v_i8 - v_u64);
    (v_u32 - v_u16, v_u32 - v_i16, v_u32 - v_u8, v_u32 - v_i8);
    (v_u16 - v_u32, v_i16 - v_u32, v_u8 - v_u32, v_i8 - v_u32);
    (v_u16 - v_u8, v_u16 - v_i8);
    (v_u8 - v_u16, v_i8 - v_u16);
    (v_i64 - v_u32, v_i64 - v_i32, v_i64 - v_u16, v_i64 - v_i16,
     v_i64 - v_u8, v_i64 - v_i8);
    (v_u32 - v_i64, v_i32 - v_i64, v_u16 - v_i64, v_i16 - v_i64,
     v_u8 - v_i64, v_i8 - v_i64);
    (v_i32 - v_u16, v_i32 - v_i16, v_i32 - v_u8, v_i32 - v_i8);
    (v_u16 - v_i32, v_i16 - v_i32, v_u8 - v_i32, v_i8 - v_i32);
    (v_i16 - v_u8, v_i16 - v_i8);
    (v_u8 - v_i16, v_i8 - v_i16);

    (v_u64 & v_u32, v_u64 & v_i32, v_u64 & v_u16, v_u64 & v_i16,
     v_u64 & v_u8, v_u64 & v_i8);
    (v_u32 & v_u64, v_i32 & v_u64, v_u16 & v_u64, v_i16 & v_u64,
     v_u8 & v_u64, v_i8 & v_u64);
    (v_u32 & v_u16, v_u32 & v_i16, v_u32 & v_u8, v_u32 & v_i8);
    (v_u16 & v_u32, v_i16 & v_u32, v_u8 & v_u32, v_i8 & v_u32);
    (v_u16 & v_u8, v_u16 & v_i8);
    (v_u8 & v_u16, v_i8 & v_u16);
    (v_i64 & v_u32, v_i64 & v_i32, v_i64 & v_u16, v_i64 & v_i16,
     v_i64 & v_u8, v_i64 & v_i8);
    (v_u32 & v_i64, v_i32 & v_i64, v_u16 & v_i64, v_i16 & v_i64,
     v_u8 & v_i64, v_i8 & v_i64);
    (v_i32 & v_u16, v_i32 & v_i16, v_i32 & v_u8, v_i32 & v_i8);
    (v_u16 & v_i32, v_i16 & v_i32, v_u8 & v_i32, v_i8 & v_i32);
    (v_i16 & v_u8, v_i16 & v_i8);
    (v_u8 & v_i16, v_i8 & v_i16);

    (v_u64 | v_u32, v_u64 | v_i32, v_u64 | v_u16, v_u64 | v_i16,
     v_u64 | v_u8, v_u64 | v_i8);
    (v_u32 | v_u64, v_i32 | v_u64, v_u16 | v_u64, v_i16 | v_u64,
     v_u8 | v_u64, v_i8 | v_u64);
    (v_u32 | v_u16, v_u32 | v_i16, v_u32 | v_u8, v_u32 | v_i8);
    (v_u16 | v_u32, v_i16 | v_u32, v_u8 | v_u32, v_i8 | v_u32);
    (v_u16 | v_u8, v_u16 | v_i8);
    (v_u8 | v_u16, v_i8 | v_u16);
    (v_i64 | v_u32, v_i64 | v_i32, v_i64 | v_u16, v_i64 | v_i16,
     v_i64 | v_u8, v_i64 | v_i8);
    (v_u32 | v_i64, v_i32 | v_i64, v_u16 | v_i64, v_i16 | v_i64,
     v_u8 | v_i64, v_i8 | v_i64);
    (v_i32 | v_u16, v_i32 | v_i16, v_i32 | v_u8, v_i32 | v_i8);
    (v_u16 | v_i32, v_i16 | v_i32, v_u8 | v_i32, v_i8 | v_i32);
    (v_i16 | v_u8, v_i16 | v_i8);
    (v_u8 | v_i16, v_i8 | v_i16);

    (v_u64 ^ v_u32, v_u64 ^ v_i32, v_u64 ^ v_u16, v_u64 ^ v_i16,
     v_u64 ^ v_u8, v_u64 ^ v_i8);
    (v_u32 ^ v_u64, v_i32 ^ v_u64, v_u16 ^ v_u64, v_i16 ^ v_u64,
     v_u8 ^ v_u64, v_i8 ^ v_u64);
    (v_u32 ^ v_u16, v_u32 ^ v_i16, v_u32 ^ v_u8, v_u32 ^ v_i8);
    (v_u16 ^ v_u32, v_i16 ^ v_u32, v_u8 ^ v_u32, v_i8 ^ v_u32);
    (v_u16 ^ v_u8, v_u16 ^ v_i8);
    (v_u8 ^ v_u16, v_i8 ^ v_u16);
    (v_i64 ^ v_u32, v_i64 ^ v_i32, v_i64 ^ v_u16, v_i64 ^ v_i16,
     v_i64 ^ v_u8, v_i64 ^ v_i8);
    (v_u32 ^ v_i64, v_i32 ^ v_i64, v_u16 ^ v_i64, v_i16 ^ v_i64,
     v_u8 ^ v_i64, v_i8 ^ v_i64);
    (v_i32 ^ v_u16, v_i32 ^ v_i16, v_i32 ^ v_u8, v_i32 ^ v_i8);
    (v_u16 ^ v_i32, v_i16 ^ v_i32, v_u8 ^ v_i32, v_i8 ^ v_i32);
    (v_i16 ^ v_u8, v_i16 ^ v_i8);
    (v_u8 ^ v_i16, v_i8 ^ v_i16);

    v_u64 += v_u32; v_u64 += v_i32; v_u64 += v_u16; v_u64 += v_i16; v_u64 += v_u8; v_u64 += v_i8;
    v_i64 += v_u32; v_i64 += v_i32; v_i64 += v_u16; v_i64 += v_i16; v_i64 += v_u8; v_i64 += v_i8;
    v_u32 += v_u16; v_u32 += v_i16; v_u32 += v_u8; v_u32 += v_i8;
    v_i32 += v_u16; v_i32 += v_i16; v_i32 += v_u8; v_i32 += v_i8;
    v_u16 += v_u8; v_u16 += v_i8;
    v_i16 += v_u8; v_i16 += v_i8;

    v_u64 -= v_u32; v_u64 -= v_i32; v_u64 -= v_u16; v_u64 -= v_i16; v_u64 -= v_u8; v_u64 -= v_i8;
    v_i64 -= v_u32; v_i64 -= v_i32; v_i64 -= v_u16; v_i64 -= v_i16; v_i64 -= v_u8; v_i64 -= v_i8;
    v_u32 -= v_u16; v_u32 -= v_i16; v_u32 -= v_u8; v_u32 -= v_i8;
    v_i32 -= v_u16; v_i32 -= v_i16; v_i32 -= v_u8; v_i32 -= v_i8;
    v_u16 -= v_u8; v_u16 -= v_i8;
    v_i16 -= v_u8; v_i16 -= v_i8;

    v_u64 &= v_u32; v_u64 &= v_i32; v_u64 &= v_u16; v_u64 &= v_i16; v_u64 &= v_u8; v_u64 &= v_i8;
    v_i64 &= v_u32; v_i64 &= v_i32; v_i64 &= v_u16; v_i64 &= v_i16; v_i64 &= v_u8; v_i64 &= v_i8;
    v_u32 &= v_u16; v_u32 &= v_i16; v_u32 &= v_u8; v_u32 &= v_i8;
    v_i32 &= v_u16; v_i32 &= v_i16; v_i32 &= v_u8; v_i32 &= v_i8;
    v_u16 &= v_u8; v_u16 &= v_i8;
    v_i16 &= v_u8; v_i16 &= v_i8;

    v_u64 |= v_u32; v_u64 |= v_i32; v_u64 |= v_u16; v_u64 |= v_i16; v_u64 |= v_u8; v_u64 |= v_i8;
    v_i64 |= v_u32; v_i64 |= v_i32; v_i64 |= v_u16; v_i64 |= v_i16; v_i64 |= v_u8; v_i64 |= v_i8;
    v_u32 |= v_u16; v_u32 |= v_i16; v_u32 |= v_u8; v_u32 |= v_i8;
    v_i32 |= v_u16; v_i32 |= v_i16; v_i32 |= v_u8; v_i32 |= v_i8;
    v_u16 |= v_u8; v_u16 |= v_i8;
    v_i16 |= v_u8; v_i16 |= v_i8;

    v_u64 ^= v_u32; v_u64 ^= v_i32; v_u64 ^= v_u16; v_u64 ^= v_i16; v_u64 ^= v_u8; v_u64 ^= v_i8;
    v_i64 ^= v_u32; v_i64 ^= v_i32; v_i64 ^= v_u16; v_i64 ^= v_i16; v_i64 ^= v_u8; v_i64 ^= v_i8;
    v_u32 ^= v_u16; v_u32 ^= v_i16; v_u32 ^= v_u8; v_u32 ^= v_i8;
    v_i32 ^= v_u16; v_i32 ^= v_i16; v_i32 ^= v_u8; v_i32 ^= v_i8;
    v_u16 ^= v_u8; v_u16 ^= v_i8;
    v_i16 ^= v_u8; v_i16 ^= v_i8;

    v_i8 < 1; v_i8 > 1; v_i8 == 1; 1 > v_i8; 1 < v_i8; 1 == v_i8; v_i8 == v_i8;
    v_u8 < 1; v_u8 > 1; v_u8 == 1; 1 > v_u8; 1 < v_u8; 1 == v_u8; v_u8 == v_u8;
    v_i16 < 1; v_i16 > 1; v_i16 == 1; 1 > v_i16; 1 < v_i16; 1 == v_i16; v_i16 == v_i16;
    v_u16 < 1; v_u16 > 1; v_u16 == 1; 1 > v_u16; 1 < v_u16; 1 == v_u16; v_u16 == v_u16;
    v_i32 < 1; v_i32 > 1; v_i32 == 1; 1 > v_i32; 1 < v_i32; 1 == v_i32; v_i32 == v_i32;
    v_u32 < 1; v_u32 > 1; v_u32 == 1; 1 > v_u32; 1 < v_u32; 1 == v_u32; v_u32 == v_u32;
    v_i64 < 1; v_i64 > 1; v_i64 == 1; 1 > v_i64; 1 < v_i64; 1 == v_i64; v_i64 == v_i64;
    v_u64 < 1; v_u64 > 1; v_u64 == 1; 1 > v_u64; 1 < v_u64; 1 == v_u64; v_u64 == v_u64;

    (v_u64 < v_u32, v_u64 < v_u16, v_u64 < v_u8);
    (v_u32 < v_u64, v_u16 < v_u64, v_u8 < v_u64);
    (v_u32 < v_u16, v_u32 < v_i16, v_u32 < v_u8, v_u32 < v_i8);
    (v_u16 < v_u32, v_i16 < v_u32, v_u8 < v_u32, v_i8 < v_u32);
    (v_u16 < v_u8, v_u16 < v_i8);
    (v_u8 < v_u16, v_i8 < v_u16);
    (v_i64 < v_i32, v_i64 < v_i16, v_i64 < v_i8);
    (v_i32 < v_i64, v_i16 < v_i64, v_i8 < v_i64);
    (v_i32 < v_u16, v_i32 < v_i16, v_i32 < v_u8, v_i32 < v_i8);
    (v_u16 < v_i32, v_i16 < v_i32, v_u8 < v_i32, v_i8 < v_i32);
    (v_i16 < v_u8, v_i16 < v_i8);
    (v_u8 < v_i16, v_i8 < v_i16);

    test_usize_isize();
}

#[cfg(target_pointer_width = "64")]
fn test_usize_isize() {
    let mu8 = MachineInt(0u8);
    let mi8 = MachineInt(0i8);
    let mu16 = MachineInt(0u16);
    let mi16 = MachineInt(0i16);
    let mu32 = MachineInt(0u32);
    let mi32 = MachineInt(0i32);
    let mu64 = MachineInt(0u64);
    let mi64 = MachineInt(0i64);

    usize::from(mu8);
    isize::from(mi8);
    usize::from(mu16);
    isize::from(mi16);
    usize::from(mu32);
    isize::from(mi32);
    usize::from(mu64);
    isize::from(mi64);
}

#[cfg(target_pointer_width = "32")]
fn test_usize_isize() {
    let mu8 = MachineInt(0u8);
    let mi8 = MachineInt(0i8);
    let mu16 = MachineInt(0u16);
    let mi16 = MachineInt(0i16);
    let mu32 = MachineInt(0u32);
    let mi32 = MachineInt(0i32);

    usize::from(mu8);
    isize::from(mi8);
    usize::from(mu16);
    isize::from(mi16);
    usize::from(mu32);
    isize::from(mi32);
}

#[cfg(target_pointer_width = "16")]
fn test_usize_isize() {
    let mu8 = MachineInt(0u8);
    let mi8 = MachineInt(0i8);
    let mu16 = MachineInt(0u16);
    let mi16 = MachineInt(0i16);

    usize::from(mu8);
    isize::from(mi8);
    usize::from(mu16);
    isize::from(mi16);
}
