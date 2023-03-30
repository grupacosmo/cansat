use cansat_core::{
    quantity::{Distance, Pressure, Temperature},
    Measurements,
};

#[test]
fn convert_measurements_to_csv() {
    let meas = Measurements {
        temperature: Some(Temperature::from_celsius(21.5)),
        pressure: Some(Pressure::from_pascals(101300.64)),
        altitude: Some(Distance::from_meters(1243.32)),
        nmea: None,
        acceleration: None,
        orientation: None,
    };

    let mut buf = [0; 1024];
    let n = meas.to_csv_record(&mut buf).unwrap();
    let record = &buf[..n];

    assert_eq!(record, b"21.5,101300.64,1243.32,,,\n");
}
