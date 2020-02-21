mod common;
mod error;
mod opt;

use crate::common::*;

fn run(args: &[&str]) -> Result<Vec<i128>> {
  let (flags, program): (Vec<&str>, Vec<&str>) = args.iter().partition(|f| f.starts_with("--"));

  let opt = Opt::from_iter(flags);

  let mut stack = Vec::new();
  for word in program {
    match word {
      "add" => add(&mut stack)?,
      "sub" => sub(&mut stack)?,
      "mul" => mul(&mut stack)?,
      "div" => div(&mut stack)?,
      "endian" => endian(&mut stack)?,
      ":add" => add_all(&mut stack)?,
      ":sub" => sub_all(&mut stack)?,
      ":mul" => mul_all(&mut stack)?,
      ":div" => div_all(&mut stack)?,
      ":endian" => endian_all(&mut stack)?,
      "pop" => pop(&mut stack).map(|_| ())?,
      "." => pop_print(&mut stack)?,
      ":." => pop_print_all(&mut stack)?,
      _ => num(&mut stack, word)?,
    }

    if opt.verbose {
      println!("Stack:\t\t{:?}", &stack);
    }
  }
  Ok(stack)
}

/// Pops one item off the stack, swaps endianness, pushes the result back on the stack.
fn endian(stack: &mut Vec<i128>) -> Result<()> {
  let width = endian_width(stack)?;
  let item = pop(stack)?;

  // Takes item and converts it to le representation
  let mut bytes = item.to_le_bytes();

  // Takes first "width" bytes of le representation and it swaps those bytes
  let slice = &mut bytes[0..width as usize];
  slice.reverse();
  let swapped = i128::from_le_bytes(bytes);
  stack.push(swapped);
  Ok(())
}

fn endian_all(stack: &mut Vec<i128>) -> Result<()> {
  let width = endian_width(stack)?;

  stack
    .iter_mut()
    .for_each(|value: &mut i128| *value = endian_op(*value, width));

  Ok(())
}

/// Takes first `width` bytes of le representation and it swaps those bytes.
fn endian_op(value: i128, width: usize) -> i128 {
  // Takes item and converts it to le representation
  let mut bytes = value.to_le_bytes();
  let slice = &mut bytes[0..width];
  slice.reverse();
  i128::from_le_bytes(bytes)
}

/// Pop off byte width for endian handle error if too large or negative.
fn endian_width(stack: &mut Vec<i128>) -> Result<usize> {
  let width = pop(stack)?; // 2

  if width < 0 || width > 8 {
    return Err(Error::EndianWidth { width });
  }
  Ok(width as usize)
}

fn main() -> Result<()> {
  let mut buffer = std::env::args().collect::<Vec<String>>();
  buffer.remove(0);
  let slice = buffer.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
  run(&slice)?;
  Ok(())
}

fn pop_print_all(stack: &mut Vec<i128>) -> Result<()> {
  while !stack.is_empty() {
    pop_print(stack)?;
  }
  Ok(())
}

fn pop_print(stack: &mut Vec<i128>) -> Result<()> {
  let item = pop(stack)?;
  println!(
    "dec: {}\t\thex: 0x{:x}\t\toct: o{:o}\t\tbin: b{:b}",
    item, item, item, item
  );
  Ok(())
}

fn pop(stack: &mut Vec<i128>) -> Result<i128> {
  match stack.pop() {
    Some(x) => Ok(x),
    None => Err(Error::StackUnderflow),
  }
}

/// Pops top two items off the stack, adds them, and pushes the sum on the stack.
fn add(stack: &mut Vec<i128>) -> Result<()> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let sum = b + a;
  stack.push(sum);
  Ok(())
}

/// Pops top two items off the stack, subtracts them, and pushes the difference on the stack.
fn sub(stack: &mut Vec<i128>) -> Result<()> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let difference = b - a;
  stack.push(difference);
  Ok(())
}

/// Pops top two items off the stack, multiplies them, and pushes the product on the stack.
fn mul(stack: &mut Vec<i128>) -> Result<()> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let product = b * a;
  stack.push(product);
  Ok(())
}

/// Pops top two items off the stack, divides them, and pushes the quotient on the stack.
fn div(stack: &mut Vec<i128>) -> Result<()> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let quotient = b / a;
  stack.push(quotient);
  Ok(())
}

/// Pops all items before `:add` off the stack, adds them, and pushes the sum on the stack.
fn add_all(stack: &mut Vec<i128>) -> Result<()> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let mut sum = b + a;

  while !stack.is_empty() {
    sum += stack.pop().unwrap();
  }
  stack.push(sum);
  Ok(())
}

/// Pops all items before `:sub` off the stack, subtracts them, and pushes the difference on the stack.
fn sub_all(stack: &mut Vec<i128>) -> Result<()> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let mut difference = b - a;

  while !stack.is_empty() {
    difference = stack.pop().unwrap() - difference;
  }
  stack.push(difference);
  Ok(())
}

/// Pops all items before `:mul` off the stack, multiplies them, and pushes the product on the stack.
fn mul_all(stack: &mut Vec<i128>) -> Result<()> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let mut product = b * a;

  while !stack.is_empty() {
    product *= stack.pop().unwrap();
  }
  stack.push(product);
  Ok(())
}

/// Pops all items before `:div` off the stack, divides them, and pushes the quotient on the stack.
fn div_all(stack: &mut Vec<i128>) -> Result<()> {
  let a = pop(stack)?;
  let b = pop(stack)?;
  let mut quotient = b / a;

  while !stack.is_empty() {
    quotient = stack.pop().unwrap() / quotient;
  }
  stack.push(quotient);
  Ok(())
}

/// Parse arg as a number and push it onto the stack
fn num(stack: &mut Vec<i128>, arg: &str) -> Result<()> {
  if arg.starts_with("0x") {
    let arg = &arg[2..arg.len()];
    stack.push(i128::from_str_radix(&arg, 16).unwrap());
  } else if arg.starts_with("x") {
    let arg = &arg[1..arg.len()];
    stack.push(i128::from_str_radix(&arg, 16).unwrap());
  } else if arg.starts_with("0o") {
    let arg = &arg[2..arg.len()];
    stack.push(i128::from_str_radix(&arg, 8).unwrap());
  } else if arg.starts_with("o") {
    let arg = &arg[1..arg.len()];
    stack.push(i128::from_str_radix(&arg, 8).unwrap());
  } else if arg.starts_with("0b") {
    let arg = &arg[2..arg.len()];
    stack.push(i128::from_str_radix(&arg, 2).unwrap());
  } else if arg.starts_with("b") {
    let arg = &arg[1..arg.len()];
    stack.push(i128::from_str_radix(&arg, 2).unwrap());
  } else {
    stack.push(arg.parse().unwrap());
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  /* Test helper function and macros */

  fn lex(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
  }

  fn test(text: &str) -> Result<Vec<i128>> {
    run(&lex(text))
  }

  /// Tests intended to succeed
  macro_rules! test {
        {
            name: $name:ident,
            text: $text:expr,
            want: $want:expr $(,)?
        } => {
            #[test]
            fn $name() {
                let have = test($text).expect("Expected success");
                let want = $want.iter().cloned().map(|x: i128| x).collect::<Vec<i128>>();
                assert_eq!(have, want);
            }
        };
    }

  /// Tests intended to fail
  macro_rules! error {
        {
            name: $name:ident,
            text: $text:expr,
            want: $want:expr $(,)?
        } => {
            #[test]
            fn $name() {
                let have = test($text).expect_err("Expected error");
                let want = $want;
                assert_eq!(have, want);
            }
        };
    }

  /// Test lexer
  #[test]
  fn lex_empty() {
    let text = "";
    let have = lex(text);
    let want: Vec<String> = Vec::new();
    assert_eq!(have, want);
  }

  #[test]
  fn lex_blank() {
    let text = "       ";
    let have = lex(text);
    let want: Vec<String> = Vec::new();
    assert_eq!(have, want);
  }

  #[test]
  fn lex_word() {
    let text = "foo";
    let have = lex(text);
    let want = vec!["foo".to_string()];
    assert_eq!(have, want);
  }

  #[test]
  fn lex_args() {
    let text = "1 2 add";
    let have = lex(text);
    let want = vec!["1".to_string(), "2".to_string(), "add".to_string()];
    assert_eq!(have, want);
  }

  /* Test errors */

  // Error when popping on empty stack
  // $ qc pop
  error! {
    name: pop_empty,
    text: "pop",
    want: Error::StackUnderflow,
  }

  // $ qc 1 add
  error! {
    name: add_underflow,
    text: "1 add",
    want: Error::StackUnderflow,
  }

  /* Test add */

  // $qc 1 2 add
  test! {
    name: add_two_args,
    text: "1 2 add",
    want: [3],
  }

  // $qc 1 2 3 add
  test! {
    name: add_three_args,
    text: "1 2 3 add",
    want: [1, 5],
  }

  // $qc 1 2 :add
  test! {
    name: add_all_two_args,
    text: "1 2 :add",
    want: [3],
  }

  // $qc 1 2 3 :add
  test! {
    name: add_all_three_args,
    text: "1 2 3 :add",
    want: [6],
  }

  /* Test sub */

  // $ qc 1 2 sub
  test! {
    name: sub_two_args,
    text: "1 2 sub",
    want: [-1],
  }

  // $ qc 1 2 3 sub
  test! {
    name: sub_three_args,
    text: "1 2 3 sub",
    want: [1, -1],
  }

  // $ qc 1 2 :sub
  test! {
    name: sub_all_two_args,
    text: "1 2 :sub",
    want: [-1],
  }

  // $qc 1 2 3 :sub
  test! {
    name: sub_all_three_args,
    text: "1 2 3 :sub",
    want: [2],
  }

  /* Test mul */

  // $ qc 2 3 mul
  test! {
    name: mul_two_args,
    text: "2 3 mul",
    want: [6],
  }

  // $ qc 1 2 3 mul
  test! {
    name: mul_three_args,
    text: "1 2 3 mul",
    want: [1, 6],
  }

  // $ qc 2 4 :mul
  test! {
    name: mul_all_two_args,
    text: "2 4 :mul",
    want: [8],
  }

  // $ qc 1 2 3 :mul
  test! {
    name: mul_all_three_args,
    text: "1 2 3 :mul",
    want: [6],
  }

  /* Test div */

  // $ qc 9 3 div
  test! {
    name: div_two_args,
    text: "9 3 div",
    want: [3],
  }

  // $ qc 1 6 2 div
  test! {
    name: div_three_args,
    text: "1 6 2 div",
    want: [1, 3],
  }

  // $ qc 9 3 :div
  test! {
    name: div_all_two_args,
    text: "9 3 :div",
    want: [3],
  }

  // $ qc 6 2 1 :div
  test! {
    name: div_all_three_args,
    text: "6 2 1 :div",
    want: [3],
  }

  /* Test miscellaneous binary calcs */

  // $ qc 4 7 9 add 2 8 mul
  test! {
    name: add_three_mul_two,
    text: "4 7 9 add 2 8 mul",
    want: [4, 16, 16],
  }

  // $ qc 4 7 9 add add 2 8 mul mul
  test! {
    name: add_two_mul_two,
    text: "4 7 9 add add 2 8 mul mul",
    want: [320],
  }

  test! {
    name: endian_single_byte_no_op,
    text: "0x12 1 endian",
    want: [0x12],
  }

  error! {
    name: endian_0,
    text: "0x123456 10 endian",
    want: Error::EndianWidth { width: 10 },
  }

  test! {
    name: endian_swap_suffix,
    text: "0x123456 2 endian",
    want: [0x125634],
  }

  test! {
    name: endian_single_byte,
    text: "0x1200 2 endian",
    want: [0x12],
  }

  test! {
    name: endian_one_args_0,
    text: "0x1234 2 endian",
    want: [0x3412],
  }

  test! {
    name: endian_one_args_1,
    text: "0xbabe 2 endian",
    want: [0xbeba],
  }

  test! {
    name: endian_one_arg_8_bytes_0,
    text: "0xe803000000000000 8 endian",
    want: [0x00000000000003e8],
  }

  test! {
    name: endian_one_arg_8_bytes_1,
    text: "0x00000000000007d0 8 endian",
    want: [0xd007000000000000],
  }

  test! {
    name: endian_one_arg_8_bytes_2,
    text: "0xd007000000000000 8 endian",
    want: [0x00000000000007d0],
  }

  test! {
    name: endian_all_two_args_4_bytes,
    text: "0xdeadbeef 0xbabebeef 4 :endian",
    want: [0xefbeadde, 0xefbebeba],
  }

  test! {
    name: endian_all_two_args_8_bytes_0,
    text: "0x1234000000000000 0x5678000000000000 8 :endian",
    want: [0x0000000000003412, 0x0000000000007856],
  }

  test! {
    name: endian_all_two_args_8_bytes_1,
    text: "0xe803000000000000 0xd007000000000000  8 :endian",
    want: [0x00000000000003e8, 0x00000000000007d0],
  }
}
