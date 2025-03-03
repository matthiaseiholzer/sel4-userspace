# Userspace for seL4
seL4 is a high-assurance, high-performance operating system microkernel. It is unique because of its comprehensive formal verification, without compromising performance.
It provides fine-grained access control through capabilities, and controls communication between components of the system.
seL4's formal verification sets it apart from any other operating system. In a nutshell, it provides the highest assurance of isolation between applications running in the system, meaning that a compromise in one part of the system can be contained and prevented from harming other, potentially more critical parts of the system. 
Being a microkernel, seL4 contains none of the usual OS services; such services are
provided by programs running in user mode[sel4].

The goal of this project is to develeop userspace services, which provide this functionality in the userspace based on seL4.

## Virtual memory management
seL4 does not provide virtual memory management, beyond kernel primitives for manipulating hardware paging structures. 
Therefore, the user space services must provide creating intermediate paging structures, mapping and unmapping pages.

## High availababilty
seL4 is proven to bug free. Contrary to this, the userspace services/servers aren't proven to be bug free. If such a service crashes, it is necessary to restart it automatically.
Minix3's reincanation server [minix3] implements the functionality to restart a crashed server. A similar concept is implemented to restart crashed services automatically to reach 
high availability.

## Rust
Rust is a general-purpose programming language emphasizing performance, type safety, and concurrency. It enforces memory safety, meaning that all references point to valid memory[rust]. 
This is why it is predestined to be used as system programming language and used in this project.

[sel4]: https://sel4.systems/About/
[minix3]: https://www.minix3.org/docs/login-2010.pdf
[rust]: https://en.wikipedia.org/wiki/Rust_(programming_language)