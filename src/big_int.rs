use std::cmp::max;
use std::i64;
use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Rem, Shl, Shr, Sub};

const BASE: u64 = 16;
const KARATSUBA_MIN_LENGTH: usize = 4;

#[derive(Debug)]
pub struct BigInt {
    pub digits: Vec<u64>,
    pub is_negative: bool,
}

impl Default for BigInt {
    fn default() -> Self {
        Self { digits: vec![], is_negative: false }
    }
}

impl BigInt {
    pub fn new() -> Self {
        Self { digits: vec![], is_negative: false }
    }

    pub fn get_hex(&self) -> String {
        let mut result = String::new();
        if self.is_negative {
            for digit in self.digits.iter().rev() {
                let test = BASE as i64 - *digit as i64 - 1;
                let mut hex_str = format!("{:x}", test);
                let end_index = hex_str.len()-1;
                hex_str = hex_str.chars().nth(end_index).unwrap().to_string();
                result.push_str(&hex_str);
            }
        } else {
            for digit in self.digits.iter().rev() {
                let hex_str = format!("{:x}", digit);
                result.push_str(&hex_str);
            }
        }
        return result;
    }

    pub fn set_hex(&mut self, number: String) -> Result<(), String> {
        if number.is_empty() {
            return Err(String::from("String is empty."));
        }
        let mut slice = number.clone();
        if number.len() > 1 &&
            ['8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'a', 'b', 'c', 'e', 'f'].contains(&number.chars().nth(0).unwrap()) {
            slice = make_it_positive(number);
            self.is_negative = true;
        }
        for item in slice.chars().rev() {
            let hex_int = u64::from_str_radix(&item.to_string(), 16).unwrap();
            self.digits.push(hex_int);
        }
        while self.digits.len() > 1 && self.digits.last().unwrap() == &0u64 {
            self.digits.pop();
        }
        return Ok(());
    }
}

impl Not for BigInt {
    type Output = Self;

    fn not(self) -> Self {
        let mut answer = BigInt::new();
        answer.is_negative = !self.is_negative;
        for digit in &self.digits {
            answer.digits.push(*digit)
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

    fn shr(self, _rhs: usize) -> Self {
        let mut answer = self.clone();
        return answer;
    }
}

impl Shl<usize> for BigInt {
    type Output = Self;

    fn shl(self, _rhs: usize) -> Self {
        let mut answer = self.clone();
        return answer;
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(mut self, mut _rhs: Self) -> Self {
        let mut answer = BigInt::new();
        if self.is_negative && !_rhs.is_negative {
            answer = _rhs - self;
        } else if !self.is_negative && _rhs.is_negative {
            answer = self - _rhs;
        } else {
            while self.digits.len() > _rhs.digits.len() {
                _rhs.digits.push(0);
            }
            while self.digits.len() < _rhs.digits.len() {
                self.digits.push(0);
            }
            let mut carry = 0;
            for i in 0..self.digits.len() {
                let digit_sum = self.digits[i] + _rhs.digits[i] + carry;
                if digit_sum >= BASE {
                    answer.digits.push(digit_sum - BASE);
                    carry = 1;
                } else {
                    answer.digits.push(digit_sum);
                    carry = 0;
                }
            }
            if carry == 1 {
                answer.digits.push(1);
            }
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
        if self.is_negative && !_rhs.is_negative {
            answer = _rhs - self;
        } else if self.is_negative && _rhs.is_negative {
            answer = self + _rhs;
        } else if !self.is_negative && _rhs.is_negative {
            _rhs.is_negative = false;
            _rhs = _rhs + BigInt{ digits: vec![1], is_negative: false };
            answer = self + _rhs;
        } else {
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
                    answer.digits.push((digit_diff + BASE as i64) as u64);
                    carry = 1;
                } else {
                    answer.digits.push(digit_diff as u64);
                    carry = 0;
                }
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

    fn rem(self, rhs: Self) -> Self {
        let mut answer = BigInt::new();

        return answer;
    }
}

impl PartialEq<Self> for BigInt {
    fn eq(&self, other: &Self) -> bool {
        if (self.is_negative != other.is_negative) ||
            (self.digits.len() != other.digits.len()){
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

impl Clone for BigInt {
    fn clone(&self) -> Self {
        return BigInt{ digits: self.digits.clone(), is_negative: self.is_negative}
    }
}

fn make_it_positive(number: String) -> String {
    let mut result = String::new();
    for item in number.chars() {
        let digit = i64::from_str_radix(&item.to_string(), 16).unwrap();
        let test = BASE as i64 - digit - 1;
        let mut hex_str = format!("{:x}", test);
        let end_index = hex_str.len()-1;
        hex_str = hex_str.chars().nth(end_index).unwrap().to_string();
        result.push_str(hex_str.as_str());
    }
    return result;
}
