import serial
import time
import csv
import pynmea2
import sys
from termcolor import colored

import argparse

import serial.tools.list_ports
ports = serial.tools.list_ports.comports()

class Reciever():
    def __init__(self, port_name, baudrate=9600, timeout=1.0):
        self.device = serial.Serial()
        self.device.port = port_name
        self.device.baudrate = baudrate
        self.device.timeout = timeout
            
        print(f"Port settings:\nName: {self.device.port}\nBaudrate: {self.device.baudrate}\nTimeout: {self.device.timeout}\n")

    
    def open_and_test_port(self):
        input("Do You want to open port for RECEIVER device?\n")
        try:
            self.device.open()
            print(colored(f"Port {self.device.port} opened.\n", "green"))
        except:
            print(colored(f"Cannot open port {self.device.port}.\n", "red"))
            
        input("Do You want to test device connection?\n")
        cmd_ok = 'AT\r\n'
        try:
            self.device.write(cmd_ok.encode()) 
            msg = self.device.readline().decode("utf-8")
            print(msg)
            if "AT: OK" in msg:
                print(colored("Connection test successful\n", "green"))
            else:
                print(colored("Connection test failed\n", "red"))
        except Exception as error:
            print(error)


    def set_reciever_mode(self):
        input("Do You want to set device in TEST mode?\n")
        cmd_test = 'AT+MODE=TEST\r\n'
        try:
            self.device.write(cmd_test.encode())
            msg = self.device.readline().decode("utf-8")
            print(msg)
            if "MODE: TEST" in msg:
                print(colored("Device in TEST mode, ready to set in reciever mode\n", "green"))
            else:
                print(colored("TEST mode not set\n", "red"))
        except Exception as error:
            print(error)

        input("Do You want to set device in RECIEVER mode?\n")
        cmd_receive = 'AT+TEST=RXLRPKT\r\n'
        try:
            self.device.write(cmd_receive.encode())
            msg = self.device.readline().decode("utf-8")
            print(msg)
            if "TEST: RXLRPKT" in msg:
                print(colored("Device in RECEIVER mode, ready to recieve message\n", "green"))
            else:
                print(colored("RECEIVER mode not set\n", "red"))
        except Exception as error:
            print(error)
            
    def listen(self, parse="string"):
        if parse == "string":
            input("Are You ready to listen for any string?\n")
            print(colored("RECEIVER is listening...\n", "green"))
            while True:
                while self.device.in_waiting:
                    output = self.device.readlines()
                    print(colored("Message recieved\n", "green"))
                    for line in output:
                        decoded_line = line.decode("utf-8")
                        print(decoded_line)
                        if "RX" in decoded_line:
                            lines = decoded_line.split(" ")
                            hex = lines[2].replace("\"","")
                            parsed_line = bytes.fromhex(hex).decode('utf-8').replace("'","\"")
                            print(colored(f"Parsed message: \n{parsed_line}\n", "light_blue"))

        elif parse == "timestamp":
            input("Are You ready to listen and compare timestamps?\n")
            print(colored("RECEIVER is listening...\n", "green"))
            while True:
                while self.device.in_waiting:
                    output = self.device.readlines()
                    print(colored("Message recieved\n", "green"))
                    for line in output:
                        decoded_line = line.decode("utf-8")
                        if "RX" in decoded_line:
                            actual_time = time.time()
                            lines = decoded_line.split(" ")
                            hex = lines[2].replace("\"","")
                            parsed_line = bytes.fromhex(hex).decode('utf-8')
                            recieved_time = float(parsed_line)
                            print(colored(f"Recieved timestamp: {recieved_time}"))
                            print(colored(f"Actual timestamp: {actual_time}\n"))
                            print(colored(f"Time difference in seconds: \n{actual_time - recieved_time}\n", "light_yellow"))
                            
        elif parse == "gps_only":
            input("Are You ready to listen and parse only GPS data?\n")
            print(colored("RECEIVER is listening...\n", "green"))
            while True:
                while self.device.in_waiting:
                    output = self.device.readlines()
                    print(colored("\nMessage recieved\n", "green"))
                    for line in output:
                        decoded_line = line.decode("utf-8")
                        if "RX" in decoded_line:
                            lines = decoded_line.split(" ")
                            hex = lines[2].replace("\"","")
                            data = bytes.fromhex(hex).decode('utf-8').replace("'","\"")
                            try:
                                msg = pynmea2.parse(data)
                                timestamp = ''
                                try:
                                    timestamp = msg.timestamp
                                except AttributeError:
                                    pass

                                try:
                                    print(f"{timestamp} | Latitude: {msg.lat}")
                                except AttributeError:
                                    pass

                                try:
                                    print(f"{timestamp} | Longitude: {msg.lon}")
                                except AttributeError:
                                    pass

                                try:
                                    fix = ''
                                    if int(msg.mode_fix_type) == 1:
                                        fix = 'No Fix'
                                    elif int(msg.mode_fix_type) == 2:
                                        fix = '2D Fix'
                                    elif int(msg.mode_fix_type) == 3:
                                        fix = '3D Fix'
                                    print("GPS connection: ", fix)
                                except AttributeError:
                                    pass

                            except pynmea2.ParseError as e:
                                                print('Parse error: {}'.format(e))
                                                continue
                                            
        elif parse == "gps_cansat":
            input("Are You ready to listen and parse GPS data with CANSAT other data?\n")
            print(colored("RECEIVER is listening...\n", "green"))
            while True:
                # TODO: in_waiting, out_waiting or something else?
                # check what is write_timeout
                
                # compare writelines | write - timeout, behavior 
                # compare readlines, readline | read - timeout, behavior
                while self.device.in_waiting:
                    output = self.device.readlines()
                    print(colored("\nMessage recieved\n", "green"))
                    for line in output:
                        decoded_line = line.decode("utf-8")
                        if "RX" in decoded_line:
                            lines = decoded_line.split(" ")
                            hex = lines[2].replace("\"","")
                            parsed_line = bytes.fromhex(hex).decode('utf-8').replace("'","\"")
                            print(parsed_line)
                            if "$G" in parsed_line:
                                reader = csv.reader([parsed_line])
                                for row in reader:
                                    for data in row:
                                        if "$G" in data:
                                            try:
                                                msg = pynmea2.parse(data)
                                                timestamp = ''
                                                try:
                                                    timestamp = msg.timestamp
                                                except AttributeError:
                                                    pass

                                                try:
                                                    print(f"{timestamp} | Latitude: {msg.lat}")
                                                except AttributeError:
                                                    pass

                                                try:
                                                    print(f"{timestamp} | Longitude: {msg.lon}")
                                                except AttributeError:
                                                    pass

                                                try:
                                                    fix = ''
                                                    if int(msg.mode_fix_type) == 1:
                                                        fix = 'No Fix'
                                                    elif int(msg.mode_fix_type) == 2:
                                                        fix = '2D Fix'
                                                    elif int(msg.mode_fix_type) == 3:
                                                        fix = '3D Fix'
                                                    print("GPS connection: ", fix)
                                                except AttributeError:
                                                    pass

                                            except pynmea2.ParseError as e:
                                                print('Parse error: {}'.format(e))
                                                continue
                            
        else:
            print(colored("Wrong parse method\n", "red"))
        


if __name__ == "__main__":
    for port, desc, hwid in sorted(ports):
        print(f"{port}: {desc}")
    if len(sys.argv) < 2:
        print("Usage: python3 src/receiver.py [PORT]")
        exit()
    port_name = sys.argv[1]
    lora_reciever = Reciever(sys.argv[1], timeout=0.5) # Choose port for RECEIVER and timeout
    lora_reciever.open_and_test_port()
    lora_reciever.set_reciever_mode()
    lora_reciever.listen(parse="gps_cansat") # Here you can choose from parse method: string, timestamp, gps_only or gps_cansat

# TODO: use read_until method instead of read 