// Copyright 2018 Ed McCardell
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "cargo-clippy", feature(tool_lints))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::cast_lossless))]

use std::cmp::Ordering;
use std::fmt;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,
    BitXorAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign,
    Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct MachineInt<T>(pub T);

pub trait AsFrom<T>: Sized {
    fn as_from(_: T) -> Self;
}

// Trait<T> for MachineInt<T>
// Trait<MachineInt<T>> for <T>
macro_rules! lit_impl {
    ($trait:ident, $meth:ident, $fn:ident) => {};
    ($trait:ident, $meth:ident, $fn:ident, $t:ty) => {
        impl $trait<$t> for MachineInt<$t> {
            type Output = Self;
            #[inline]
            fn $meth(self, rhs: $t) -> Self {
                MachineInt((self.0).$fn(rhs))
            }
        }

        impl $trait<MachineInt<$t>> for $t {
            type Output = MachineInt<$t>;
            #[inline]
            fn $meth(self, rhs: Self::Output) -> Self::Output {
                MachineInt(self.$fn(rhs.0))
            }
        }

        impl $trait<MachineInt<$t>> for MachineInt<$t> {
            type Output = MachineInt<$t>;
            #[inline]
            fn $meth(self, rhs: Self) -> Self {
                MachineInt((self.0).$fn(rhs.0))
            }
        }
    };
    ($trait:ident, $meth:ident, $fn:ident, $t:ty, $($tail:tt)*) => {
        lit_impl!($trait, $meth, $fn, $t);
        lit_impl!($trait, $meth, $fn, $($tail)*);
    };
}

lit_impl!(Add, add, wrapping_add, i8, u8, i16, u16, i32, u32, i64, u64);
lit_impl!(Sub, sub, wrapping_sub, i8, u8, i16, u16, i32, u32, i64, u64);
lit_impl!(Mul, mul, wrapping_mul, i8, u8, i16, u16, i32, u32, i64, u64);
lit_impl!(Div, div, div, i8, u8, i16, u16, i32, u32, i64, u64);
lit_impl!(Rem, rem, rem, i8, u8, i16, u16, i32, u32, i64, u64);
lit_impl!(BitAnd, bitand, bitand, u8, u16, u32, u64);
lit_impl!(BitOr, bitor, bitor, u8, u16, u32, u64);
lit_impl!(BitXor, bitxor, bitxor, u8, u16, u32, u64);

macro_rules! lit_assign_impl {
    ($trait:ident, $meth:ident, $fn:ident) => {};
    ($trait:ident, $meth:ident, $fn:ident, $t:ty) => {
        impl $trait<$t> for MachineInt<$t> {
            #[inline]
            fn $meth(&mut self, rhs: $t) {
                self.0 = (self.0).$fn(rhs);
            }
        }

        impl $trait<MachineInt<$t>> for MachineInt<$t> {
            #[inline]
            fn $meth(&mut self, rhs: Self) {
                self.0 = (self.0).$fn(rhs.0);
            }
        }
    };
    ($trait:ident, $meth:ident, $fn:ident, $t:ty, $($tail:tt)*) => {
        lit_assign_impl!($trait, $meth, $fn, $t);
        lit_assign_impl!($trait, $meth, $fn, $($tail)*);
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
lit_assign_impl!(AddAssign, add_assign, wrapping_add,
                 i8, u8, i16, u16, i32, u32, i64, u64);
#[cfg_attr(rustfmt, rustfmt_skip)]
lit_assign_impl!(SubAssign, sub_assign, wrapping_sub,
                 i8, u8, i16, u16, i32, u32, i64, u64);
#[cfg_attr(rustfmt, rustfmt_skip)]
lit_assign_impl!(MulAssign, mul_assign, wrapping_mul,
                 i8, u8, i16, u16, i32, u32, i64, u64);
#[cfg_attr(rustfmt, rustfmt_skip)]
lit_assign_impl!(DivAssign, div_assign, div,
                 i8, u8, i16, u16, i32, u32, i64, u64);
#[cfg_attr(rustfmt, rustfmt_skip)]
lit_assign_impl!(RemAssign, rem_assign, rem,
                 i8, u8, i16, u16, i32, u32, i64, u64);
lit_assign_impl!(BitAndAssign, bitand_assign, bitand, u8, u16, u32, u64);
lit_assign_impl!(BitOrAssign, bitor_assign, bitor, u8, u16, u32, u64);
lit_assign_impl!(BitXorAssign, bitxor_assign, bitxor, u8, u16, u32, u64);

// Trait<T> for MachineInt<U>
// Trait<MachineInt<U> for T
// Trait<MachineInt<T>> for MachineInt<T>
// where T is the unsigned counterpart of U.
macro_rules! ibit_lit_impl {
    ($trait:ident, $meth:ident, $fn:ident, $lhs:ty, $rhs:ty) => {
        impl $trait<$rhs> for MachineInt<$lhs> {
            type Output = MachineInt<$lhs>;
            #[inline]
            fn $meth(self, rhs: $rhs) -> Self::Output {
                MachineInt((self.0).$fn(rhs as $lhs))
            }
        }

        impl $trait<MachineInt<$lhs>> for $rhs {
            type Output = MachineInt<$lhs>;
            #[inline]
            fn $meth(self, rhs: MachineInt<$lhs>) -> Self::Output {
                MachineInt((self as $lhs).$fn(rhs.0))
            }
        }

        impl $trait<MachineInt<$lhs>> for MachineInt<$lhs> {
            type Output = MachineInt<$lhs>;
            #[inline]
            fn $meth(self, rhs: MachineInt<$lhs>) -> Self::Output {
                MachineInt((self.0).$fn(rhs.0))
            }
        }
    };
    ($trait:ident, $meth:ident, $fn:ident, $lhs:ty, $rhs:ty, $($tail:tt)*) => {
        ibit_lit_impl!($trait, $meth, $fn, $lhs, $rhs);
        ibit_lit_impl!($trait, $meth, $fn, $($tail)*);
    };
}

ibit_lit_impl!(BitAnd, bitand, bitand, i64, u64, i32, u32, i16, u16, i8, u8);
ibit_lit_impl!(BitOr, bitor, bitor, i64, u64, i32, u32, i16, u16, i8, u8);
ibit_lit_impl!(BitXor, bitxor, bitxor, i64, u64, i32, u32, i16, u16, i8, u8);

macro_rules! ibit_lit_assign_impl {
    ($trait:ident, $meth:ident, $fn:ident, $lhs:ty, $rhs:ty) => {
        impl $trait<$rhs> for MachineInt<$lhs> {
            #[inline]
            fn $meth(&mut self, rhs: $rhs) {
                self.0 = (self.0).$fn(rhs as $lhs);
            }
        }

        impl $trait<MachineInt<$rhs>> for MachineInt<$lhs> {
            #[inline]
            fn $meth(&mut self, rhs: MachineInt<$rhs>) {
                self.0 = (self.0).$fn(rhs.0 as $lhs)
            }
        }

        impl $trait<MachineInt<$lhs>> for MachineInt<$lhs> {
            #[inline]
            fn $meth(&mut self, rhs: MachineInt<$lhs>) {
                self.0 = (self.0).$fn(rhs.0)
            }
        }
    };
    ($trait:ident, $meth:ident, $fn:ident, $lhs:ty, $rhs:ty, $($tail:tt)*) => {
        ibit_lit_assign_impl!($trait, $meth, $fn, $lhs, $rhs);
        ibit_lit_assign_impl!($trait, $meth, $fn, $($tail)*);
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
ibit_lit_assign_impl!(BitAndAssign, bitand_assign, bitand,
                      i64, u64, i32, u32, i16, u16, i8, u8);
#[cfg_attr(rustfmt, rustfmt_skip)]
ibit_lit_assign_impl!(BitOrAssign, bitor_assign, bitor,
                      i64, u64, i32, u32, i16, u16, i8, u8);
#[cfg_attr(rustfmt, rustfmt_skip)]
ibit_lit_assign_impl!(BitXorAssign, bitxor_assign, bitxor,
                      i64, u64, i32, u32, i16, u16, i8, u8);

// Neg, Not, Eq, Ord for MachineInt<T>
// PartialEq<T> for MachineInt<T>, PartialEq<MachineInt<T>> for T
// PartialOrd<T> for MachineInt<T>, PartialOrd<MachineInt<T>> for T
macro_rules! misc_impl {
    ($($t:ty)*) => ($(
        impl Neg for MachineInt<$t> {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                MachineInt(self.0.wrapping_neg())
            }
        }

        impl Not for MachineInt<$t> {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                MachineInt(!self.0)
            }
        }

        impl Eq for MachineInt<$t> {}

        impl Ord for MachineInt<$t> {
            fn cmp(&self, rhs: &Self) -> Ordering {
                self.0.cmp(&rhs.0)
            }
        }

        impl PartialEq<$t> for MachineInt<$t> {
            #[inline]
            fn eq(&self, rhs: &$t) -> bool {
                self.0 == *rhs
            }
        }

        impl PartialEq<MachineInt<$t>> for $t {
            #[inline]
            fn eq(&self, rhs: &MachineInt<$t>) -> bool {
                *self == rhs.0
            }
        }

        impl PartialOrd<$t> for MachineInt<$t> {
            #[inline]
            fn partial_cmp(&self, rhs: &$t) -> Option<Ordering> {
                Some(self.0.cmp(&*rhs))
            }
        }

        impl PartialOrd<MachineInt<$t>> for $t {
            #[inline]
            fn partial_cmp(&self, rhs: &MachineInt<$t>) -> Option<Ordering> {
                Some(self.cmp(&rhs.0))
            }
        }

        impl AsFrom<$t> for MachineInt<$t> {
            #[inline]
            fn as_from(val: $t) -> Self {
                MachineInt(val)
            }
        }

        impl AsFrom<MachineInt<$t>> for MachineInt<$t> {
            #[inline]
            fn as_from(val: MachineInt<$t>) -> Self {
                val
            }
        }

        impl From<$t> for MachineInt<$t> {
            #[inline]
            fn from(val: $t) -> Self {
                MachineInt(val)
            }
        }

        impl From<MachineInt<$t>> for $t {
            #[inline]
            fn from(val: MachineInt<$t>) -> Self {
                val.0
            }
        }

        impl Shl<u32> for MachineInt<$t> {
            type Output = Self;
            #[inline]
            fn shl(self, rhs: u32) -> Self {
                MachineInt(self.0.wrapping_shl(rhs))
            }
        }

        impl Shr<u32> for MachineInt<$t> {
            type Output = Self;
            #[inline]
            fn shr(self, rhs: u32) -> Self {
                MachineInt(self.0.wrapping_shr(rhs))
            }
        }

        impl ShlAssign<u32> for MachineInt<$t> {
            #[inline]
            fn shl_assign(&mut self, rhs: u32) {
                self.0 = self.0.wrapping_shl(rhs)
            }
        }

        impl ShrAssign<u32> for MachineInt<$t> {
            #[inline]
            fn shr_assign(&mut self, rhs: u32) {
                self.0 = self.0.wrapping_shr(rhs)
            }
        }
    )*)
}

misc_impl!(i8 u8 i16 u16 i32 u32 i64 u64);

// Trait<MachineInt<T>> for MachineInt<U>
// where T has the same signedness as U
// for PartialEq, PartialOrd
macro_rules! cmp_impl {
    ($lhs:ty; $rhs:ty) => {
        impl PartialEq<MachineInt<$rhs>> for MachineInt<$lhs> {
            #[inline]
            fn eq(&self, rhs: &MachineInt<$rhs>) -> bool {
                self.0 == rhs.0 as $lhs
            }
        }

        impl PartialOrd<MachineInt<$rhs>> for MachineInt<$lhs> {
            #[inline]
            fn partial_cmp(&self, rhs: &MachineInt<$rhs>) -> Option<Ordering> {
                Some(self.0.cmp(&(rhs.0 as $lhs)))
            }
        }
    };
    ($lhs:ty; $rhs:ty, $($tail:tt)*) => {
        cmp_impl!($lhs; $rhs);
        cmp_impl!($lhs; $($tail)*);
    };
}

cmp_impl!(u64; u64, u32, u16, u8);
cmp_impl!(i64; i64, i32, i16, i8);
cmp_impl!(u32; u64, u32, u16, u8);
cmp_impl!(i32; i64, i32, i16, i8);
cmp_impl!(u16; u64, u32, u16, u8);
cmp_impl!(i16; i64, i32, i16, i8);
cmp_impl!(u8; u64, u32, u16, u8);
cmp_impl!(i8; i64, i32, i16, i8);

// PartialEq<MachineInt<T>> for MachineInt<U>
// PartialOrd<MachineInt<T>> for MachineInt<U>
// where P: From<T> and P: From<U>
macro_rules! icmp_impl {
    () => {};
    ($lhs:ty, $rhs:ty, $p:ty) => {
        impl PartialEq<MachineInt<$rhs>> for MachineInt<$lhs> {
            #[inline]
            fn eq(&self, rhs: &MachineInt<$rhs>) -> bool {
                (self.0 as $p) == (rhs.0 as $p)
            }
        }

        impl PartialOrd<MachineInt<$rhs>> for MachineInt<$lhs> {
            #[inline]
            fn partial_cmp(&self, rhs: &MachineInt<$rhs>) -> Option<Ordering> {
                Some((self.0 as $p).cmp(&(rhs.0 as $p)))
            }
        }
    };
    ($lhs:ty, $rhs:ty, $p:ty; $($tail:tt)*) => {
        icmp_impl!($lhs, $rhs, $p);
        icmp_impl!($($tail)*);
    };
}

// TODO: u64/ixx
#[cfg_attr(rustfmt, rustfmt_skip)]
icmp_impl!(u32, i32, i64; i32, u32, i64;
           u32, i16, i64; i16, u32, i64;
           u32, i8, i64; i8, u32, i64);
#[cfg_attr(rustfmt, rustfmt_skip)]
icmp_impl!(i32, u16, i32; u16, i32, i32;
           i32, u8, i32; u8, i32, i32;
           u16, i16, i32; i16, u16, i32;
           u16, i8, i32; i8, u16, i32);
#[cfg_attr(rustfmt, rustfmt_skip)]
icmp_impl!(i16, u8, i16; u8, i16, i16;
           u8, i8, i16; i8, u8, i16);

macro_rules! add_impl {
    () => {};
    ($b:ty, $s:ty) => {
        impl Add<MachineInt<$s>> for MachineInt<$b> {
            type Output = MachineInt<$b>;
            #[inline]
            fn add(self, rhs: MachineInt<$s>) -> Self::Output {
                MachineInt((self.0).wrapping_add(rhs.0 as $b))
            }
        }

        impl Add<MachineInt<$b>> for MachineInt<$s> {
            type Output = MachineInt<$b>;
            #[inline]
            fn add(self, rhs: MachineInt<$b>) -> Self::Output {
                MachineInt((self.0 as $b).wrapping_add(rhs.0))
            }
        }

        impl AddAssign<MachineInt<$s>> for MachineInt<$b> {
            #[inline]
            fn add_assign(&mut self, rhs: MachineInt<$s>) {
                self.0 = self.0.wrapping_add(rhs.0 as $b);
            }
        }

        impl Sub<MachineInt<$s>> for MachineInt<$b> {
            type Output = MachineInt<$b>;
            #[inline]
            fn sub(self, rhs: MachineInt<$s>) -> Self::Output {
                MachineInt((self.0).wrapping_sub(rhs.0 as $b))
            }
        }

        impl Sub<MachineInt<$b>> for MachineInt<$s> {
            type Output = MachineInt<$b>;
            #[inline]
            fn sub(self, rhs: MachineInt<$b>) -> Self::Output {
                MachineInt((self.0 as $b).wrapping_sub(rhs.0))
            }
        }

        impl SubAssign<MachineInt<$s>> for MachineInt<$b> {
            #[inline]
            fn sub_assign(&mut self, rhs: MachineInt<$s>) {
                self.0 = self.0.wrapping_sub(rhs.0 as $b);
            }
        }
    };
    ($b:ty, $s:ty; $($tail:tt)*) => {
        add_impl!($b, $s);
        add_impl!($($tail)*);
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
add_impl!(u64, u32; u64, u16; u64, u8; u64, i32; u64, i16; u64, i8;
          i64, u32; i64, u16; i64, u8; i64, i32; i64, i16; i64, i8;
          u32, u16; u32, u8; u32, i16; u32, i8;
          i32, u16; i32, u8; i32, i16; i32, i8;
          u16, u8; u16, i8;
          i16, u8; i16, i8);

macro_rules! bit_impl {
    () => {};
    ($b:ty, $s:ty, $p:ty) => {
        impl BitAnd<MachineInt<$s>> for MachineInt<$b> {
            type Output = MachineInt<$b>;
            #[inline]
            fn bitand(self, rhs: MachineInt<$s>) -> Self::Output {
                MachineInt(self.0 & (rhs.0 as $p as $b))
            }
        }

        impl BitAnd<MachineInt<$b>> for MachineInt<$s> {
            type Output = MachineInt<$b>;
            #[inline]
            fn bitand(self, rhs: MachineInt<$b>) -> Self::Output {
                MachineInt((self.0 as $p as $b) & rhs.0)
            }
        }

        impl BitAndAssign<MachineInt<$s>> for MachineInt<$b> {
            #[inline]
            fn bitand_assign(&mut self, rhs: MachineInt<$s>) {
                self.0 = self.0 & (rhs.0 as $p as $b);
            }
        }

        impl BitOr<MachineInt<$s>> for MachineInt<$b> {
            type Output = MachineInt<$b>;
            #[inline]
            fn bitor(self, rhs: MachineInt<$s>) -> Self::Output {
                MachineInt(self.0 | (rhs.0 as $p as $b))
            }
        }

        impl BitOr<MachineInt<$b>> for MachineInt<$s> {
            type Output = MachineInt<$b>;
            #[inline]
            fn bitor(self, rhs: MachineInt<$b>) -> Self::Output {
                MachineInt((self.0 as $p as $b) | rhs.0)
            }
        }

        impl BitOrAssign<MachineInt<$s>> for MachineInt<$b> {
            #[inline]
            fn bitor_assign(&mut self, rhs: MachineInt<$s>) {
                self.0 = self.0 | (rhs.0 as $p as $b);
            }
        }

        impl BitXor<MachineInt<$s>> for MachineInt<$b> {
            type Output = MachineInt<$b>;
            #[inline]
            fn bitxor(self, rhs: MachineInt<$s>) -> Self::Output {
                MachineInt(self.0 ^ (rhs.0 as $p as $b))
            }
        }

        impl BitXor<MachineInt<$b>> for MachineInt<$s> {
            type Output = MachineInt<$b>;
            #[inline]
            fn bitxor(self, rhs: MachineInt<$b>) -> Self::Output {
                MachineInt((self.0 as $p as $b) ^ rhs.0)
            }
        }

        impl BitXorAssign<MachineInt<$s>> for MachineInt<$b> {
            #[inline]
            fn bitxor_assign(&mut self, rhs: MachineInt<$s>) {
                self.0 = self.0 ^ (rhs.0 as $p as $b);
            }
        }
    };
    ($b:ty, $s:ty, $p:ty; $($tail:tt)*) => {
        bit_impl!($b, $s, $p);
        bit_impl!($($tail)*);
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
bit_impl!(u64, u32, u32; u64, u16, u16; u64, u8, u8;
          u64, i32, u32; u64, i16, u16; u64, i8, u8;
          i64, u32, u32; i64, u16, u16; i64, u8, u8;
          i64, i32, u32; i64, i16, u16; i64, i8, u8;
          u32, u16, u16; u32, u8, u8;
          u32, i16, u16; u32, i8, u8;
          i32, u16, u16; i32, u8, u8;
          i32, i16, u16; i32, i8, u8;
          u16, u8, u8; u16, i8, u8;
          i16, u8, u8; i16, i8, u8);

macro_rules! from_impl {
    ($lhs:ty, $rhs:ty) => {
        impl From<MachineInt<$rhs>> for MachineInt<$lhs> {
            fn from(val: MachineInt<$rhs>) -> Self {
                MachineInt(val.0 as $lhs)
            }
        }

        impl From<$rhs> for MachineInt<$lhs> {
            fn from(val: $rhs) -> Self {
                MachineInt(val as $lhs)
            }
        }

        impl From<MachineInt<$rhs>> for $lhs {
            fn from(val: MachineInt<$rhs>) -> Self {
                val.0 as $lhs
            }
        }
    };
    ($lhs:ty, $rhs:ty; $($tail:tt)*) => {
        from_impl!($lhs, $rhs);
        from_impl!($($tail)*);
    };
}

from_impl!(u64, u32; u64, u16; u64, u8);
from_impl!(i64, u32; i64, u16; i64, u8; i64, i32; i64, i16; i64, i8);
from_impl!(u32, u16; u32, u8);
from_impl!(i32, u16; i32, u8; i32, i16; i32, i8);
from_impl!(u16, u8);
from_impl!(i16, u8; i16, i8);

macro_rules! as_from_impl {
    ($lhs:ty; $rhs:ty) => {
        impl AsFrom<MachineInt<$rhs>> for MachineInt<$lhs> {
            #[inline]
            fn as_from(val: MachineInt<$rhs>) -> Self {
                MachineInt(val.0 as $lhs)
            }
        }

        impl AsFrom<$rhs> for MachineInt<$lhs> {
            #[inline]
            fn as_from(val: $rhs) -> Self {
                MachineInt(val as $lhs)
            }
        }
    };
    ($lhs:ty; $rhs:ty, $($tail:tt)*) => {
        as_from_impl!($lhs; $rhs);
        as_from_impl!($lhs; $($tail)*);
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
as_from_impl!(u64; i64, u32, i32, u16, i16, u8, i8);
as_from_impl!(i64; u64, u32, i32, u16, i16, u8, i8);
as_from_impl!(u32; u64, i64, i32, u16, i16, u8, i8);
as_from_impl!(i32; u64, i64, u32, u16, i16, u8, i8);
as_from_impl!(u16; u64, i64, u32, i32, i16, u8, i8);
as_from_impl!(i16; u64, i64, u32, i32, u16, u8, i8);
as_from_impl!(u8; u64, i64, u32, i32, u16, i16, i8);
as_from_impl!(i8; u64, i64, u32, i32, u16, i16, u8);

impl<T: fmt::Display> fmt::Display for MachineInt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T: fmt::Debug> fmt::Debug for MachineInt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: fmt::LowerHex> fmt::LowerHex for MachineInt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl<T: fmt::UpperHex> fmt::UpperHex for MachineInt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

impl<T: fmt::Binary> fmt::Binary for MachineInt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

impl<T: fmt::Octal> fmt::Octal for MachineInt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Octal::fmt(&self.0, f)
    }
}
