extern crate timewarp;

use timewarp::warp::*;
use chrono::naive::NaiveTime;

#[test]
fn time_assign() {
    let mut segment: [u8; 8] = [ 1, 2, b':', 4, 5, b':', 7, 8 ];
    let time_to_assign = NaiveTime::from_hms(12, 33, 44);
    assign_time(&mut segment, &time_to_assign);
    assert_eq!(segment, [ b'1', b'2', b':', b'3', b'3', b':', b'4', b'4' ]);
}

#[test]
fn byte_parsing() {
    let num = 10;
    let bytes = ntb(&num);
    assert_eq!(bytes, [b'1', b'0']);

    let num = 31;
    let bytes = ntb(&num);
    assert_eq!(bytes, [b'3', b'1']);

    let num = 98;
    let bytes = ntb(&num);
    assert_eq!(bytes, [b'9', b'8']);

    let bytes = [b'5', b'7'];
    let num = btn(&bytes);
    assert_eq!(num, 57);
    
    let bytes = [b'1', b'2'];
    let num = btn(&bytes);
    assert_eq!(num, 12);
    
    let bytes = [b'0', b'5'];
    let num = btn(&bytes);
    assert_eq!(num, 5);
}

#[test]
fn time_segment_detection() {
    let fragment= "alokaef fea f afmf a mf aa f 0:011 55:33 11:22 444::21::12 dsadasdads mm 11:23:55 dfadsfa 1:2:3 01:15::1".as_bytes();
    let mut segm: Vec<u8> = Vec::new();
    for b in fragment {
        segm.push(*b);
    }
    let found = find_times(&segm);
    assert_eq!(1, found.len());

    let from = found.first().unwrap();
    let to = *from + 8;
    assert_eq!("11:23:55".as_bytes(), &segm[*from..to]);
}

#[test]
fn time_add() {
    let time = NaiveTime::from_hms(11, 23, 55);
    let time = time.overflowing_add_signed(chrono::Duration::seconds(5)).0;
    assert_eq!(time, NaiveTime::from_hms(11, 24, 0));
}

#[test]
fn warp_test() {
    let fragment = "alokaef fea f afmf a mf aa f 0:011 55:33 11:22 444::21::12 dsadasdads mm 11:23:55 dfadsfa 1:2:3 01:15::1".as_bytes();
    let mut segm: Vec<u8> = Vec::new();
    for b in fragment {
        segm.push(*b);
    }
    let test_fragment_start = find_times(&segm);
    let test_fragment_start = test_fragment_start.first().unwrap();

    let value = 5;
    warp_text(&mut segm, &value);
    let fragment = "alokaef fea f afmf a mf aa f 0:011 55:33 11:22 444::21::12 dsadasdads mm 11:24:00 dfadsfa 1:2:3 01:15::1".as_bytes();
    let mut second_segm: Vec<u8> = Vec::new();
    for b in fragment {
        second_segm.push(*b);
    }

    assert_eq!(&segm[*test_fragment_start..*test_fragment_start + 8], &second_segm[*test_fragment_start..*test_fragment_start +  8]);
    assert_eq!(segm, second_segm);
}

