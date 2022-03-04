use std::io;

pub fn fibo(n: u32) -> u128 {
    if n < 2 {
        n as u128
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

// noinspection SpellCheckingInspection
pub fn get_fibo_array(n: u32) -> Vec<u128> {
    let mut array: Vec<u128> = Vec::new();

    for i in 1..(n + 1) {
        array.push(fibo(i))
    }

    array
}
