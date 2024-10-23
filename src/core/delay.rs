#![allow(dead_code)]

pub fn non_exact_time_delay(delay: u32){
    let mut count = 0u32;
    while count < delay {
        count+=1;
    }
}
