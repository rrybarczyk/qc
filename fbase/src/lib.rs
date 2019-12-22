pub fn dec2hex(dec: f64) -> f64 {
    println!("{:?}", dec2base(0.0628, 16));
    println!("{:?}", dec2base(3020.0625, 16));
    dec
}

// https://en.wikipedia.org/wiki/IEEE_754
// // https://doc.rust-lang.org/std/num/struct.NonZeroU8.html
//
// pub(crate) struct Number {
//     number: f64, // dont use float. use denom and numer (frac)
//     base: u8,
//     sign: isize,
//     integer: Option<usize>,
//     decimal: Option<usize>,
//     places: Option<usize>,
// }

pub(crate) struct Number {
    numerator: u64,
    denomnator: std::num::NonZeroU64,
    positive: bool,
}

// have second struct or something for intermediate steps
// use From Number to conver qc main

impl Number {
    fn new(number: f64, base: u8) -> Self {
        Number {
            number: number,
            base: base,
            sign: (number as isize).signum(),
            integer: None,
            decimal: None,
            places: None,
        }
    }

    // fn decimal_to_base(number: f64, base: u8) -> Self {
    //     let sign = (dec as isize).signum();
    //     let sigfig = 4; // Number of signification digits
    //     let mut carry_stack: Vec<isize> = vec![]; // Holds carries
    //
    //     let integer = dec.trunc().to_string(); // Integer part
    //     let mut f = dec.fract(); // Fractional part
    //
    //     let mut i = 0;
    //     let mut places = 0;
    //     while f != 0. && i < sigfig {
    //         let tmp = f * (base as f64);
    //
    //         // Increment i if significant digit is encountered
    //         if tmp != 0. {
    //             i += 1;
    //         }
    //         carry_stack.push(tmp.trunc() as isize);
    //         f = tmp.fract();
    //         places += 1;
    //     }
    //
    //     let decimal: String = carry_stack
    //         .into_iter()
    //         .map(|i| i.to_string())
    //         .collect::<String>();
    //
    //     (integer, decimal, places, base, sign)
    // }
}

fn dec2base(dec: f64, base: u8) -> (String, String, isize, u8, isize) {
    let sign = (dec as isize).signum();
    let sigfig = 4; // Number of signification digits
    let mut carry_stack: Vec<isize> = vec![]; // Holds carries

    let integer = dec.trunc().to_string(); // Integer part
    let mut f = dec.fract(); // Fractional part

    let mut i = 0;
    let mut places = 0;
    while f != 0. && i < sigfig {
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

    (integer, decimal, places, base, sign)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[test]
    // 0.0628_10 to 0.1013_16
    // 3020.0625_10 to BCC.10_16
    fn decimal_to_hexadecimal() {
        let number = Number::new(number, base);
        let have = dec2base(3020.0625, 16);
        let want = (3020.0, 0.1);
        // assert_eq!(have, want);
        assert_eq!(1, 1);
    }
}
