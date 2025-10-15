#![allow(non_camel_case_types)]
pub struct ix {
    pub sign: bool,
    pub vals: Vec<u64>,
}

pub fn ix_from_hex(s: &str) -> ix {
    let mut t = s.trim();
    let mut sign = false;
    if let Some(rest) = t.strip_prefix('+') {
        t = rest;
    } else if let Some(rest) = t.strip_prefix('-') {
        t = rest;
        sign = true;
    }

    if let Some(rest) = t.strip_prefix("0x") {
        t = rest;
    } else if let Some(rest) = t.strip_prefix("0X") {
        t = rest;
    }

    let mut st = t.trim_start_matches('0');
    if st.is_empty() {
        return ix { sign: false, vals: vec![0] };
    }

    let mut vals: Vec<u64> = Vec::new();
    while !st.is_empty() {
        let take_from = st.len().saturating_sub(16);
        let chunk = &st[take_from..];
        let limb = u64::from_str_radix(chunk, 16).expect("invalid hex digit in input");
        vals.push(limb);
        st = &st[..take_from];
    }
    ix { sign, vals }
}

pub fn ix_to_hex(x: &ix) -> String {

    let mut len = x.vals.len();
    while len > 0 && x.vals[len - 1] == 0 {
        len -= 1;
    }

    if len == 0 {
        return "0".to_string();
    }

    let mut s = String::new();
    s.push_str(&format!("{:x}", x.vals[len - 1]));
    for i in (0..len - 1).rev() {
        s.push_str(&format!("{:016x}", x.vals[i]));
    }

    if x.sign {
        format!("-{}", s)
    } else {
        s
    }
}

pub fn add_ix(a: &ix, b: &ix) -> ix {
    match (a.sign, b.sign) {
        (false, false) => {
            let vals = add_mag(&a.vals, &b.vals);
            let is_zero = vals.iter().all(|&w| w == 0);
            ix { sign: false, vals: if is_zero { vec![0] } else { vals } }
        }

        (true, true) => {
            let vals = add_mag(&a.vals, &b.vals);
            let is_zero = vals.iter().all(|&w| w == 0);
            ix { sign: if is_zero { false } else { true }, vals: if is_zero { vec![0] } else { vals } }
        }

        (false, true) => {
            if gte_mag(&a.vals, &b.vals) {
                let vals = sub_mag(&a.vals, &b.vals);
                let is_zero = vals.iter().all(|&w| w == 0);
                ix { sign: false, vals: if is_zero { vec![0] } else { vals } }
            } else {
                let vals = sub_mag(&b.vals, &a.vals);
                let is_zero = vals.iter().all(|&w| w == 0);
                ix { sign: if is_zero { false } else { true }, vals: if is_zero { vec![0] } else { vals } }
            }
        }

        (true, false) => {
            if gte_mag(&b.vals, &a.vals) {
                let vals = sub_mag(&b.vals, &a.vals);
                let is_zero = vals.iter().all(|&w| w == 0);
                ix { sign: false, vals: if is_zero { vec![0] } else { vals } }
            } else {
                let vals = sub_mag(&a.vals, &b.vals);
                let is_zero = vals.iter().all(|&w| w == 0);
                ix { sign: if is_zero { false } else { true }, vals: if is_zero { vec![0] } else { vals } }
            }
        }
    }
}

pub fn sub_ix(a: &ix, b: &ix) -> ix {
    let b_is_zero = b.vals.iter().all(|&w| w == 0);
    if b_is_zero {
        return ix { sign: a.sign, vals: a.vals.clone() };
    }
    let nb = ix { sign: !b.sign, vals: b.vals.clone() };
    add_ix(a, &nb)
}

pub fn mul_ix(a: &ix, b: &ix) -> ix {
    let a_zero = a.vals.iter().all(|&w| w == 0);
    let b_zero = b.vals.iter().all(|&w| w == 0);
    if a_zero || b_zero {
        return ix { sign: false, vals: vec![0] };
    }

    let n = a.vals.len();
    let m = b.vals.len();
    let mut out: Vec<u64> = vec![0; n + m];

    for i in 0..n {
        let ai = a.vals[i] as u128;
        let mut carry: u128 = 0;
        for j in 0..m {
            let acc = (out[i + j] as u128) + ai * (b.vals[j] as u128) + carry;
            out[i + j] = acc as u64;
            carry = acc >> 64;
        }

        let acc2 = (out[i + m] as u128) + carry;
        out[i + m] = acc2 as u64;
    }

    let mut k = out.len();
    while k > 1 && out[k - 1] == 0 { k -= 1; }
    out.truncate(k);

    ix { sign: a.sign ^ b.sign, vals: out }
}

fn add_mag(aug_vals: &Vec<u64>, add_vals: &Vec<u64>) -> Vec<u64> 
{
    let n = aug_vals.len().max(add_vals.len());
    let mut result = Vec::with_capacity(n + 1);
    let mut carry: u128 = 0;

    for i in 0..n {
        let x = *aug_vals.get(i).unwrap_or(&0) as u128;
        let y = *add_vals.get(i).unwrap_or(&0) as u128;
        let sum = x + y + carry;

        result.push(sum as u64);
        carry = sum >> 64;
    }

    if carry != 0 {
        result.push(carry as u64);
    }

    result
}

fn sub_mag(min_vals: &Vec<u64>, sub_vals: &Vec<u64>) -> Vec<u64> 
{
    let mut result = Vec::with_capacity(min_vals.len());
    let mut borrow: u128 = 0;

    for i in 0..min_vals.len() {
        let x = min_vals[i] as u128;
        let y = *sub_vals.get(i).unwrap_or(&0) as u128;

        let diff = (1u128 << 64) + x - y - borrow;
        result.push(diff as u64);

        borrow = if x < y + borrow { 1 } else { 0 };
    }

    while result.len() > 1 && *result.last().unwrap() == 0 {
        result.pop();
    }

    result
}

fn gte_mag(a_vals: &Vec<u64>, b_vals: &Vec<u64>) -> bool 
{
    let mut al = a_vals.len();
    while al > 0 && a_vals[al - 1] == 0 {
        al -= 1;
    }

    let mut bl = b_vals.len();
    while bl > 0 && b_vals[bl - 1] == 0 {
        bl -= 1;
    }

    if al != bl {
        return al > bl;
    }

    for i in (0..al).rev() {
        if a_vals[i] != b_vals[i] {
            return a_vals[i] > b_vals[i];
        }
    }

    true
}