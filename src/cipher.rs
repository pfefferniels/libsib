//! The XOR keystream that obscures compressed chunk payloads.
//!
//! Two classic LCGs (MSVC `rand` and ANSI C `rand`) are mixed under control of a
//! third 15-bit generator. The algorithm was first published by cw2k in SCORA
//! (<https://github.com/cw2k/SCORA>, `CryptedStream.java`); Scorch files use the
//! same scheme with a different selector multiplier.

const KEY1_MASK: u32 = 0x1F30_D27B;
const KEY2_MASK: u32 = 0x3A4C_80B2;
const POST_XOR: [u32; 4] = [27811, 12742, 37606, 5005];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    Sibelius,
    Scorch,
}

#[derive(Clone, Debug)]
pub struct Keystream {
    variant: Variant,
    key1: u32,
    key2: u32,
    selector: u32,
}

impl Keystream {
    pub fn new(seed: u32, variant: Variant) -> Self {
        let base = seed & !7;
        Self {
            variant,
            key1: base ^ KEY1_MASK,
            key2: base ^ KEY2_MASK,
            selector: (seed & 7) << 4,
        }
    }

    fn advance_selector(&mut self) -> usize {
        self.selector = match self.variant {
            Variant::Sibelius => self.selector.wrapping_mul(13821) & 0x7FFF,
            Variant::Scorch => self.selector.wrapping_mul(47485) & 0xFFFF,
        };
        ((self.selector >> 8) & 3) as usize
    }
}

impl Iterator for Keystream {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let mode = self.advance_selector();
        let mut mixed = 0;
        if mode != 2 {
            self.key1 = self.key1.wrapping_mul(214_013).wrapping_add(2_531_011);
            mixed = self.key1;
        }
        if mode != 1 {
            self.key2 = self.key2.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            mixed ^= self.key2;
        }
        Some((POST_XOR[mode] ^ ((mixed >> 16) & 0x7FFF)) as u8)
    }
}

/// XORs `data` with the keystream for `seed`. Encryption and decryption are the same operation.
pub fn apply(data: &[u8], seed: u32) -> Vec<u8> {
    data.iter()
        .zip(Keystream::new(seed, Variant::Sibelius))
        .map(|(byte, key)| byte ^ key)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_is_an_involution() {
        let plain = b"0123456789abcdef".repeat(10);
        assert_eq!(apply(&apply(&plain, 0x0BAF_C16F), 0x0BAF_C16F), plain);
    }

    #[test]
    fn zero_selector_never_leaves_mode_zero() {
        let mut ks = Keystream::new(0x1000, Variant::Sibelius);
        assert!((0..1000).all(|_| ks.advance_selector() == 0));
    }
}
