#[derive(Debug, Default)]
pub struct Hash128to64 {
    state: u64,
}

impl std::hash::Hasher for Hash128to64 {
    fn write(&mut self, bytes: &[u8]) {
        // println!("{}", bytes.len());
        assert!(bytes.len() == 8);
        self.state = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        // assert!(bytes.len() == 16);
        // let a = u64::from_le_bytes(bytes[0..8].try_into().unwrap()) + 0b_0000_0000_0000_0000_0000_0000_0000_0000_1111_1111_1111_1111_1111_1111_1111_1111;
        // let b = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) + 0b_0000_0000_0000_0000_0000_0000_0000_0000_1111_1111_1111_1111_1111_1111_1111_1111;
        // self.state = !(a & 0b_0000_0000_0000_0000_0000_0000_0000_0000_1111_1111_1111_1111_1111_1111_1111_1111)
        //             | (b & 0b_0000_0000_0000_0000_0000_0000_0000_0000_1111_1111_1111_1111_1111_1111_1111_1111);
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

#[derive(Debug, Default, Clone)]
pub struct BuildHash128to64;

impl std::hash::BuildHasher for BuildHash128to64 {
    type Hasher = Hash128to64;
    fn build_hasher(&self) -> Hash128to64 {
        Hash128to64 { state: 0 }
    }
}

#[test]
fn testbsbss() {
    // let
    // let a : [u8; 8] = u.to_ne_bytes();

    let mut hm = std::collections::HashMap::<u64, u64, BuildHash128to64>::with_hasher(BuildHash128to64);
    // ...
    let i: i8 = (0 as i8).wrapping_add(0b_0000_1111);
    println!("{i:b}");
}