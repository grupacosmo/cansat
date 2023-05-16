import serial.tools.list_ports
import argparse
import threading
from termcolor import colored

from pylora import Receiver, Transmitter

def choose_port():
    ports = sorted(serial.tools.list_ports.comports())
    num_of_ports = len(ports)
    for idx, port in enumerate(ports):
        print(f"{idx+1}) {port}")

    while True:
        port_idx = input("\nChoose Port to connect: ")
        try:
            port_idx = int(port_idx) - 1   
            if port_idx not in range(0, num_of_ports):
                print(colored("Invalid Port", "red"))
                continue
            else:
                break
        except:
            print(colored("Invalid Port", "red"))
            continue
    
    port_name = ports[port_idx].name
    print(colored(f"\nPort {port_name} chosen for LoRa\n", "light_blue"))
    return port_name

def choose_mode():
    print("LoRa PC has two modes:\n1) Receiver\n2) Transmitter")
    while True:
        choice = input("Choose mode: ")
        if choice == "1":
            mode = "receiver"
            print(colored("Receiver mode chosen for LoRa", "light_blue"))
            break
        elif choice == "2":
            mode = "transmitter"
            print(colored("Transmitter mode chosen for LoRa", "light_blue"))
            break
        else:
            print(colored("Wrong input, choose 1 or 2", "red"))
            continue

    return mode


def cli_parser():
    parser = argparse.ArgumentParser(
                    prog='LoRa PC',
                    description='Connect with LoRa-E5 Device through USB serial port, and using device as transmitter or receiver',
                    )
    parser.add_argument('-p', '--port', type=str)
    parser.add_argument('-b', '--baudrate', type=int, default=9600)
    parser.add_argument('-t', '--timeout', type=float, default=1.0)
    parser.add_argument('-m', '--mode', type=str, choices=['receiver', 'transmitter'])
    
    args = parser.parse_args()
    
    port = args.port
    baudrate = args.baudrate
    timeout = args.timeout
    mode = args.mode
    
    return port, baudrate, timeout, mode

if __name__ == "__main__":
    
    port, baudrate, timeout, mode = cli_parser()
    
    if port == None:
        port = choose_port()
    
    if mode == None:
        mode = choose_mode()
        
    # TODO: add rest of the methods
    if mode == 'receiver':
        print(port, baudrate, timeout, mode)
        lora = Receiver(port, baudrate, timeout)
        
        listening_thread = threading.Thread(target=lora.listen)
        listening_thread.start()
        
        parsing_thread = threading.Thread(target=lora.parse_msg)
        parsing_thread.start()
    
    elif mode == 'transmitter':
        print(port, baudrate, timeout, mode)
        lora = Transmitter(port, baudrate, timeout)
        lora.send_message()
    

