#!/bin/bash

rm -rf ./build
set -e

cargo build

cmake -DCMAKE_TOOLCHAIN_FILE="./kernel/gcc.cmake" -S . -B ./build -G Ninja -DPLATFORM=qemu-riscv-virt -DSIMULATION=TRUE -DCROSS_COMPILER_PREFIX=riscv64-elf- -DRISCV64=TRUE -DKernelRiscvExtD=TRUE -DKernelRiscvExtF=TRUE -DKernelArchRiscV=TRUE -DUseRiscVOpenSBI=TRUE -DOPENSBI_PLAT_ISA=rv64imafdc_zicsr_zifencei -DSEL4_CACHE_DIR="$HOME/.sel4_cache" -DKernelDebugBuild:BOOL=ON -DCMAKE_BUILD_TYPE:STRING=Debug -DCONFIG_DEBUG_BUILD=TRUE -DKernelPrinting=TRUE -DKernelVerificationBuild=FALSE -DLibSel4FunctionAttributes=public

cmake --build ./build