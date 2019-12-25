pub fn dec2hex(dec: f64) -> String {
    dec2base(dec, 16)
}

// pub fn dec2oct(dec: f64) -> String {
//     dec2base(dec, 8);
// }

fn dec2base(dec: f64, base: u8) -> String {
    let (mut integer, mut decimal, places, _sign) = breakdown_number(dec, base);

    let decimal_len = decimal.len();
    let diff = places - decimal_len as isize;

    if diff != 0 {
        let mut leading_zeros = String::from_utf8(vec![b'0'; diff as usize]).unwrap();
        leading_zeros.push_str(&decimal);
        decimal = leading_zeros;
    }

    integer.push_str(".");
    integer.push_str(&decimal);

    integer
}

fn breakdown_number(dec: f64, base: u8) -> (String, String, isize, isize) {
    let sign = (dec as isize).signum();
    let sigfig = 4; // Number of signification digits
    let mut carry_stack: Vec<isize> = vec![]; // Holds carries

    let integer = format!("{:#X}", dec.trunc() as usize); // Integer part
    let mut f = dec.fract(); // Fractional part

    let mut i = 0;
    let mut places = 0;
    while f != 0. && i < sigfig {
        dbg!(&carry_stack);
        dbg!(f);
        let tmp = f * (base as f64);

        // Increment i if significant digit is encountered
        if tmp != 0. {
            i += 1;
        }
        carry_stack.push(tmp.trunc() as isize);
        f = tmp.fract();
        places += 1;
    }

    let decimal: String = carry_stack
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>();

    (integer, decimal, places, sign)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    // 3020.0625_10 to BCC.10_16
    fn decimal_to_hexadecimal() {
        let have = breakdown_number(3020.0625, 16);
        let want = ("0xBCC".to_string(), "1".to_string(), 1, 1);
        assert_eq!(have, want);

        let have = breakdown_number(0.0628, 16);
        let want = ("0x0".to_string(), "1013".to_string(), 4, 0);
        assert_eq!(have, want);

        let have = breakdown_number(3.14, 16);
        let want = ("0x3".to_string(), "1013".to_string(), 4, 1);
        assert_eq!(have, want);
    }

    // #[test]
    // fn dec2hex_pi() {
    //     let pii: f64 = 3.14159;
    //     let have = dec2hex(pii);
    //     let want = String::from("0x3.243F6A");
    //     assert_eq!(have, want);
    // }
}
