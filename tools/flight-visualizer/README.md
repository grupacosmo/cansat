# Flight Visualizer 
This is a tool to visualize the flight data from the flight controller.

# Usage

Flight visualizer uses stdin as input. It expects the data to be as csv with `|` as separator. It 
reads only lines that start with `>|` and ignores all other lines. Value order:
1. temperature
2. pressure
3. height
4. nmea

For example:
```
>|28.2|10123|150|$GPGGA,,,,,,0,00,99.99,,,,,,*48',-0.25439453,0.88671875,0.34960938,-0.011324655,-0.00039969373,0.003996937,1.1204091,0.2633056
>|28.5|10032|170|$GPGGA,,,,,,0,00,99.99,,,,,,*48',-0.25439453,0.88671875,0.34960938,-0.011324655,-0.00039969373,0.003996937,1.1204091,0.2633056
>|27.3|10100|201|$GPGGA,,,,,,0,00,99.99,,,,,,*48',-0.25439453,0.88671875,0.34960938,-0.011324655,-0.00039969373,0.003996937,1.1204091,0.2633056
```
1. First dataset 
   1. temperature: 28.2
   2. pressure: 10123
   3. height: 150
2. Second dataset
   1. temperature: 28.5
   2. pressure: 10032
   3. height: 170
3. Third dataset
   1. temperature: 27.3
   2. pressure: 10100
   3. height: 201

    
# Run
Just run and provide valid format on stdin

## With working lora transmitter
Build [lora-cli](../lora-cli) (see [README.md](../lora-cli/README.md)) and run:
```bash
## chose one that fits your needs

# lora-cli in release mode
../../target/release/lora-cli | cargo run

# or lora-cli in debug mode
../../target/debug/lora-cli | cargo run

# with log file (release)
../../target/release/lora-cli | tee /tmp/cansat.log | cargo run
```

## Without working lora (generated data)
Build [dummy-lora](../dummy-lora-receiver) (see [README.md](../dummy-lora-receiver/README.md)) and run:
```bash
## chose one that fits your needs

# dummy-lora-receiver in release mode
../../target/release/dummy-lora-receiver | cargo run

# or dummy-lora-receiver in debug mode
../../target/debug/dummy-lora-receiver | cargo run
```

# Code 
Structure:
- [app.rs](src/app.rs) - window and stdin processing
- [ui.rs](src/ui.rs) - drawing ui
- [data.rs](src/data.rs) - data structures

```
stdin --string--> crate::app --create::data::Data--> crate::ui --> egui
                       ^
             eframe ---'
```

