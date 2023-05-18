import serial.tools.list_ports
import argparse
from termcolor import colored

from pylora import Receiver, ReceiverThreading, Transmitter


def choose_port():
    ports = sorted(serial.tools.list_ports.comports())
    num_of_ports = len(ports)
    for idx, port in enumerate(ports):
        print(f"{idx+1}) {port}")

    while True:
        port_idx = input("\nChoose Port to connect: ")
        try:
            port_idx = int(port_idx) - 1
            if port_idx in range(0, num_of_ports):
                break
        except:
            pass
        print(colored("Invalid Port", "red"))

    port_name = ports[port_idx].device
    return port_name


def choose_mode():
    print("LoRa PC has two modes:\n1) Receiver\n2) Transmitter")
    while True:
        choice = input("Choose mode: ")
        if choice == "1":
            mode = "receiver"
            break
        elif choice == "2":
            mode = "transmitter"
            break
        else:
            print(colored("Wrong input, choose 1 or 2", "red"))
            continue

    return mode


def cli_parse():
    parser = argparse.ArgumentParser(
        prog="LoRa PC",
        description="Connect with LoRa-E5 Device through USB serial port, and using device as transmitter or receiver",
    )
    parser.add_argument("-p", "--port", type=str)
    parser.add_argument("-b", "--baudrate", type=int, default=9600)
    parser.add_argument("-t", "--timeout", type=float, default=1.0)
    parser.add_argument("-m", "--mode", type=str, choices=["receiver", "transmitter"])
    parser.add_argument(
        "-msg",
        "--message",
        type=str,
        choices=["string", "timestamp", "enumerated", "csv", "lora1", "lora2"],
    )

    args = parser.parse_args()

    port = args.port
    baudrate = args.baudrate
    timeout = args.timeout
    mode = args.mode
    message = args.message

    return port, baudrate, timeout, mode, message


if __name__ == "__main__":
    port, baudrate, timeout, mode, message = cli_parse()

    if port == None:
        port = choose_port()
        print(colored(f"\nPort {port} chosen for LoRa\n", "light_blue"))

    if mode == None:
        mode = choose_mode()
        print(colored(f"{mode} mode chosen for LoRa", "light_blue"))

    # TODO: add rest of the methods
    if mode == "receiver":
        print(port, baudrate, timeout, mode)

        # Threading check
        # lora = ReceiverThreading(port, baudrate, timeout)

        # lora.start()
        # try:
        #     while True:
        #         lora.parse_msg()
        # except KeyboardInterrupt:
        #     lora.stop()

        # Normal synchronous
        lora = Receiver(port, baudrate, timeout)
        lora.listen()

    elif mode == "transmitter":
        print(port, baudrate, timeout, mode)
        lora = Transmitter(port, baudrate, timeout)
        if message:
            lora.send_message(message)
        else:
            lora.send_message()
