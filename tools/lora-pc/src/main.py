import serial.tools.list_ports
from termcolor import colored

from pylora import Reciever

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
                print("Invalid Port")
                continue
            else:
                break
        except:
            print("Invalid Port")
            continue
    
    port_name = ports[port_idx].name
    print(colored(f"\nPort {port_name} chosen for LoRa Receiver\n", "light_blue"))
    return port_name

if __name__ == "__main__":
    port_name = choose_port()
    
    lora_reciever = Reciever(port_name, timeout=0.5) # Choose port for RECEIVER and timeout
    # lora_reciever.run_device()
    # lora_reciever.listen(parse="gps_cansat") # Here you can choose from parse method: string, timestamp, gps_only or gps_cansat

# TODO: use read_until method instead of read 