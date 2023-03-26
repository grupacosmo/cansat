use cansat_gps::Gps;
use cansat_test_utils::mock;

const FIRST_MSG: &[u8] = b"$GPGLL,,,,,,V,N*64\r\n";
const SECOND_MSG: &[u8] = b"$GPGLL,,,,,080619.00,V,N*4C\r\n";

#[test]
fn gps_last_nmea_returns_none_if_no_msg_received() {
    let uart = mock::Serial::new(FIRST_MSG.to_owned());
    let mut gps = Gps::new(uart);

    gps.read_serial().unwrap();
    gps.read_serial().unwrap();

    assert!(gps.last_nmea().is_none());
}

#[test]
fn gps_last_nmea_returns_msg_if_msg_received() {
    let uart = mock::Serial::new(FIRST_MSG.to_owned());
    let mut gps = Gps::new(uart);

    // Read until the first message is fully received (the second element of the tuple returned by read_serial() is true)
    // So as long as it is 'false', it means that the read operation has not yet been completed, and only when the value becomes 'true', it means that the entire GPS message has been read.
    // to print _read_data use 'cargo test --package cansat-gps -- --nocapture' command
    while let Ok((_read_data, false)) = gps.read_serial() {
        //print!("{}", _read_data);
    }

    // Check if the message has been successfully received (i.e., when in 'is_some()' the returned value is not empty = true).
    assert!(gps.last_nmea().is_some());
}

#[test]
fn gps_last_nmea_returns_first_msg_if_second_not_yet_terminated() {
    let uart = mock::Serial::new([FIRST_MSG, SECOND_MSG].concat());
    let mut gps = Gps::new(uart);

    // Similar to the comments above
    while let Ok((_read_data, false)) = gps.read_serial() {
        // print!("{}", _read_data);
    }

    // Check if the first message is available.
    assert_eq!(gps.last_nmea().unwrap(), FIRST_MSG);
}

#[test]
fn gps_last_nmea_returns_second_read_msg() {
    let uart = mock::Serial::new([FIRST_MSG, SECOND_MSG].concat());
    let mut gps = Gps::new(uart);

    // Similar to the comments above
    while let Ok((_read_data, false)) = gps.read_serial() {
        // print!("{}", _read_data);
    }
    while let Ok((_read_data, false)) = gps.read_serial() {
        // print!("{}", _read_data);
    }

    // Check if the second message is available.
    assert_eq!(gps.last_nmea().unwrap(), SECOND_MSG);
}
