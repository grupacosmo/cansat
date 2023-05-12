# LoRa Receiver 

## Installation

Python v. 3.11.2 was used for development.

I recommend to use Python virtual environment e.g. built-in `venv` for running code:

- Create a virtual environment
```sh
python -m venv .venv
```

- Enable virtual environment
  - on Windows Powershell
  ```powershell
  .\.venv\Scripts\activate
  ```
  - on Mac/Linux
  ```sh
  source .venv/bin/activate
  ```

- Install the Python dependencies on the virtual environment
```sh
python -m pip install -r requirements.txt
```
- Access the source folder
```sh
cd src
```

- Start the program
```sh
python reciever.py
```

## Sending and receiving messages

There are two Python programs. `transmitter.py` and `receiver.py`. If used together corectly few things could be obtained:

- Sending/reading any string: 
  
  In `transmitter.py` use method `lora_transmitter.send_any_string("Hello from LoRa")`.
  
  In `receiver.py` use method `lora_reciever.listen(parse="string")`.
- Comparing timestamps:
  
  In `transmitter.py` use method `lora_transmitter.send_timestamp()`.
  
  In `receiver.py` use method `lora_reciever.listen(parse="timestamp")`.

  This will allow you to compare timestamp of send and recieved message to check the delay between them. Manipulate serial port timeout, or distance between LoRa modules to check the difference
- Parsing only GPS data
  
  In `transmitter.py` use method `lora_transmitter.send_gps_data()`.
  
  In `receiver.py` use method `lora_reciever.listen(parse="gps_only")`.

  This will parse GPS data when only this data is in CSV format.
- Parsing full cansat data including
  
  In `transmitter.py` use method `lora_transmitter.send_full_canast_data()`.
  
  In `receiver.py` use method `lora_reciever.listen(parse="gps_cansat")`.

  This will parse data from cansat (all sensors) + parse the GPS data if it's embedded in CSV file with doublequotes
