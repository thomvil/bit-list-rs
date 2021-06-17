use std::cmp::max;
use std::fmt::{self, Debug};

macro_rules! impl_bit_index {
    ($bit_index_name:ident, $bit_index_type:ty) => {
        /// A list of bits to track elements. Little-endian and zero-indexed.`
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $bit_index_name {
            /// The bits to track elements
            bits: $bit_index_type,
            /// The number of elements to track. Leading zeros do not represent anything, the zero's in the least `nb_bits` positions represent the absence of the corresponding element.
            nb_bits: u8,
        }

        impl $bit_index_name {
            const SIZE: u8 = std::mem::size_of::<$bit_index_type>() as u8 * 8;

            pub fn new(nb_bits: u8) -> Result<Self, String> {
                if nb_bits > Self::SIZE {
                    Err(format!(
                        "This BitIndex can only keep {} bits, not {}",
                        Self::SIZE,
                        nb_bits
                    ))
                } else {
                    Ok(Self {
                        bits: Self::init(nb_bits),
                        nb_bits,
                    })
                }
            }

            pub fn empty(nb_bits: u8) -> Result<Self, String> {
                Self::new(nb_bits).map(|mut bi| {
                    bi.clear();
                    bi
                })
            }

            pub fn unwrap(&self) -> $bit_index_type {
                self.bits
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                self.bits == 0
            }

            #[inline]
            pub fn clear(&mut self) {
                self.bits = 0;
            }

            pub fn restore(&mut self) {
                self.bits = Self::init(self.nb_bits);
            }

            pub fn nb_elements(&self) -> u8 {
                self.bits.count_ones() as u8
            }

            pub fn get(&mut self, idx: u8) -> Option<u8> {
                self.get_from_low_end(idx)
            }

            pub fn get_from_low_end(&self, idx: u8) -> Option<u8> {
                let nb_elements = self.nb_elements();
                self.get_check(idx).and_then(|_| match idx {
                    0 => self.smallest(),
                    i if i == nb_elements - 1 => self.largest(),
                    i if i > (nb_elements / 2) => self.get_from_high_end(nb_elements - idx - 1),
                    i => {
                        let mut my_clone = self.clone();
                        for _ in (0..i) {
                            my_clone.pop_smallest();
                        }
                        let res = my_clone.smallest();
                        res
                    }
                })
            }

            pub fn get_from_high_end(&self, idx: u8) -> Option<u8> {
                let nb_elements = self.nb_elements();
                self.get_check(idx).and_then(|_| match idx {
                    0 => self.largest(),
                    i if i == nb_elements - 1 => self.smallest(),
                    i if i > (nb_elements / 2) => self.get_from_low_end(nb_elements - idx - 1),
                    i => {
                        let mut my_clone = self.clone();
                        for _ in (0..i) {
                            my_clone.pop_largest();
                        }
                        let res = my_clone.largest();
                        res
                    }
                })
            }

            fn get_check(&self, idx: u8) -> Option<u8> {
                if idx >= self.nb_bits {
                    panic!(
                        "This {} can only handle inputs upto {}",
                        stringify!($bit_index_name),
                        self.nb_bits
                    );
                }
                if self.is_empty() || idx >= self.nb_elements() {
                    return None;
                }
                Some(0)
            }

            pub fn pop(&mut self, idx: u8) -> Option<u8> {
                let res = self.get(idx);
                res.map(|bit_nb| self.unset_bit(bit_nb));
                res
            }

            pub fn pop_from_low_end(&mut self, idx: u8) -> Option<u8> {
                let res = self.get_from_low_end(idx);
                res.map(|bit_nb| self.unset_bit(bit_nb));
                res
            }

            pub fn pop_from_high_end(&mut self, idx: u8) -> Option<u8> {
                let res = self.get_from_high_end(idx);
                res.map(|bit_nb| self.unset_bit(bit_nb));
                res
            }

            pub fn smallest(&self) -> Option<u8> {
                if self.is_empty() {
                    None
                } else {
                    Some(self.bits.trailing_zeros() as u8)
                }
            }

            pub fn pop_smallest(&mut self) -> Option<u8> {
                let res = self.smallest();
                res.map(|bit_nb| self.unset_bit(bit_nb));
                res
            }

            pub fn largest(&self) -> Option<u8> {
                if self.is_empty() {
                    None
                } else {
                    Some((Self::SIZE as u8) - self.bits.leading_zeros() as u8 - 1)
                }
            }

            pub fn pop_largest(&mut self) -> Option<u8> {
                let res = self.largest();
                res.map(|bit_nb| self.unset_bit(bit_nb));
                res
            }

            // explicit check not necessary: handled by `single_bit`
            #[inline]
            pub fn set_bit(&mut self, bit_nb: u8) {
                self.bits |= self.single_bit(bit_nb);
            }

            // explicit check not necessary: handled by `all_but_single_bit`
            #[inline]
            pub fn unset_bit(&mut self, bit_nb: u8) {
                self.bits &= self.all_but_single_bit(bit_nb);
            }

            pub fn add(&mut self, bits: $bit_index_type) {
                self.bits |= bits
            }

            pub fn absorb(&mut self, other: $bit_index_name) {
                self.add(other.bits);
                self.nb_bits = max(self.nb_bits, other.nb_bits);
            }

            #[inline]
            fn single_bit(&self, bit_nb: u8) -> $bit_index_type {
                self.check_input(bit_nb);
                1 << bit_nb
            }

            // explicit check not necessary: handled by `single_bit`
            #[inline]
            fn all_but_single_bit(&self, bit_nb: u8) -> $bit_index_type {
                <$bit_index_type>::MAX ^ self.single_bit(bit_nb)
            }

            #[inline]
            fn check_input(&self, i: u8) {
                if i >= self.nb_bits {
                    panic!(
                        "This {} can only handle inputs upto {}",
                        stringify!($bit_index_name),
                        self.nb_bits
                    )
                }
            }

            #[inline]
            fn init(nb_bits: u8) -> $bit_index_type {
                if nb_bits == Self::SIZE {
                    <$bit_index_type>::MAX
                } else {
                    (1 << nb_bits) - 1
                }
            }
        }

        impl Debug for $bit_index_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                writeln!(f, "{} {{", stringify!($bit_index_name))?;
                writeln!(f, "    nb_bits: {}", self.nb_bits)?;
                writeln!(f, "    bits: {:b}", self.bits)?;
                writeln!(f, "}}")
            }
        }
    };
}

impl_bit_index!(BitIndex8, u8);
impl_bit_index!(BitIndex16, u16);
impl_bit_index!(BitIndex32, u32);
impl_bit_index!(BitIndex64, u64);
impl_bit_index!(BitIndex128, u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let bi = BitIndex8::new(4).unwrap();
        assert_eq!(0b1111, bi.unwrap());
        assert!(BitIndex8::new(9).is_err());

        let bi = BitIndex64::new(44).unwrap();
        assert_eq!(0b11111111111111111111111111111111111111111111, bi.unwrap());
        assert!(BitIndex64::new(69).is_err());
    }

    #[test]
    fn empty() {
        let mut bi = BitIndex8::empty(5).unwrap();
        assert_eq!(None, bi.largest());
        assert_eq!(0, bi.nb_elements());
        bi.restore();
        assert_eq!(5, bi.nb_elements());
        assert_eq!(Some(4), bi.largest());
    }

    #[test]
    fn nb_elements() {
        let mut bi = BitIndex8::new(5).unwrap();
        assert_eq!(5, bi.nb_elements());
        bi.pop_largest();
        assert_eq!(4, bi.nb_elements());
    }

    #[test]
    fn set_unset_bits() {
        let mut bi = BitIndex8::new(4).unwrap();
        assert_eq!(0b1111, bi.unwrap());
        bi.unset_bit(2);
        assert_eq!(0b1011, bi.unwrap());
        bi.unset_bit(0);
        assert_eq!(0b1010, bi.unwrap());
        bi.unset_bit(0);
        assert_eq!(0b1010, bi.unwrap());
        bi.set_bit(2);
        assert_eq!(0b1110, bi.unwrap());
        bi.set_bit(2);
        assert_eq!(0b1110, bi.unwrap());
    }

    #[test]
    #[should_panic]
    fn set_panic() {
        BitIndex8::new(4).unwrap().set_bit(4);
    }

    #[test]
    #[should_panic]
    fn unset_panic() {
        BitIndex8::new(4).unwrap().unset_bit(4);
    }

    #[test]
    fn smallest() {
        let mut bi = BitIndex8::new(4).unwrap();
        assert_eq!(Some(0), bi.smallest());
        bi.unset_bit(0);
        assert_eq!(Some(1), bi.smallest());
        bi.unset_bit(3);
        assert_eq!(Some(1), bi.smallest());
        bi.unset_bit(1);
        bi.unset_bit(2);
        assert_eq!(None, bi.smallest());
    }

    #[test]
    fn largest() {
        let mut bi = BitIndex8::new(4).unwrap();
        assert_eq!(Some(3), bi.largest());
        bi.unset_bit(3);
        assert_eq!(Some(2), bi.largest());
        bi.unset_bit(2);
        bi.unset_bit(1);
        assert_eq!(Some(0), bi.largest());
        bi.unset_bit(0);
        assert_eq!(None, bi.largest());
    }

    #[test]
    fn pop_smallest() {
        let mut bi = BitIndex8::new(4).unwrap();
        assert_eq!(Some(0), bi.pop_smallest());
        assert_eq!(Some(1), bi.pop_smallest());
        assert_eq!(Some(2), bi.pop_smallest());
        assert_eq!(Some(3), bi.pop_smallest());
        assert_eq!(None, bi.pop_smallest());
    }

    #[test]
    fn pop_largest() {
        let mut bi = BitIndex8::new(4).unwrap();
        assert_eq!(Some(3), bi.pop_largest());
        assert_eq!(Some(2), bi.pop_largest());
        assert_eq!(Some(1), bi.pop_largest());
        assert_eq!(Some(0), bi.pop_largest());
        assert_eq!(None, bi.pop_largest());

        let mut bi = BitIndex8::new(4).unwrap();
        bi.unset_bit(1);
        assert_eq!(Some(3), bi.pop_largest());
        assert_eq!(Some(2), bi.pop_largest());
        assert_eq!(Some(0), bi.pop_largest());
        assert_eq!(None, bi.pop_largest());
    }

    #[test]
    fn get() {
        let mut bi = BitIndex8::new(4).unwrap();
        bi.unset_bit(1);
        assert_eq!(3, bi.nb_elements());
        assert_eq!(Some(0), bi.get(0));
        assert_eq!(Some(2), bi.get(1));
        assert_eq!(Some(3), bi.get(2));
        assert_eq!(None, bi.get(3));

        let mut bi = BitIndex64::new(64).unwrap();
        bi.unset_bit(1);
        assert_eq!(Some(62), bi.get_from_low_end(61));
        assert_eq!(Some(3), bi.get_from_high_end(60));

        let mut bi = BitIndex8::new(8).unwrap();
        bi.unset_bit(1);
        assert_eq!(Some(2), bi.get_from_high_end(5));
        assert_eq!(Some(6), bi.get_from_low_end(5));
    }

    #[test]
    fn get_largest() {
        let mut bi = BitIndex8::new(4).unwrap();
        bi.unset_bit(1);
        assert_eq!(3, bi.nb_elements());
        assert_eq!(Some(3), bi.get_from_high_end(0));
        assert_eq!(Some(2), bi.get_from_high_end(1));
        assert_eq!(Some(0), bi.get_from_high_end(2));
        assert_eq!(None, bi.get(3));
    }

    #[test]
    #[should_panic]
    fn get_panic() {
        let mut bi = BitIndex8::new(4).unwrap();

        assert_eq!(None, bi.get(4));
        assert_eq!(None, bi.get(10));
    }
}
