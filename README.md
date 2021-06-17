# BitIndex

A little-endian zero-indexed bitstring representation.

Internally represented by the primitive unsigned integer types. Use the appropriate `BitIndex` depending on the number of indices that need to be tracked.
- `u8` -> `BitIndex8`
- `u16` -> `BitIndex16`
- `u32` -> `BitIndex32`
- `u64` -> `BitIndex64`
- `u128` -> `BitIndex128`

## Usage

A `BitIndex` is initialized with ones for the requested availables indices. A `BitIndex` can be unwrapped to return a copy of the internal representation.
````rust
let bi_res = BitIndex8::new(5) // -> Ok(BitIndex8)
let bi = bi_res.unwrap() // -> BitIndex8
assert_eq!(0b11111, bi.unwrap());
````

Bits can be directly set and unset. `BitIndex` is zero-indexed.
````rust
let mut bi = BitIndex8::new(5).unwrap() // -> BitIndex8
bi.unset(0);
assert_eq!(0b11110, bi.unwrap());
bi.set(0);
assert_eq!(0b11111, bi.unwrap());
````

Smallest and largest availables indices can be queried.
````rust
let mut bi = BitIndex8::new(5).unwrap() // -> BitIndex8
assert_eq!(Some(4), bi.largest());
bi.unset(4);
bi.unset(3);
assert_eq!(Some(2), bi.largest());
bi.clear();
assert_eq!(None, bi.largest());
bi.restore();
assert_eq!(Some(4), bi.largest());

assert_eq!(Some(0), bi.smallest());
bi.unset(1);
bi.unset(2);
assert_eq!(Some(2), bi.smallest());
bi.clear();
assert_eq!(None, bi.smallest());
bi.restore();
assert_eq!(Some(0), bi.smallest());
````

Popping the smallest/largest returns the value, and unsets it
````rust
let mut bi = BitIndex8::new(5).unwrap() // -> BitIndex8
assert_eq!(Some(4), bi.pop_largest());
assert_eq!(Some(3), bi.largest());
assert_eq!(Some(3), bi.pop_largest());
assert_eq!(Some(2), bi.pop_largest());
assert_eq!(Some(1), bi.pop_largest());
assert_eq!(Some(0), bi.pop_largest());
assert_eq!(None, bi.pop_largest());

bi.restore();
assert_eq!(Some(0), bi.pop_smallest());
assert_eq!(Some(1), bi.smallest());
````