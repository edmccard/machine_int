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
}
