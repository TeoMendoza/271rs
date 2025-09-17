#[macro_export]
macro_rules! choice {
    ( $e:expr, $f:expr, $g:expr ) => {
        ( ($e & $f) ^ ((!$e) & $g) )
    };
}

#[macro_export]
macro_rules! majority {
    ( $a:expr, $b:expr, $c:expr ) => {
        ( ($a & $b) | ($a & $c) | ($b & $c) )
    };
}


#[macro_export]
macro_rules! rotate_right {
    ($x:expr, $k:expr) => {{
        let x = $x;
        let k = $k;
        let n: u32 = (core::mem::size_of_val(&x) * 8) as u32;
        let r: u32 = (k as u32) % n;
        if r == 0 {
            x
        } else {
            (x >> r) | (x << (n - r))
        }
    }};
}

#[macro_export]
macro_rules! rotate_left {
    ($x:expr, $k:expr) => {{
        let x = $x;
        let k = $k;
        let n: u32 = (core::mem::size_of_val(&x) * 8) as u32;
        let r: u32 = (k as u32) % n;
        if r == 0 {
            x
        } else {
            (x << r) | (x >> (n - r))
        }
    }};
}