pub fn clear_bit(u: &mut u8, bit: u8) -> () {
    *u &= !(0b00000001 << bit);
}

pub fn set_bit(u: &mut u8, bit: u8) -> () {
    *u |= 0b00000001 << bit;
}

pub fn check_bit(u: u8, bit: u8) -> bool {
    return ((u >> bit) & 0b00000001) == 1;
}

pub fn toggle_bit(u: &mut u8, bit: u8, set: bool) -> () {
    if set {
        set_bit(u, bit);
    } else {
        clear_bit(u, bit);
    }
}

#[cfg(test)]
mod bit_tests {
    use crate::libs::bit::{check_bit, toggle_bit};

    #[test]
    fn test_toggle_bit() {
        let mut n: u8 = 0b10000000;
        toggle_bit(&mut n, 0, true);
        assert_eq!(n, 0b10000001);
    }

    #[test]
    fn test_check_bit() {
        let n: u8 = 0b10000000;
        let set = check_bit(n, 7);
        assert_eq!(set, true);
    }
}
