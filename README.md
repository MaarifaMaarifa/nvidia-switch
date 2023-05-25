# NVIDIA-SWITCH

Switch on or off your Nvidia GPU in your Linux machine.

## Prerequisites

For the program to work, you need to do the following steps.

- Install [acpi-call](https://github.com/mkottman/acpi_call) through your distro's package manager.
- Make sure the acpi kernel module is loaded as **nvidia-switch** makes use of it to turn switch the GPU.

## Usage

Run **nvidia-switch** with root privileges
```shell
# Turn off the gpu
nvidia-switch off

# Turn on the gpu
nvidia-switch on

# Get help information
nvidia-switch --help
  
```