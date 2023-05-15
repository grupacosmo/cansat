import serial
import pynmea2

import time
import csv
from termcolor import colored

# TODO: Test serial port behavior:

# [ ] check what is write_timeout - probably the same as UART timeout in LoRa device: AT+UART=TIMEOUT
# "AT parser inside the modem start counts from first "AT" character is received, 
# when counter overflows, a "Input timeout" event will be triggered."

# [ ] compare writelines | write - timeout, behavior 

# [ ] compare readlines | readline | read - timeout, behavior

class Receiver():
    # TODO: add saving to file
    def __init__(self, port_name, baudrate=9600, timeout=1.0):
        self.device = serial.Serial()
        self.device.port = port_name
        self.device.baudrate = baudrate
        self.device.timeout = timeout
            
        print(f"Port settings:\nName: {self.device.port}\nBaudrate: {self.device.baudrate}\nTimeout: {self.device.timeout}\n")

        input("Open port for LoRa Receiver device?\n")
        try:
            self.device.open()
            print(colored(f"Port {self.device.port} opened.\n", "green"))
        except Exception as error:
            raise RuntimeError(colored(f"Cannot open port {self.device.port}\n", "red")) from error

    def check_error(self, msg):
        error_code = msg.split(" ")[-1].translate({ord(i): None for i in '\n\r'})
        error_msg = colored(f"{error_code}: ", "red")
        match error_code:
            case "ERROR(-1)":
                error_msg += "Parameters is invalid"
                
            case "ERROR(-10)":
                error_msg += "Command unknown"
                
            case "ERROR(-11)":
                error_msg += "Command is in wrong format"
                
            case "ERROR(-12)":
                error_msg += "Command is unavailable in current mode (Check with 'AT+MODE')"
                
            case "ERROR(-20)":
                error_msg += "Too many parameters. LoRaWAN modem support max 15 parameters"
                
            case "ERROR(-21)":
                error_msg += "Length of command is too long (exceed 528 bytes)"
                
            case "ERROR(-22)":
                error_msg += "Receive end symbol timeout, command must end with <LF>"
                
            case "ERROR(-23)":
                error_msg += "Invalid character received"
                
            case "ERROR(-24)":
                error_msg += "Either length of command is too long, receive end symbol timeout or invalid character received"

        return error_msg
    
    def temp(self):
        
        cmd1 = 'AT1\n\r'
        cmd2 = 'AT\n\r'
        

        stamp1 = time.perf_counter()
        self.device.write(cmd1.encode("ascii"))
        # msg = self.device.read(64)
        # print(msg.decode("ascii"))
        
        stamp2 = time.perf_counter()
        self.device.write(cmd2.encode("ascii"))
        time.sleep(1)
        msg = self.device.readlines()
        stamp3 = time.perf_counter()

        for line in msg:
            decoded_line = line.decode("ascii")
            print(decoded_line)
            if "AT: OK" in decoded_line:
                print(colored("Success\n", "green"))
            elif "ERROR" in decoded_line:
                print(self.check_error(decoded_line))

        msg = self.device.readlines()
        stamp4 = time.perf_counter()
        for line in msg:
            decoded_line = line.decode("ascii")
            print(decoded_line)
            if "AT: OK" in decoded_line:
                print(colored("Success\n", "green"))
            elif "ERROR" in decoded_line:
                print(self.check_error(decoded_line))
         
        print(stamp2 - stamp1)
        print(stamp3 - stamp2)
        print(stamp4 - stamp3)
        
        
        
    def run_device(self):
        try:
            input("Test device connection?\n")
            cmd_ok = 'AT\r\n'
            while True:
                self.device.write(cmd_ok.encode("ascii")) 
                msg = self.device.readline().decode("ascii")
                print(msg)
                if "AT: OK" in msg:
                    print(colored("Connection test successful\n", "green"))
                    break
                elif "ERROR" in msg:
                    print(self.check_error(msg))
                    input("Try again?")
                    continue

            input("Set device in RECIEVER mode?\n")
            cmd_test = 'AT+MODE=TEST\r\n'

            self.device.write(cmd_test.encode("ascii"))
            msg = self.device.readline().decode("ascii")
            print(msg)
            if "MODE: TEST" not in msg:
                print("LoRa Exception:", colored("TEST mode not set\n", "red"))
                exit()

            cmd_receive = 'AT+TEST=RXLRPKT\r\n'

            self.device.write(cmd_receive.encode("ascii"))
            msg = self.device.readline().decode("ascii")
            print(msg)
            if "TEST: RXLRPKT" in msg:
                print(colored("Device in RECEIVER mode\n", "green"))
            else:
                print(colored("RECEIVER mode not set\n", "red"))
                exit()
        except Exception as error:
            raise error
            
    def listen(self, parse="string"):
    # TODO: Test `read_until` method instead of `read`
    # TODO: Test receiver loop with: in_waiting, out_waiting or without it.
        if parse == "string":
            input("Are You ready to listen for any string?\n")
            print(colored("RECEIVER is listening...\n", "green"))
            while True:
                while self.device.in_waiting:
                    output = self.device.readlines()
                    print(colored("Message recieved\n", "green"))
                    for line in output:
                        decoded_line = line.decode("ascii")
                        print(decoded_line)
                        if "RX" in decoded_line:
                            lines = decoded_line.split(" ")
                            hex = lines[2].replace("\"","")
                            parsed_line = bytes.fromhex(hex).decode("ascii").replace("'","\"")
                            print(colored(f"Parsed message: \n{parsed_line}\n", "light_blue"))

        elif parse == "timestamp":
            input("Are You ready to listen and compare timestamps?\n")
            print(colored("RECEIVER is listening...\n", "green"))
            while True:
                while self.device.in_waiting:
                    output = self.device.readlines()
                    print(colored("Message recieved\n", "green"))
                    for line in output:
                        decoded_line = line.decode("ascii")
                        if "RX" in decoded_line:
                            actual_time = time.time()
                            lines = decoded_line.split(" ")
                            hex = lines[2].replace("\"","")
                            parsed_line = bytes.fromhex(hex).decode("ascii")
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
                        decoded_line = line.decode("ascii")
                        if "RX" in decoded_line:
                            lines = decoded_line.split(" ")
                            hex = lines[2].replace("\"","")
                            data = bytes.fromhex(hex).decode("ascii").replace("'","\"")
                            try:
                                msg = pynmea2.parse(data)
                                if msg.mode_fix_type == "1":
                                    print(colored("No Fix", "red"))
                                elif msg.mode_fix_type == "2":
                                    print(colored("2D Fix", "light_blue"))
                                elif msg.mode_fix_type == "3":
                                    print(colored("3D Fix", "green"))
                            except pynmea2.ParseError as e:
                                print('Parse error: {}'.format(e))
                                continue
                                            
        elif parse == "gps_cansat":
            input("Are You ready to listen and parse GPS data with CANSAT other data?\n")
            print(colored("RECEIVER is listening...\n", "green"))
            while True:

                while self.device.in_waiting:
                    output = self.device.readlines()
                    print(colored("\nMessage recieved\n", "green"))
                    # TODO: clear this mess!!!
                    for line in output:
                        decoded_line = line.decode("ascii")
                        if "RX" in decoded_line:
                            lines = decoded_line.split(" ")
                            hex = lines[2].replace("\"","")
                            parsed_line = bytes.fromhex(hex).decode('ascii').replace("'","\"")
                            print(parsed_line)
                            if "$GPGGA" in parsed_line:
                                reader = csv.reader([parsed_line])
                                for row in reader:
                                    for data in row:
                                        if "$G" in data:
                                            try:
                                                # TODO: add more info from GPS
                                                msg = pynmea2.parse(data)
                                                if msg.mode_fix_type == "1":
                                                    print(colored("No Fix", "red"))
                                                elif msg.mode_fix_type == "2":
                                                    print(colored("2D Fix", "light_blue"))
                                                elif msg.mode_fix_type == "3":
                                                    print(colored("3D Fix", "green"))
                                            except pynmea2.ParseError as e:
                                                print('Parse error: {}'.format(e))
                                                continue
                            
        else:
            print(colored("Wrong parse method\n", "red"))
        
class Transmitter():
    # TODO: Finish transmitter similar to receiver
    def __init__(self, port_name, baudrate=9600, timeout=1.0):
        self.device = serial.Serial()
        self.device.port = port_name
        self.device.baudrate = baudrate
        self.device.timeout = timeout
            
        print(f"Port settings:\nName: {self.device.port}\nBaudrate: {self.device.baudrate}\nTimeout: {self.device.timeout}\n")

        input("Open port for LoRa Transmitter device?\n")
        try:
            self.device.open()
            print(colored(f"Port {self.device.port} opened.\n", "green"))
        except Exception as error:
            raise RuntimeError(colored(f"Cannot open port {self.device.port}\n", "red")) from error


if __name__ == "__main__":
    print("This is LoRa library, import desired Class to your main file")
