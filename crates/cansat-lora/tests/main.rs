use cansat_lora::Lora;
use cansat_test_utils::mock;

#[test]
fn receive_response() {
    let serial = mock::Serial::new(b"cmd: ok\r\nother".to_vec());
    let mut lora = Lora::new(serial);
    let mut response = [0; 32];

    lora.send(b"cmd\r\n").unwrap();
    let nwritten = lora.receive(&mut response).unwrap();

    let serial = lora.into_serial();
    assert!(&response[..nwritten] == b"cmd: ok\r\n");
    assert!(serial.tx_data.len() == b"other".len());
}

#[test]
fn drain_on_overflow_1() {
    let serial = mock::Serial::new(b"\r\n".to_vec());
    let mut lora = Lora::new(serial);
    let mut response = [0; 0];

    lora.send(b"cmd\r\n").unwrap();
    let err = lora.receive(&mut response).unwrap_err();

    let serial = lora.into_serial();
    assert_eq!(err, cansat_lora::Error::Overflow);
    assert!(serial.tx_data.len() == 0);
}

#[test]
fn drain_on_overflow_2() {
    let serial = mock::Serial::new(b"\r\n".to_vec());
    let mut lora = Lora::new(serial);
    let mut response = [0; 1];

    lora.send(b"cmd\r\n").unwrap();
    let err = lora.receive(&mut response).unwrap_err();

    let serial = lora.into_serial();
    assert_eq!(err, cansat_lora::Error::Overflow);
    assert!(serial.tx_data.len() == 0);
}

#[test]
fn drain_on_overflow_3() {
    let serial = mock::Serial::new(b"response\r\n".to_vec());
    let mut lora = Lora::new(serial);
    let mut response = [0; 3];

    lora.send(b"cmd\r\n").unwrap();
    let err = lora.receive(&mut response).unwrap_err();

    let serial = lora.into_serial();
    assert_eq!(err, cansat_lora::Error::Overflow);
    assert!(serial.tx_data.len() == 0);
}
