# How to Run the Project

## 1. Enter USB Bootloader Mode

Before running the project, you need to put the Pico into USB Bootloader mode. Follow these steps:

1. Disconnect the Pico from the USB port.
2. Hold down the **BOOTSEL** button on the Pico.
3. While holding the button, reconnect the Pico to the USB port.
4. The Pico will now enter USB Bootloader mode.

## 2. Run the Project

Once the Pico is in USB Bootloader mode, run the following command to build and execute the project:

```bash
cargo run