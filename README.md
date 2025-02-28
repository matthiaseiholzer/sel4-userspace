

kernel -> libsel4 -> include -> sel4 -> functions.h -> __sel4_ipc_buffer -> remove __thread

kernel -> libsel4/src/sel4_bootinfo.c -> __sel4_ipc_buffer -> remove __thread

rust-toolchain -> 
[toolchain]
#channel = "nightly-2023-06-15"
#targets = [ "riscv64gc-unknown-none-elf" ]

Introduces a bug, while initating the timer
https://github.com/seL4/seL4/commit/69040396083959df9bb7cdc193cdf3f13af5bde8

git reset --hard e1bdd809b6d7005379d3f147f4f95ec7865e1cca

# for testing
rustup toolchain install stable-riscv64gc-unknown-linux-gnu

# Principle of least privilege

# Reincarnation Server

# Live update

A live update is an update to a service while it is active, allowing the service's code and data to be changed without affecting the environment around it. As a result, these services can be be updated at run time without requiring a system reboot.

# Dynamic Architectures

# Modularity
