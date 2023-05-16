import serial
import pynmea2
import threading
from queue import Queue

import time
import csv
from termcolor import colored

# TODO: Test serial port behavior:

# [x] check what is write_timeout - probably the same as UART timeout in LoRa device: AT+UART=TIMEOUT
# "AT parser inside the modem start counts from first "AT" character is received, 
# when counter overflows, a "Input timeout" event will be triggered."

# [ ] compare writelines | write - timeout, behavior 

# [ ] compare readlines | readline | read - timeout, behavior

class LoRa:
    def __init__(self, port_name, baudrate=9600, timeout=1.0):
        self.device = serial.Serial()
        self.device.port = port_name
        self.device.baudrate = baudrate
        self.device.timeout = timeout

        print(f"Port settings:\nName: {self.device.port}\nBaudrate: {self.device.baudrate}\nTimeout: {self.device.timeout}\n")
        self.open_port_and_test()

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

    def open_port_and_test(self):
        input("Open port for LoRa device?\n")
        try:
            self.device.open()
            print(colored(f"Port {self.device.port} opened.\n", "green"))
        
            input("Test device connection?\n")
            cmd_ok = 'AT\r\n'
            while True:
                self.device.write(cmd_ok.encode("ascii")) 
                msg = self.device.readline().decode("ascii")
                print(msg)
                if "AT: OK" in msg:
                    print(colored("Connection successful\n", "green"))
                    break
                elif "ERROR" in msg:
                    print(self.check_error(msg))
                    input("Try again?")
                    continue
            
            cmd_test = 'AT+MODE=TEST\r\n'

            self.device.write(cmd_test.encode("ascii"))
            msg = self.device.readline().decode("ascii")
            print(msg)
            if "MODE: TEST" in msg:
                    print(colored("TEST mode set\n", "green"))
            elif "ERROR" in msg:
                print(self.check_error(msg))
                exit()
    
        except Exception as error:
            raise RuntimeError(colored(f"Cannot open port {self.device.port}\n", "red")) from error
    
           
class ReceiverThreading(LoRa):
    # TODO: add saving to file
    def __init__(self, port_name, baudrate=9600, timeout=1.0):
        super().__init__(port_name, baudrate, timeout)
        
        self.data_queue = Queue()
        self.stop_event = threading.Event()
    
    def start(self):
        self.stop_event.clear()
        threading.Thread(target=self.listen).start()
        
    def stop(self):
        self.stop_event.set()
        
    def __del__(self):
        self.device.close()

            
    def listen(self):
        # TODO: Test `read_until` method instead of `read`
        try:
            input("Set device in RECIEVER mode?\n")
            cmd_receive = 'AT+TEST=RXLRPKT\r\n'

            self.device.write(cmd_receive.encode("ascii"))
            msg = self.device.readline().decode("ascii")
            print(msg)
            if "TEST: RXLRPKT" in msg:
                print(colored("Device in RECEIVER mode\n", "green"))
            elif "ERROR" in msg:
                print(self.check_error(msg))
                exit()
        except Exception as error:
            raise error

        print(colored("RECEIVER is listening...\n", "green"))   
        while True:
            if self.device.in_waiting:
                try:
                    output = self.device.readlines()
                    if output:
                        self.data_queue.put(output)
                except Exception as error:
                    print(error)
                    continue

    def parse_msg(self):
        if not self.data_queue.empty():
            output = self.data_queue.get()
            for line in output:
                decoded_line = line.decode("ascii")
                print(decoded_line)
        
        
class Receiver(LoRa):
    # TODO: add saving to file
    def __init__(self, port_name, baudrate=9600, timeout=1.0):
        super().__init__(port_name, baudrate, timeout)

    def __del__(self):
        self.device.close()

            
    def listen(self):
        # TODO: Test `read_until` method instead of `read`
        try:
            input("Set device in RECIEVER mode?\n")
            cmd_receive = 'AT+TEST=RXLRPKT\r\n'

            self.device.write(cmd_receive.encode("ascii"))
            msg = self.device.readline().decode("ascii")
            print(msg)
            if "TEST: RXLRPKT" in msg:
                print(colored("Device in RECEIVER mode\n", "green"))
            elif "ERROR" in msg:
                print(self.check_error(msg))
                exit()
        except Exception as error:
            raise error

        print(colored("RECEIVER is listening...\n", "green"))   
        while True:
            while self.device.in_waiting:
                try:
                    output = self.device.readlines()
                    self.parse_msg(output)    
                except Exception as error:
                    print(error)
                    continue

    def parse_msg(self, output):
        for line in output:
            decoded_line = line.decode("ascii")
            print(decoded_line)
    
        # if parse == "string":
        #     input("Are You ready to listen for any string?\n")
        #     print(colored("RECEIVER is listening...\n", "green"))
        #     while True:
        #             output = self.device.readlines()
        #             print(colored("Message recieved\n", "green"))
        #             for line in output:
        #                 decoded_line = line.decode("ascii")
        #                 print(decoded_line)
        #                 if "RX" in decoded_line:
        #                     lines = decoded_line.split(" ")
        #                     hex = lines[2].replace("\"","")
        #                     parsed_line = bytes.fromhex(hex).decode("ascii").replace("'","\"")
        #                     print(colored(f"Parsed message: \n{parsed_line}\n", "light_blue"))

        # elif parse == "timestamp":
        #     input("Are You ready to listen and compare timestamps?\n")
        #     print(colored("RECEIVER is listening...\n", "green"))
        #     while True:
        #         while self.device.in_waiting:
        #             output = self.device.readlines()
        #             print(colored("Message recieved\n", "green"))
        #             for line in output:
        #                 decoded_line = line.decode("ascii")
        #                 if "RX" in decoded_line:
        #                     actual_time = time.time()
        #                     lines = decoded_line.split(" ")
        #                     hex = lines[2].replace("\"","")
        #                     parsed_line = bytes.fromhex(hex).decode("ascii")
        #                     recieved_time = float(parsed_line)
        #                     print(colored(f"Recieved timestamp: {recieved_time}"))
        #                     print(colored(f"Actual timestamp: {actual_time}\n"))
        #                     print(colored(f"Time difference in seconds: \n{actual_time - recieved_time}\n", "light_yellow"))
                            
        # elif parse == "gps_only":
        #     input("Are You ready to listen and parse only GPS data?\n")
        #     print(colored("RECEIVER is listening...\n", "green"))
        #     while True:
        #         while self.device.in_waiting:
        #             output = self.device.readlines()
        #             print(colored("\nMessage recieved\n", "green"))
        #             for line in output:
        #                 decoded_line = line.decode("ascii")
        #                 if "RX" in decoded_line:
        #                     lines = decoded_line.split(" ")
        #                     hex = lines[2].replace("\"","")
        #                     data = bytes.fromhex(hex).decode("ascii").replace("'","\"")
        #                     try:
        #                         msg = pynmea2.parse(data)
        #                         if msg.mode_fix_type == "1":
        #                             print(colored("No Fix", "red"))
        #                         elif msg.mode_fix_type == "2":
        #                             print(colored("2D Fix", "light_blue"))
        #                         elif msg.mode_fix_type == "3":
        #                             print(colored("3D Fix", "green"))
        #                     except pynmea2.ParseError as e:
        #                         print('Parse error: {}'.format(e))
        #                         continue
                                            
        # elif parse == "gps_cansat":
        #     input("Are You ready to listen and parse GPS data with CANSAT other data?\n")
        #     print(colored("RECEIVER is listening...\n", "green"))
        #     while True:

        #         while self.device.in_waiting:
        #             output = self.device.readlines()
        #             print(colored("\nMessage recieved\n", "green"))
        #             # TODO: clear this mess!!!
        #             for line in output:
        #                 decoded_line = line.decode("ascii")
        #                 if "RX" in decoded_line:
        #                     lines = decoded_line.split(" ")
        #                     hex = lines[2].replace("\"","")
        #                     parsed_line = bytes.fromhex(hex).decode('ascii').replace("'","\"")
        #                     print(parsed_line)
        #                     if "$GPGGA" in parsed_line:
        #                         reader = csv.reader([parsed_line])
        #                         for row in reader:
        #                             for data in row:
        #                                 if "$G" in data:
        #                                     try:
        #                                         # TODO: add more info from GPS
        #                                         msg = pynmea2.parse(data)
        #                                         if msg.mode_fix_type == "1":
        #                                             print(colored("No Fix", "red"))
        #                                         elif msg.mode_fix_type == "2":
        #                                             print(colored("2D Fix", "light_blue"))
        #                                         elif msg.mode_fix_type == "3":
        #                                             print(colored("3D Fix", "green"))
        #                                     except pynmea2.ParseError as e:
        #                                         print('Parse error: {}'.format(e))
        #                                         continue
                            
        # else:
        #     print(colored("Wrong parse method\n", "red"))
        
        # def parse_msg(self):
        #     pass
        
        # def save_to_file(self):
        #     pass
        
        
class Transmitter(LoRa):
    def __init__(self, port_name, baudrate=9600, timeout=1.0):
        super().__init__(port_name, baudrate, timeout)
        
    def send_message(self, message_type=None):
        if message_type == None:
            print("Message type to send:\n1) Any string\n2) Timestamp\n3) CSV")
            while True:
                choice = input("Choose type: ")
                if choice == "1":
                    message_type = "string"
                    break
                elif choice == "2":
                    mode = "timestamp"
                    break
                elif choice == "3":
                    mode = "csv"
                    break
                else:
                    print(colored("Wrong input, choose 1, 2 or 3", "red"))
                    continue
        if message_type == "string":
            pass
        
        
        elif message_type == "timestamp":
            while True:
                try:
                    cmd_send = f'AT+TEST=TXLRSTR, "{time.time()}"\r\n'
                    self.device.write(cmd_send.encode())
                    output = self.device.readlines()
                    for line in output:
                        decoded_line = line.decode("ascii")
                        print(decoded_line)
                        if "ERROR" in decoded_line:
                            print(self.check_error(decoded_line))

                    if any("TEST: TX DONE" in line.decode("ascii") for line in output):
                        print(colored("Message send succesfully\n", "green"))
                    else:
                        print(colored("Message not sent\n", "red"))

                except Exception as error:
                    print(error)
        elif message_type == "csv":
            pass


if __name__ == "__main__":
    print("This is LoRa library, import desired Class to your main file")
