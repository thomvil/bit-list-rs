macro_rules! impl_bit_index {
    ($bit_index_name:ident, $bit_index_type:ty) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $bit_index_name {
            bits: $bit_index_type,
            nb_bits: usize,
        }

        impl $bit_index_name {
            const SIZE: usize = std::mem::size_of::<$bit_index_type>() * 8;

            pub fn new(nb_bits: usize) -> Result<Self, String> {
                if (nb_bits as usize) > Self::SIZE {
                    Err(format!(
                        "This BitManipulator can only keep {} bits, not {}",
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

            pub fn unwrap(&self) -> $bit_index_type {
                self.bits
            }

            pub fn is_empty(&self) -> bool {
                self.bits == 0
            }

            pub fn clear(&mut self) {
                self.bits = 0;
            }

            pub fn restore(&mut self) {
                self.bits = Self::init(self.nb_bits);
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
            pub fn set_bit(&mut self, bit_nb: u8) {
                self.bits |= self.single_bit(bit_nb);
            }

            // explicit check not necessary: handled by `all_but_single_bit`
            pub fn unset_bit(&mut self, bit_nb: u8) {
                self.bits &= self.all_but_single_bit(bit_nb);
            }

            fn single_bit(&self, bit_nb: u8) -> $bit_index_type {
                self.check_input(bit_nb);
                1 << bit_nb
            }

            // explicit check not necessary: handled by `single_bit`
            fn all_but_single_bit(&self, bit_nb: u8) -> $bit_index_type {
                <$bit_index_type>::MAX ^ self.single_bit(bit_nb)
            }

            fn check_input(&self, i: u8) {
                if (i as usize) >= self.nb_bits {
                    panic!(
                        "This BitManipulator can only handle inputs upto {}",
                        self.nb_bits
                    )
                }
            }

            fn init(nb_bits: usize) -> $bit_index_type {
                (1 << nb_bits) - 1
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
        let bm = BitIndex8::new(4).unwrap();
        assert_eq!(0b1111, bm.unwrap());
        assert!(BitIndex8::new(9).is_err());

        let bm = BitIndex64::new(44).unwrap();
        assert_eq!(0b11111111111111111111111111111111111111111111, bm.unwrap());
        assert!(BitIndex64::new(69).is_err());
    }

    #[test]
    fn set_unset_bits() {
        let mut bm = BitIndex8::new(4).unwrap();
        assert_eq!(0b1111, bm.unwrap());
        bm.unset_bit(2);
        assert_eq!(0b1011, bm.unwrap());
        bm.unset_bit(0);
        assert_eq!(0b1010, bm.unwrap());
        bm.unset_bit(0);
        assert_eq!(0b1010, bm.unwrap());
        bm.set_bit(2);
        assert_eq!(0b1110, bm.unwrap());
        bm.set_bit(2);
        assert_eq!(0b1110, bm.unwrap());
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
        let mut bm = BitIndex8::new(4).unwrap();
        assert_eq!(Some(0), bm.smallest());
        bm.unset_bit(0);
        assert_eq!(Some(1), bm.smallest());
        bm.unset_bit(3);
        assert_eq!(Some(1), bm.smallest());
        bm.unset_bit(1);
        bm.unset_bit(2);
        assert_eq!(None, bm.smallest());
    }

    #[test]
    fn largest() {
        let mut bm = BitIndex8::new(4).unwrap();
        assert_eq!(Some(3), bm.largest());
        bm.unset_bit(3);
        assert_eq!(Some(2), bm.largest());
        bm.unset_bit(2);
        bm.unset_bit(1);
        assert_eq!(Some(0), bm.largest());
        bm.unset_bit(0);
        assert_eq!(None, bm.largest());
    }

    #[test]
    fn pop_smallest() {
        let mut bm = BitIndex8::new(4).unwrap();
        assert_eq!(Some(0), bm.pop_smallest());
        assert_eq!(Some(1), bm.pop_smallest());
        assert_eq!(Some(2), bm.pop_smallest());
        assert_eq!(Some(3), bm.pop_smallest());
        assert_eq!(None, bm.pop_smallest());
    }

    #[test]
    fn pop_largest() {
        let mut bm = BitIndex8::new(4).unwrap();
        assert_eq!(Some(3), bm.pop_largest());
        assert_eq!(Some(2), bm.pop_largest());
        assert_eq!(Some(1), bm.pop_largest());
        assert_eq!(Some(0), bm.pop_largest());
        assert_eq!(None, bm.pop_largest());

        let mut bm = BitIndex8::new(4).unwrap();
        bm.unset_bit(1);
        assert_eq!(Some(3), bm.pop_largest());
        assert_eq!(Some(2), bm.pop_largest());
        assert_eq!(Some(0), bm.pop_largest());
        assert_eq!(None, bm.pop_largest());
    }
}
