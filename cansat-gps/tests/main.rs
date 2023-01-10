use cansat_gps::Gps;
use cansat_test_utils::mock;

const FIRST_MSG: &[u8] = b"$GPGLL,,,,,,V,N*64\r\n";
const SECOND_MSG: &[u8] = b"$GPGLL,,,,,080619.00,V,N*4C\r\n";

#[test]
fn gps_last_nmea_returns_none_if_no_msg_received() {
    let uart = mock::Serial::new(FIRST_MSG.to_owned());
    let mut gps = Gps::new(uart);

    gps.read_uart().unwrap();
    gps.read_uart().unwrap();

    assert!(gps.last_nmea().is_none());
}

#[ignore]
#[test]
fn gps_last_nmea_returns_msg_if_msg_received() {
    let uart = mock::Serial::new(FIRST_MSG.to_owned());
    let _ = Gps::new(uart);

    todo!()
}

#[ignore]
#[test]
fn gps_last_nmea_returns_first_msg_if_second_not_yet_terminated() {
    let uart = mock::Serial::new([FIRST_MSG, SECOND_MSG].concat());
    let _ = Gps::new(uart);

    todo!()
}

#[ignore]
#[test]
fn gps_last_nmea_returns_second_read_msg() {
    let uart = mock::Serial::new([FIRST_MSG, SECOND_MSG].concat());
    let _ = Gps::new(uart);

    todo!()
}
