use cansat_gps::Gps;
use cansat_test_utils::mock;

const FIRST_MSG: &[u8] = b"$GPGLL,,,,,,V,N*64\r\n";
const SECOND_MSG: &[u8] = b"$GPGLL,,,,,080619.00,V,N*4C\r\n";

#[test]
fn gps_doesnt_return_msg_if_it_wasnt_read_from_serial() {
    let uart = mock::Serial::new(FIRST_MSG.to_owned());
    let mut gps = Gps::new(uart);

    gps.read_uart().unwrap();
    gps.read_uart().unwrap();

    assert_eq!(None, gps.last_nmea());
}

#[ignore]
#[test]
fn gps_returns_msg_if_one_was_read_from_serial() {
    let uart = mock::Serial::new(FIRST_MSG.to_owned());
    let _ = Gps::new(uart);

    todo!()
}

#[ignore]
#[test]
fn gps_returns_first_msg_after_starting_reading_next_one() {
    let uart = mock::Serial::new([FIRST_MSG, SECOND_MSG].concat());
    let _ = Gps::new(uart);

    todo!()
}

#[ignore]
#[test]
fn gps_returns_second_msg_after_reading_both() {
    let uart = mock::Serial::new([FIRST_MSG, SECOND_MSG].concat());
    let _ = Gps::new(uart);

    todo!()
}
