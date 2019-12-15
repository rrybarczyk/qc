mod common;
mod error;
use crate::common::*;

fn run(args: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<f64>, Error> {
    let mut stack = Vec::new();
    for arg in args {
        let arg = arg.as_ref();
        match arg {
            "add" => add(&mut stack)?,
            "sub" => sub(&mut stack)?,
            "mul" => mul(&mut stack)?,
            "div" => div(&mut stack)?,
            ":add" => add_all(&mut stack)?,
            ":sub" => sub_all(&mut stack)?,
            ":mul" => mul_all(&mut stack)?,
            ":div" => div_all(&mut stack)?,
            "." => pop_print(&mut stack)?,
            _ => num(&mut stack, arg),
        }
    }
    Ok(stack)
}

fn main() -> Result<(), Error> {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);
    run(&args)?;
    Ok(())
}

fn pop_print(stack: &mut Vec<f64>) -> Result<(), Error> {
    let item = pop(stack)?;
    println!("{}", item);
    Ok(())
}

fn pop(stack: &mut Vec<f64>) -> Result<f64, Error> {
    match stack.pop() {
        Some(x) => Ok(x),
        None => Err(Error::StackUnderflow),
    }
}

/// Pops top two items off the stack, adds them, and pushes the sum on the stack.
fn add(stack: &mut Vec<f64>) -> Result<(), Error> {
    let a = pop(stack)?;
    let b = pop(stack)?;
    let sum = b + a;
    stack.push(sum);
    Ok(())
}

/// Pops top two items off the stack, subtracts them, and pushes the difference on the stack.
fn sub(stack: &mut Vec<f64>) -> Result<(), Error> {
    let a = pop(stack)?;
    let b = pop(stack)?;
    let difference = b - a;
    stack.push(difference);
    Ok(())
}

/// Pops top two items off the stack, multiplies them, and pushes the product on the stack.
fn mul(stack: &mut Vec<f64>) -> Result<(), Error> {
    let a = pop(stack)?;
    let b = pop(stack)?;
    let product = b * a;
    stack.push(product);
    Ok(())
}

/// Pops top two items off the stack, divides them, and pushes the quotient on the stack.
fn div(stack: &mut Vec<f64>) -> Result<(), Error> {
    let a = pop(stack)?;
    let b = pop(stack)?;
    let quotient = b / a;
    stack.push(quotient);
    Ok(())
}

/// Pops all items before `:add` off the stack, adds them, and pushes the sum on the stack.
fn add_all(stack: &mut Vec<f64>) -> Result<(), Error> {
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
fn sub_all(stack: &mut Vec<f64>) -> Result<(), Error> {
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
fn mul_all(stack: &mut Vec<f64>) -> Result<(), Error> {
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
fn div_all(stack: &mut Vec<f64>) -> Result<(), Error> {
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
fn num(stack: &mut Vec<f64>, arg: &str) {
    stack.push(arg.parse().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// qc 1 2 add
    fn two_args_add() {
        let args = &["1", "2", "add"];
        let have = run(args).unwrap();
        let want = vec![3.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 2 3 add
    fn three_args_add() {
        let args = &["1", "2", "3", "add"];
        let have = run(args).unwrap();
        let want = vec![1.0, 5.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 2 sub
    fn two_args_sub() {
        let args = &["1", "2", "sub"];
        let have = run(args).unwrap();
        let want = vec![-1.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 2 3 sub
    fn three_args_sub() {
        let args = &["1", "2", "3", "sub"];
        let have = run(args).unwrap();
        let want = vec![1.0, -1.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 2 mul
    fn two_args_mul() {
        let args = &["1", "2", "mul"];
        let have = run(args).unwrap();
        let want = vec![2.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 2 3 mul
    fn three_args_mul() {
        let args = &["1", "2", "3", "mul"];
        let have = run(args).unwrap();
        let want = vec![1.0, 6.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 9 3 div
    fn two_args_div() {
        let args = &["9", "3", "div"];
        let have = run(args).unwrap();
        let want = vec![3.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 6 2 div
    fn three_args_div() {
        let args = &["1", "6", "2", "div"];
        let have = run(args).unwrap();
        let want = vec![1.0, 3.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 2 :add
    fn two_args_add_all() {
        let args = &["1", "2", ":add"];
        let have = run(args).unwrap();
        let want = vec![3.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 2 3 :add
    fn three_args_add_all() {
        let args = &["1", "2", "3", ":add"];
        let have = run(args).unwrap();
        let want = vec![6.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 2 :sub
    fn two_args_sub_all() {
        let args = &["1", "2", ":sub"];
        let have = run(args).unwrap();
        let want = vec![-1.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 2 3 :sub
    fn three_args_sub_all() {
        let args = &["1", "2", "3", ":sub"];
        let have = run(args).unwrap();
        let want = vec![2.0];
        assert_eq!(have, want);

        let args = &["6", "2", "1", ":sub"];
        let have = run(args).unwrap();
        let want = vec![5.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 2 mul
    fn two_args_mul_all() {
        let args = &["1", "2", ":mul"];
        let have = run(args).unwrap();
        let want = vec![2.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 2 3 :mul
    fn three_args_mul_all() {
        let args = &["1", "2", "3", ":mul"];
        let have = run(args).unwrap();
        let want = vec![6.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 9 3 :div
    fn two_args_div_all() {
        let args = &["9", "3", ":div"];
        let have = run(args).unwrap();
        let want = vec![3.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 1 6 2 :div
    fn three_args_div_all() {
        let args = &["6", "2", "1", ":div"];
        let have = run(args).unwrap();
        let want = vec![3.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 4 7 9 add 2 8 mul
    fn three_add_two_mul() {
        let args = &["4", "7", "9", "add", "2", "8", "mul"];
        let have = run(args).unwrap();
        let want = vec![4.0, 16.0, 16.0];
        assert_eq!(have, want);
    }

    #[test]
    /// qc 4 7 9 add add 2 8 mul mul
    fn three_add_add_two_mul_mul() {
        let args = &["4", "7", "9", "add", "add", "2", "8", "mul", "mul"];
        let have = run(args).unwrap();
        let want = vec![320.0];
        assert_eq!(have, want);
    }

    #[test]
    fn stack_underflow() {
        let args = &["1", "add"];
        let have = run(args);
        let want = Err(Error::StackUnderflow);
        assert_eq!(have, want);
    }
}
