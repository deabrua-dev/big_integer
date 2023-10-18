use std::cmp::{max, Ordering};
use std::i64;
use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Rem, Shl, Shr, Sub};

#[derive(Debug)]
pub struct BigInt {
    pub digits: Vec<u64>,
}

impl Default for BigInt {
    fn default() -> Self {
        Self { digits: vec![]}
    }
}

impl BigInt {
    pub fn new() -> Self {
        Self { digits: vec![]}
    }

    pub fn get_hex(&self) -> String {
        return self.digits.iter()
            .rev()
            .map(|&x| format!("{:x}", x).chars()
                .last()
                .unwrap()
                .to_string())
            .collect::<Vec<String>>()
            .join("");
    }

    pub fn set_hex(&mut self, number: String) -> Result<(), String> {
        if number.is_empty() {
            return Err(String::from("String is empty."));
        }
        let hex_str = number.clone();
        for chunk  in hex_str.chars().rev() {
            self.digits.push(u64::from_str_radix(chunk.to_string().as_str(), 16).unwrap());
        }
        while *self.digits.last().unwrap() == 0 {
            self.digits.pop();
        }
        return Ok(());
    }
}

impl Not for BigInt {
    type Output = Self;

    fn not(self) -> Self {
        let mut answer = BigInt::new();
        for digit in self.digits {
            let not_digit = !digit;
            answer.digits.push(not_digit);
        }
        return answer;
    }
}

impl BitXor for BigInt {
    type Output = Self;

    fn bitxor(self, _rhs: Self) -> Self {
        let mut answer = BigInt::new();
        let max_len = max(self.digits.len(), _rhs.digits.len());
        for i in 0..max_len {
            let num1 = if i < self.digits.len() { self.digits[i] } else { 0 };
            let num2 = if i < _rhs.digits.len() { _rhs.digits[i] } else { 0 };
            let result = num1 ^ num2;
            answer.digits.push(result);
        }
        return answer;
    }
}

impl BitOr for BigInt {
    type Output = Self;

    fn bitor(self, _rhs: Self) -> Self {
        let mut answer = BigInt::new();
        let max_len = max(self.digits.len(), _rhs.digits.len());
        for i in 0..max_len {
            let num1 = if i < self.digits.len() { self.digits[i] } else { 0 };
            let num2 = if i < _rhs.digits.len() { _rhs.digits[i] } else { 0 };
            let result = num1 | num2;
            answer.digits.push(result);
        }
        return answer;
    }
}

impl BitAnd for BigInt {
    type Output = Self;

    fn bitand(self, _rhs: Self) -> Self {
        let mut answer = BigInt::new();
        let max_len = max(self.digits.len(), _rhs.digits.len());
        for i in 0..max_len {
            let num1 = if i < self.digits.len() { self.digits[i] } else { 0 };
            let num2 = if i < _rhs.digits.len() { _rhs.digits[i] } else { 0 };
            let result = num1 & num2;
            answer.digits.push(result);
        }
        return answer;
    }
}

impl Shr<usize> for BigInt {
    type Output = Self;

    fn shr(self, _shift: usize) -> Self {
        todo!()
    }
}

impl Shl<usize> for BigInt {
    type Output = Self;

    fn shl(self, _shift: usize) -> Self {
        todo!()
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(mut self, mut _rhs: Self) -> Self {
        let mut answer = BigInt::new();
        while self.digits.len() > _rhs.digits.len() {
            _rhs.digits.push(0);
        }
        while self.digits.len() < _rhs.digits.len() {
            self.digits.push(0);
        }
        let mut carry = 0;
        for i in 0..self.digits.len() {
            let digit_sum = self.digits[i] + _rhs.digits[i] + carry;
            if digit_sum >= 16 {
                answer.digits.push(digit_sum - 16);
                carry = 1;
            } else {
                answer.digits.push(digit_sum);
                carry = 0;
            }
        }
        if carry == 1 {
            answer.digits.push(1);
        }
        while *answer.digits.last().unwrap() == 0 {
            answer.digits.pop();
        }
        return answer;
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(mut self, mut _rhs: Self) -> Self {
        let mut answer = BigInt::new();
        while self.digits.len() > _rhs.digits.len() {
            _rhs.digits.push(0);
        }
        while self.digits.len() < _rhs.digits.len() {
            self.digits.push(0);
        }
        let mut carry = 0;
        for i in 0..self.digits.len() {
            let digit_diff = self.digits[i] as i64 - _rhs.digits[i] as i64 - carry;
            if digit_diff < 0 {
                answer.digits.push((digit_diff + 16 as i64) as u64);
                carry = 1;
            } else {
                answer.digits.push(digit_diff as u64);
                carry = 0;
            }
        }
        while *answer.digits.last().unwrap() == 0 {
            answer.digits.pop();
        }
        return answer;
    }
}

impl Rem for BigInt {
    type Output = Self;

    fn rem(self, _rhs: Self) -> Self {
        todo!()
    }
}

impl PartialEq<Self> for BigInt {
    fn eq(&self, other: &Self) -> bool {
        if self.digits.len() != other.digits.len() {
            return false;
        }
        for i in 0..self.digits.len() {
            if self.digits[i] != other.digits[i] {
                return false;
            }
        }
        return true;
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.digits.len() < other.digits.len() {
            return Some(Ordering::Less);
        } else if self.digits.len() > other.digits.len() {
            return Some(Ordering::Greater);
        }

        for (self_digit, other_digit) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if self_digit < other_digit {
                return Some(Ordering::Less);
            } else if self_digit > other_digit {
                return Some(Ordering::Greater);
            }
        }
        return Some(Ordering::Equal);
    }
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        return BigInt{ digits: self.digits.clone() }
    }
}
