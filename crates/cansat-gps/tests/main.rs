use cansat_gps::Gps;
use cansat_test_utils::mock;
use rand::Rng;

const FIRST_MSG: &[u8] = b"$GPGLL,,,,,,V,N*64\r\n";
const SECOND_MSG: &[u8] = b"$GPGLL,,,,,080619.00,V,N*4C\r\n";

#[test]
fn gps_last_nmea_returns_none_if_no_msg_received() {
    let uart = mock::Serial::new(FIRST_MSG.to_owned());
    let mut gps: Gps<_, 82> = Gps::new(uart);

    gps.read_serial().unwrap();
    gps.read_serial().unwrap();

    assert!(gps.last_nmea().is_none());
}

#[test]
fn gps_last_nmea_returns_msg_if_msg_received() {
    let uart = mock::Serial::new(FIRST_MSG.to_owned());
    let mut gps: Gps<_, 82> = Gps::new(uart);

    while let (_, _is_msg_terminated @ false) = gps.read_serial().unwrap() {}

    assert_eq!(gps.last_nmea().unwrap(), FIRST_MSG);
}

#[test]
fn gps_last_nmea_returns_first_msg_if_second_not_yet_terminated() {
    let uart = mock::Serial::new([FIRST_MSG, SECOND_MSG].concat());
    let mut gps: Gps<_, 82> = Gps::new(uart);
    let mut rng = rand::thread_rng();

    while let (_, _is_msg_terminated @ false) = gps.read_serial().unwrap() {}

    for _ in 0..rng.gen_range(1..SECOND_MSG.len()) {
        gps.read_serial().unwrap();
    }

    assert_eq!(gps.last_nmea().unwrap(), FIRST_MSG);
}

#[test]
fn gps_last_nmea_returns_second_read_msg() {
    let uart = mock::Serial::new([FIRST_MSG, SECOND_MSG].concat());
    let mut gps: Gps<_, 82> = Gps::new(uart);

    while let (_, _is_msg_terminated @ false) = gps.read_serial().unwrap() {}
    while let (_, _is_msg_terminated @ false) = gps.read_serial().unwrap() {}

    assert_eq!(gps.last_nmea().unwrap(), SECOND_MSG);
}
