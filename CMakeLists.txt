include(./settings.cmake)
#sel4_tutorials_regenerate_tutorial(${CMAKE_CURRENT_SOURCE_DIR})

cmake_minimum_required(VERSION 3.26.4)
# declare the hello-world CMake project and the languages it is written in (just C)
project(sel4_userspace C ASM)

# In future tutorials, these setup steps will be replaced with
find_package(seL4 REQUIRED)
find_package(elfloader-tool REQUIRED)
find_package(musllibc REQUIRED)
find_package(util_libs REQUIRED)

sel4_import_kernel()
elfloader_import_project()

add_compile_options(
#    -mtls-dialect=desc
    -ftls-model=local-exec
)

# This sets up environment build flags and imports musllibc and runtime libraries.
musllibc_setup_build_environment_with_sel4runtime()
sel4_import_libsel4()
util_libs_import_libraries()


# Name the executable and list source files required to build it
add_executable(sel4_userspace src/constructors.c)

# List of libraries to link with the application.
target_link_libraries(
    sel4_userspace
    sel4_autoconf
    sel4runtime 
    muslc 
    sel4
    utils
    sel4runtime_Config
)

target_link_libraries(sel4_userspace ${project_dir}/target/riscv64gc-unknown-none-elf/debug/libsel4_userspace.a)
target_link_libraries(sel4_userspace -Wl,-u -Wl,register_debug_putchar)
target_include_directories(sel4_userspace PUBLIC include)


include(${project_dir}/kernel/configs/seL4Config.cmake)
set(CapDLLoaderMaxObjects 20000 CACHE STRING "" FORCE)
set(KernelRootCNodeSizeBits 16 CACHE STRING "")

ApplyData61ElfLoaderSettings(${KernelPlatform} ${KernelSel4Arch})

# Tell the build system that this application is the root task. 
include(rootserver)
DeclareRootserver(sel4_userspace)

if(SIMULATION)
    include(simulation)
    GenerateSimulateScript()
endif()


