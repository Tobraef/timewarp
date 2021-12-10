use chrono::prelude::*;
use chrono::Duration;

pub fn btn(bytes: &[u8]) -> u32 {
    let mut num: u32 = 0;
    let mut mult = 1;
    for b in bytes.iter().rev() {
        num += ((*b - b'0') as u32) * mult;
        mult *= 10;
    }
    num
}

pub fn ntb(num: &u32) -> [u8;2] {
    // will resolve 1 instead of b'1'
    [ ((num / 10) as u8) + b'0', ((num % 10) as u8) + b'0']
}

pub fn is_time_segment(segment: &[u8]) -> bool {
    //[0] checked
    segment[1].is_ascii_digit() && segment[2] == b':' && segment[3].is_ascii_digit() && segment[4].is_ascii_digit()
        && segment[5] == b':' && segment[6].is_ascii_digit() && segment[7].is_ascii_digit()
}

pub fn find_times(bytes: &Vec<u8>) -> Vec<usize> {
    let mut to_ret: Vec<usize> = Vec::new();
    let to_check = bytes.len() - 8;
    for (i, b) in bytes.iter().take(to_check).enumerate() {
        if *b == b'0' || *b == b'1' {
            if is_time_segment(&bytes[i..i+8]) {
                to_ret.push(i);
            }
        }
    }
    to_ret
}

pub fn assign_time(segment: &mut [u8], time: &NaiveTime) {
    let hr = ntb(&time.hour());
    let mn = ntb(&time.minute());
    let sc = ntb(&time.second());
    segment[0] = hr[0];
    segment[1] = hr[1];
    segment[3] = mn[0];
    segment[4] = mn[1];
    segment[6] = sc[0];
    segment[7] = sc[1];
}

pub fn warp_text(bytes: &mut Vec<u8>, value: &i64) -> usize {
    let segments = find_times(bytes);
    for seg_start in segments.iter() {
        let seg = &bytes[*seg_start..seg_start+8];
        let time = NaiveTime::from_hms(btn(&seg[0..2]), btn(&seg[3..5]), btn(&seg[6..]));
        let time = if *value > 0 {
            time.overflowing_add_signed(Duration::seconds(value.abs())).0 } else {
            time.overflowing_sub_signed(Duration::seconds(*value)).0
        };
        let mut to_assign: [u8;8] = [b':'; 8];
        assign_time(&mut to_assign, &time);
        let sl = &mut bytes[*seg_start..*seg_start+8];
        for (i, b) in sl.into_iter().enumerate() {
            *b = to_assign[i];
        }
    }
    segments.len()
}