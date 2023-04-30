import serial
import time
import csv
from pathlib import Path
from termcolor import colored

class Transmitter():
    def __init__(self, port_name, baudrate=9600, timeout=1.0):
        self.port = serial.Serial()
        self.port.port = port_name
        self.port.baudrate = baudrate
        self.port.timeout = timeout
            
        print(f"Port settings:\nName: {self.port.port}\nBaudrate: {self.port.baudrate}\nTimeout: {self.port.timeout}\n")

    
    def open_and_test_port(self):
        input("Do You want to open port for TRANSMITTER device?\n")
        try:
            self.port.open()
            print(colored(f"Port {self.port.port} opened.\n", "green"))
        except:
            print(colored(f"Cannot open port {self.port.port}.\n", "red"))
            
        input("Do You want to test device connection?\n")
        cmd_ok = 'AT\r\n'
        try:
            self.port.write(cmd_ok.encode()) 
            msg = self.port.readline().decode("utf-8")
            print(msg)
            if "AT: OK" in msg:
                print(colored("Connection test successful\n", "green"))
            else:
                print(colored("Connection test failed\n", "red"))
        except Exception as error:
            print(error)


    def set_transmitter_mode(self):
        input("Do You want to set device in TRANSMITTER TEST mode?\n")
        cmd_test = 'AT+MODE=TEST\r\n'
        try:
            self.port.write(cmd_test.encode())
            msg = self.port.readline().decode("utf-8")
            print(msg)
            if "MODE: TEST" in msg:
                print(colored("Device in TEST mode, ready to send message\n", "green"))
            else:
                print(colored("TEST mode not set\n", "red"))
        except Exception as error:
            print(error)


    def send_any_string(self, text):
        input(f"Are you ready to send the '{text}' message?\n")
        cmd_send = f'AT+TEST=TXLRSTR, "{text}"\r\n'
        try:
            self.port.write(cmd_send.encode())
            output = self.port.readlines()
            for line in output:
                print(line.decode("utf-8"))
            if any("TEST: TX DONE" in line.decode("utf-8") for line in output):
                print(colored("Message send succesfully\n", "green"))
            else:
                print(colored("Message not sent\n", "red"))
        except Exception as error:
            print(error)
       
    def send_timestamp(self):
        input(f"Are you ready to send the constant timestamps?\n")
        while True:
            try:
                cmd_send = f'AT+TEST=TXLRSTR, "{time.time()}"\r\n'
                self.port.write(cmd_send.encode())
                output = self.port.readlines()
                for line in output:
                    print(line.decode("utf-8"))
                if any("TEST: TX DONE" in line.decode("utf-8") for line in output):
                    print(colored("Message send succesfully\n", "green"))
                else:
                    print(colored("Message not sent\n", "red"))
            except Exception as error:
                print(error)
    
    def send_gps_data(self):
        input(f"Are you ready to send only GPS data?\n")
        
        path = Path(__file__).parent / "data/gps.csv"
        with path.open() as csvfile:
            reader = csv.reader(csvfile, quotechar='|')
            for row in reader:
                parsed_row = (','.join(row).replace("\"","'"))
                cmd_send=f'AT+TEST=TXLRSTR, "{parsed_row}"\r\n'
                self.port.write(cmd_send.encode())
                output = self.port.readlines()
                for line in output:
                    print(line.decode("utf-8"))
                if any("TEST: TX DONE" in line.decode("utf-8") for line in output):
                    print(colored("Message send succesfully\n", "green"))
                else:
                    print(colored("Message not sent\n", "red"))
    
    def send_full_canast_data(self):
        input(f"Are you ready to send full CANSAT data?\n")
        
        path = Path(__file__).parent / "data/cansat.csv"
        with path.open() as csvfile:
            reader = csv.reader(csvfile, quotechar='|')
            for row in reader:
                parsed_row = (','.join(row).replace("\"","'"))
                cmd_send=f'AT+TEST=TXLRSTR, "{parsed_row}"\r\n'
                self.port.write(cmd_send.encode())
                output = self.port.readlines()
                for line in output:
                    print(line.decode("utf-8"))
                if any("TEST: TX DONE" in line.decode("utf-8") for line in output):
                    print(colored("Message send succesfully\n", "green"))
                else:
                    print(colored("Message not sent\n", "red"))

        

if __name__ == "__main__":
    lora_transmitter = Transmitter("COM10", timeout=0.5) # Choose port for TRANSMITTER and timeout
    lora_transmitter.open_and_test_port()
    lora_transmitter.set_transmitter_mode()
    # lora_transmitter.send_any_string("Hello from LoRa") # Here you can choose from 'send_...' functions
    # lora_transmitter.send_timestamp()
    # lora_transmitter.send_gps_data()
    lora_transmitter.send_full_canast_data()
    