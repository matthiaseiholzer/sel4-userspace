FROM greyltc/archlinux-aur:yay

RUN pacman -Syyu --noconfirm
RUN pacman-db-upgrade
RUN pacman -S --noconfirm cargo rustup cmake ninja riscv64-elf-gcc qemu-system-riscv qemu-system-riscv-firmware python python-yaml cpio
RUN aur-install python-sel4-deps python-pyfdt python-future python-guardonce python-pyelftools cmake-format
RUN rustup default stable
RUN cargo --version
RUN rustup target install riscv64gc-unknown-none-elf
COPY . .
