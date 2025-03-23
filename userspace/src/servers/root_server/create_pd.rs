use crate::runtime::cap_space::cap_rights::CapRights;
use crate::runtime::cap_space::{CapAddr, CapData, CapInit, CapSpaceManager};
use crate::runtime::kernel::{BootInfo, IPCBuffer, Kernel, UntypedType, UserContext};
use crate::runtime::protection_domain::thread::{Thread, ThreadIdManager};
use crate::runtime::protection_domain::{ProtectionDomain, ProtectionDomainInner};
use crate::runtime::va_space::page::Attributes;
use crate::runtime::va_space::VASpaceManager;
use crate::servers::root_server::untyped_memory_manager::UntypedMemoryManager;
use core::default::Default;
use core::ptr;

pub fn create_pd<K: Kernel>(
    kernel: K,
    boot_info: &BootInfo,
    mut untyped_memory_manager: UntypedMemoryManager,
    cap_space_root_idx: CapAddr,
    tcb_space_idx: CapAddr,
    ep_space_idx: CapAddr,
    c_t0: CapAddr,
    c_t1: CapAddr,
    main_thread: fn(&mut Thread<K>, boot_info: &BootInfo) -> !,
) {
    let root_server_cspace_root = CapAddr::from(CapInit::InitThreadCNode as usize, 64);
    let (untyped_cap, _) = untyped_memory_manager.alloc(30).unwrap();

    // Create new cap table
    let reincarnation_server_cap_space_root = cap_space_root_idx;
    {
        let _ = kernel.retype(
            untyped_cap,
            UntypedType::CapTableObject,
            CapSpaceManager::ROOT_WIDTH_BITS,
            root_server_cspace_root,
            0,
            0,
            reincarnation_server_cap_space_root.addr,
            1,
        );
    }

    // ASID Pool Cap
    {
        // let _ = kernel.copy(
        //     reincarnation_server_cap_space_root,
        //     CapAddr::from(CapSpaceManager::C_ASID_OFFSET, 7),
        //     root_server_cspace_root,
        //     CapAddr::from(CapInit::InitThreadASIDPool as usize, 64),
        //     CapRights::default(),
        // );

        // let _ = kernel.copy(
        //     reincarnation_server_cap_space_root,
        //     CapAddr::from(CapSpaceManager::CSPACE_CAP, 7),
        //     root_server_cspace_root,
        //     reincarnation_server_cap_space_root,
        //     CapRights::default(),
        // );
    }

    // c_0
    {
        let _ = kernel.retype(
            untyped_cap,
            UntypedType::CapTableObject,
            CapSpaceManager::L0_WIDTH_BITS,
            reincarnation_server_cap_space_root,
            0,
            0,
            CapSpaceManager::C_0_OFFSET,
            1,
        );
    }

    // c_v_0
    {
        let _ = kernel.retype(
            untyped_cap,
            UntypedType::RiscvPageTableObject,
            0,
            reincarnation_server_cap_space_root,
            0,
            0,
            CapSpaceManager::C_V_0_OFFSET,
            1,
        );
    }

    //Assign to ASID Pool
    {
        let _ = kernel.r#move(
            root_server_cspace_root,
            c_t0,
            reincarnation_server_cap_space_root,
            CapAddr::from(
                CapSpaceManager::C_V_0_OFFSET,
                CapSpaceManager::ROOT_WIDTH_BITS,
            ),
        );

        let _ = kernel.assign(
            CapAddr::from(CapInit::InitThreadASIDPool as usize, 64),
            c_t0,
        );

        let _ = kernel.r#move(
            reincarnation_server_cap_space_root,
            CapAddr::from(
                CapSpaceManager::C_V_0_OFFSET,
                CapSpaceManager::ROOT_WIDTH_BITS,
            ),
            root_server_cspace_root,
            c_t0,
        );
    }

    // c_0_0
    {
        let _ = kernel.retype(
            untyped_cap,
            UntypedType::CapTableObject,
            CapSpaceManager::L1_WIDTH_BITS,
            reincarnation_server_cap_space_root,
            CapSpaceManager::C_0_OFFSET,
            CapSpaceManager::ROOT_WIDTH_BITS,
            0,
            1,
        );
    }

    // c_v_0_1
    {
        let _ = kernel.retype(
            untyped_cap,
            UntypedType::RiscvPageTableObject,
            0,
            reincarnation_server_cap_space_root,
            CapSpaceManager::C_0_OFFSET,
            CapSpaceManager::ROOT_WIDTH_BITS,
            1,
            1,
        );
    }

    // v_0_1
    map_page_table(
        kernel.clone(),
        root_server_cspace_root,
        reincarnation_server_cap_space_root,
        c_t0,
        c_t1,
        CapAddr::from(
            CapSpaceManager::C_V_0_OFFSET,
            CapSpaceManager::ROOT_WIDTH_BITS,
        ),
        CapAddr::from(
            (CapSpaceManager::C_0_OFFSET << CapSpaceManager::L0_WIDTH_BITS) | 1,
            CapSpaceManager::ROOT_WIDTH_BITS + CapSpaceManager::L0_WIDTH_BITS,
        ),
        0,
    );

    // c_0_0_0
    {
        let _ = kernel.retype(
            untyped_cap,
            UntypedType::CapTableObject,
            CapSpaceManager::L2_WIDTH_BITS,
            reincarnation_server_cap_space_root,
            CapSpaceManager::C_0_OFFSET << CapSpaceManager::L0_WIDTH_BITS | 0,
            CapSpaceManager::ROOT_WIDTH_BITS + CapSpaceManager::L0_WIDTH_BITS,
            0,
            1,
        );
    }

    // c_v_0_0_1
    {
        let _ = kernel.retype(
            untyped_cap,
            UntypedType::RiscvPageTableObject,
            0,
            reincarnation_server_cap_space_root,
            CapSpaceManager::C_0_OFFSET << CapSpaceManager::L0_WIDTH_BITS | 0,
            CapSpaceManager::ROOT_WIDTH_BITS + CapSpaceManager::L0_WIDTH_BITS,
            1,
            1,
        );
    }

    // v_0_0_1
    map_page_table(
        kernel.clone(),
        root_server_cspace_root,
        reincarnation_server_cap_space_root,
        c_t0,
        c_t1,
        CapAddr::from(
            CapSpaceManager::C_V_0_OFFSET,
            CapSpaceManager::ROOT_WIDTH_BITS,
        ),
        CapAddr::from(
            (CapSpaceManager::C_0_OFFSET
                << (CapSpaceManager::L0_WIDTH_BITS + CapSpaceManager::L1_WIDTH_BITS))
                | 1,
            CapSpaceManager::ROOT_WIDTH_BITS
                + CapSpaceManager::L0_WIDTH_BITS
                + CapSpaceManager::L1_WIDTH_BITS,
        ),
        0,
    );

    let user_image_frames = &boot_info.user_image_frames;

    for i in user_image_frames.start..user_image_frames.end {
        let idx_l2 = i - user_image_frames.start;
        let idx_l1 = 0;
        let idx_l0 = 0;

        let rights = CapRights::all_rights();
        let c_addr = CapSpaceManager::cap_addr_l012(idx_l0, idx_l1, idx_l2);

        let c_v_0 = CapAddr::from(
            CapSpaceManager::C_V_0_OFFSET,
            CapSpaceManager::ROOT_WIDTH_BITS,
        );

        let _ = kernel.copy(
            reincarnation_server_cap_space_root,
            c_addr,
            root_server_cspace_root,
            CapAddr::from(i, 64),
            rights.clone(),
        );

        let _ = kernel.r#move(
            root_server_cspace_root,
            c_t0,
            reincarnation_server_cap_space_root,
            c_v_0,
        );

        let _ = kernel.r#move(
            root_server_cspace_root,
            c_t1,
            reincarnation_server_cap_space_root,
            c_addr,
        );

        // ELF-loading image 'sel4_os' to 84027000
        // paddr=[84027000..840cffff]
        // vaddr=[10000..b8fff]
        // virt_entry=17fb6
        // 0x10000 / 4096 = 16
        let vaddr = (idx_l2 + 16) * VASpaceManager::PAGE_SIZE_BYTES;
        let _ = kernel.map_page(c_t1, c_t0, vaddr, CapRights::default(), Attributes::Default);

        let _ = kernel.r#move(
            reincarnation_server_cap_space_root,
            c_addr,
            root_server_cspace_root,
            c_t1,
        );

        let _ = kernel.r#move(
            reincarnation_server_cap_space_root,
            c_v_0,
            root_server_cspace_root,
            c_t0,
        );
    }

    // C_PD_0_510
    {
        let _ = kernel.retype(
            untyped_cap.clone(),
            UntypedType::CapTableObject,
            CapSpaceManager::L1_WIDTH_BITS,
            reincarnation_server_cap_space_root,
            CapSpaceManager::C_0_OFFSET,
            CapSpaceManager::ROOT_WIDTH_BITS,
            CapSpaceManager::C_PD_0_OFFSET,
            1,
        );
    }

    // C_V_PD_0_511
    {
        let _ = kernel.retype(
            untyped_cap,
            UntypedType::RiscvPageTableObject,
            0,
            reincarnation_server_cap_space_root,
            CapSpaceManager::C_0_OFFSET,
            CapSpaceManager::ROOT_WIDTH_BITS,
            CapSpaceManager::C_V_PD_0_OFFSET,
            1,
        );

        let page_table_root = CapAddr::from(
            CapSpaceManager::C_V_0_OFFSET,
            CapSpaceManager::ROOT_WIDTH_BITS,
        );

        let page_table = CapSpaceManager::cap_addr_l0(CapSpaceManager::C_V_PD_0_OFFSET);

        map_page_table(
            kernel.clone(),
            root_server_cspace_root,
            reincarnation_server_cap_space_root,
            c_t0,
            c_t1,
            page_table_root,
            page_table,
            VASpaceManager::PD_ADDRESS,
        );
    }

    // c_PD_0_511_0
    {
        let c_addr = CapSpaceManager::cap_addr_l0(CapSpaceManager::C_PD_0_OFFSET);
        let _ = kernel.retype(
            untyped_cap.clone(),
            UntypedType::CapTableObject,
            CapSpaceManager::L2_WIDTH_BITS,
            reincarnation_server_cap_space_root,
            c_addr.addr,
            c_addr.depth,
            CapSpaceManager::C_PD_1_OFFSET,
            1,
        );

        let _ = kernel.retype(
            untyped_cap,
            UntypedType::RiscvPageTableObject,
            0,
            reincarnation_server_cap_space_root,
            c_addr.addr,
            c_addr.depth,
            CapSpaceManager::C_V_PD_1_OFFSET,
            1,
        );

        let page_table_root = CapAddr::from(
            CapSpaceManager::C_V_0_OFFSET,
            CapSpaceManager::ROOT_WIDTH_BITS,
        );
        let page_table = CapSpaceManager::cap_addr_l01(
            CapSpaceManager::C_PD_0_OFFSET,
            CapSpaceManager::C_V_PD_1_OFFSET,
        );

        map_page_table(
            kernel.clone(),
            root_server_cspace_root,
            reincarnation_server_cap_space_root,
            c_t0,
            c_t1,
            page_table_root,
            page_table,
            VASpaceManager::PD_ADDRESS,
        );
    }

    // Create Protection Domain Object
    const PD_ADDRESS: usize = VASpaceManager::PD_ADDRESS;
    {
        let cap = CapSpaceManager::cap_addr_l01(
            CapSpaceManager::C_PD_0_OFFSET,
            CapSpaceManager::C_PD_1_OFFSET,
        );
        let _ = kernel.retype(
            untyped_cap.clone(),
            UntypedType::Riscv4KPage,
            0,
            reincarnation_server_cap_space_root,
            cap.addr,
            cap.depth,
            CapSpaceManager::C_V_PD_2_OFFSET,
            1,
        );

        let _ = kernel.r#move(
            root_server_cspace_root,
            c_t0,
            reincarnation_server_cap_space_root,
            CapAddr::from(
                CapSpaceManager::C_V_0_OFFSET,
                CapSpaceManager::ROOT_WIDTH_BITS,
            ),
        );

        let c_addr = CapSpaceManager::cap_addr_l012(
            CapSpaceManager::C_PD_0_OFFSET,
            CapSpaceManager::C_PD_1_OFFSET,
            CapSpaceManager::C_V_PD_2_OFFSET,
        );

        let _ = kernel.r#move(
            root_server_cspace_root,
            c_t1,
            reincarnation_server_cap_space_root,
            c_addr,
        );

        // Map the new storage to the local address space, write the init state and unmap the storage
        {
            const PD_ADDRESS_TEMP: usize = 511 * VASpaceManager::PAGE_SIZE_BYTES;
            let _ = kernel.map_page(
                c_t1,
                CapAddr::from_const(CapInit::InitThreadVSpace as usize),
                PD_ADDRESS_TEMP,
                CapRights::default(),
                Attributes::Default,
            );
            let address = PD_ADDRESS_TEMP as *mut ProtectionDomainInner<K>;
            {
                let mut thread_id_manager = ThreadIdManager::default();
                // first id is already used by this thread
                let _ = thread_id_manager.alloc_id();
                let pd = ProtectionDomainInner::new(
                    1,
                    kernel.clone(),
                    CapSpaceManager::default(),
                    VASpaceManager::default(),
                    thread_id_manager,
                );

                unsafe { ptr::write_volatile(address, pd) };
            }
            let _ = kernel.unmap_page(c_t1);
        }

        let _ = kernel.map_page(
            c_t1,
            c_t0,
            PD_ADDRESS,
            CapRights::default(),
            Attributes::Default,
        );

        let _ = kernel.r#move(
            reincarnation_server_cap_space_root,
            c_addr,
            root_server_cspace_root,
            c_t1,
        );

        let _ = kernel.r#move(
            reincarnation_server_cap_space_root,
            CapAddr::from(
                CapSpaceManager::C_V_0_OFFSET,
                CapSpaceManager::ROOT_WIDTH_BITS,
            ),
            root_server_cspace_root,
            c_t0,
        );
    }

    // L2 Page Table for Main Thread
    {
        {
            let c_addr = CapSpaceManager::cap_addr_l0(CapSpaceManager::C_MT_0_OFFSET);
            let _ = kernel.retype(
                untyped_cap.clone(),
                UntypedType::CapTableObject,
                CapSpaceManager::L2_WIDTH_BITS,
                reincarnation_server_cap_space_root,
                c_addr.addr,
                c_addr.depth,
                CapSpaceManager::C_MT_1_OFFSET,
                1,
            );

            let _ = kernel.retype(
                untyped_cap,
                UntypedType::RiscvPageTableObject,
                0,
                reincarnation_server_cap_space_root,
                c_addr.addr,
                c_addr.depth,
                CapSpaceManager::C_V_MT_1_OFFSET,
                1,
            );
        }

        let page_table_root = CapAddr::from(
            CapSpaceManager::C_V_0_OFFSET,
            CapSpaceManager::ROOT_WIDTH_BITS,
        );

        let page_table = CapSpaceManager::cap_addr_l01(
            CapSpaceManager::C_MT_0_OFFSET,
            CapSpaceManager::C_V_MT_1_OFFSET,
        );

        map_page_table(
            kernel.clone(),
            root_server_cspace_root,
            reincarnation_server_cap_space_root,
            c_t0,
            c_t1,
            page_table_root,
            page_table,
            VASpaceManager::MT_ADDRESS,
        );
    }

    let page_table_root_cap = CapAddr::from(
        CapSpaceManager::C_V_0_OFFSET,
        CapSpaceManager::ROOT_WIDTH_BITS,
    );

    // Main Thread IPC Buffer
    const MT_IPC_BUFFER_ADDRESS: usize = VASpaceManager::MT_IPC_BUFFER_ADDRESS;
    let c_ipc_buffer: CapAddr = CapSpaceManager::cap_addr_l012(
        CapSpaceManager::C_MT_IPC_BUFFER_0_OFFSET,
        CapSpaceManager::C_MT_IPC_BUFFER_1_OFFSET,
        CapSpaceManager::C_V_MT_IPC_BUFFER_2_OFFSET,
    );

    {
        {
            let cap = CapSpaceManager::cap_addr_l01(
                CapSpaceManager::C_MT_IPC_BUFFER_0_OFFSET,
                CapSpaceManager::C_MT_IPC_BUFFER_1_OFFSET,
            );

            let _ = kernel.retype(
                untyped_cap,
                UntypedType::Riscv4KPage,
                0,
                reincarnation_server_cap_space_root,
                cap.addr,
                cap.depth,
                CapSpaceManager::C_V_MT_IPC_BUFFER_2_OFFSET,
                1,
            );
        }
        let _ = kernel.r#move(
            root_server_cspace_root,
            c_t0,
            reincarnation_server_cap_space_root,
            page_table_root_cap,
        );

        let _ = kernel.r#move(
            root_server_cspace_root,
            c_t1,
            reincarnation_server_cap_space_root,
            c_ipc_buffer,
        );

        {
            const IPC_TEMP: usize = 511 * VASpaceManager::PAGE_SIZE_BYTES;
            let _ = kernel.map_page(
                c_t1,
                CapAddr::from_const(CapInit::InitThreadVSpace as usize),
                IPC_TEMP,
                CapRights::default(),
                Attributes::Default,
            );
            {
                let ipc_buffer_address = IPC_TEMP as *mut IPCBuffer;
                let ipc_buffer = IPCBuffer::default();
                unsafe { ptr::write_volatile(ipc_buffer_address, ipc_buffer) };
            }
            let _ = kernel.unmap_page(c_t1);
        }

        let _ = kernel.map_page(
            c_t1,
            c_t0,
            MT_IPC_BUFFER_ADDRESS,
            CapRights::default(),
            Attributes::Default,
        );

        let _ = kernel.r#move(
            reincarnation_server_cap_space_root,
            c_ipc_buffer,
            root_server_cspace_root,
            c_t1,
        );

        let _ = kernel.r#move(
            reincarnation_server_cap_space_root,
            page_table_root_cap,
            root_server_cspace_root,
            c_t0,
        );

        // Create endpoint
        {
            let ep_cap = CapSpaceManager::cap_addr_l01(
                CapSpaceManager::C_MT_EP_0_OFFSET,
                CapSpaceManager::C_MT_EP_1_OFFSET,
            );

            let _ = kernel.retype(
                untyped_cap,
                UntypedType::EndpointObject,
                0,
                reincarnation_server_cap_space_root,
                ep_cap.addr,
                ep_cap.depth,
                CapSpaceManager::C_V_MT_EP_2_OFFSET,
                1,
            );

            let src_ep_cap = CapSpaceManager::cap_addr_l012(
                CapSpaceManager::C_MT_EP_0_OFFSET,
                CapSpaceManager::C_MT_EP_1_OFFSET,
                CapSpaceManager::C_V_MT_EP_2_OFFSET,
            );

            let _ = kernel.copy(
                root_server_cspace_root,
                ep_space_idx,
                reincarnation_server_cap_space_root,
                src_ep_cap,
                CapRights::default(),
            );
        }
    }

    // Map Stack page
    const MT_STACK_TOP: usize = VASpaceManager::MT_STACK_TOP;
    {
        for i in 0..VASpaceManager::MT_STACK_SIZE_PAGES {
            let cap = CapSpaceManager::cap_addr_l01(
                CapSpaceManager::C_MT_STACK_0_OFFSET,
                CapSpaceManager::C_MT_STACK_1_OFFSET,
            );

            let c_l2_offset = CapSpaceManager::C_V_MT_STACK_2_OFFSET + i;
            let _ = kernel.retype(
                untyped_cap,
                UntypedType::Riscv4KPage,
                0,
                reincarnation_server_cap_space_root,
                cap.addr,
                cap.depth,
                c_l2_offset,
                1,
            );

            let _ = kernel.r#move(
                root_server_cspace_root,
                c_t0,
                reincarnation_server_cap_space_root,
                page_table_root_cap,
            );

            let c_addr = CapSpaceManager::cap_addr_l012(
                CapSpaceManager::C_MT_STACK_0_OFFSET,
                CapSpaceManager::C_MT_STACK_1_OFFSET,
                c_l2_offset,
            );

            let _ = kernel.r#move(
                root_server_cspace_root,
                c_t1,
                reincarnation_server_cap_space_root,
                c_addr,
            );

            let v_addr = VASpaceManager::MT_STACK_ADDRESS + i * VASpaceManager::PAGE_SIZE_BYTES;
            let _ = kernel.map_page(
                c_t1,
                c_t0,
                v_addr,
                CapRights::default(),
                Attributes::Default,
            );

            let _ = kernel.r#move(
                reincarnation_server_cap_space_root,
                c_addr,
                root_server_cspace_root,
                c_t1,
            );

            let _ = kernel.r#move(
                reincarnation_server_cap_space_root,
                page_table_root_cap,
                root_server_cspace_root,
                c_t0,
            );
        }
    }

    //Create new TCB
    {
        let cap = CapSpaceManager::cap_addr_l01(
            CapSpaceManager::C_MT_TCB_0_OFFSET,
            CapSpaceManager::C_MT_TCB_1_OFFSET,
        );

        let _ = kernel.retype(
            untyped_cap,
            UntypedType::TCBObject,
            0,
            reincarnation_server_cap_space_root,
            cap.addr,
            cap.depth,
            CapSpaceManager::C_V_MT_TCB_2_OFFSET,
            1,
        );

        let c_addr = CapSpaceManager::cap_addr_l012(
            CapSpaceManager::C_MT_TCB_0_OFFSET,
            CapSpaceManager::C_MT_TCB_1_OFFSET,
            CapSpaceManager::C_V_MT_TCB_2_OFFSET,
        );

        let _ = kernel.r#move(
            root_server_cspace_root,
            tcb_space_idx,
            reincarnation_server_cap_space_root,
            c_addr,
        );

        let _ = kernel.r#move(
            root_server_cspace_root,
            c_t0,
            reincarnation_server_cap_space_root,
            page_table_root_cap,
        );

        let _ = kernel.r#move(
            root_server_cspace_root,
            c_t1,
            reincarnation_server_cap_space_root,
            c_ipc_buffer,
        );

        const GUARD_SIZE: u8 = (usize::BITS as u8) - CapSpaceManager::CSPACE_WIDTH_BITS;
        let badge = CapData::new(0, GUARD_SIZE);

        let _ = kernel.configure(
            tcb_space_idx,
            CapAddr::null(),
            reincarnation_server_cap_space_root,
            badge,
            c_t0,
            0,
            MT_IPC_BUFFER_ADDRESS,
            c_t1,
        );

        let _ = kernel.r#move(
            reincarnation_server_cap_space_root,
            c_ipc_buffer,
            root_server_cspace_root,
            c_t1,
        );

        let mut user_context = UserContext::default();

        user_context.pc = main_thread as usize;
        user_context.sp = MT_STACK_TOP - 8;
        user_context.a0 = MAIN_THREAD_ADDRESS;
        user_context.a1 = (boot_info as *const BootInfo) as usize;
        //user_context.tp = MAIN_THREAD_TLS_ADDRES;

        let _ = kernel.write_registers(tcb_space_idx, &user_context);

        let _ = kernel.r#move(
            reincarnation_server_cap_space_root,
            page_table_root_cap,
            root_server_cspace_root,
            c_t0,
        );

        let _ = kernel.r#copy(
            reincarnation_server_cap_space_root,
            c_addr,
            root_server_cspace_root,
            tcb_space_idx,
            CapRights::default(),
        );
    }

    // Create Main Thread Object
    const MAIN_THREAD_ADDRESS: usize = VASpaceManager::MT_ADDRESS;
    {
        {
            let cap = CapSpaceManager::cap_addr_l01(
                CapSpaceManager::C_MT_0_OFFSET,
                CapSpaceManager::C_MT_1_OFFSET,
            );

            let _ = kernel.retype(
                untyped_cap,
                UntypedType::Riscv4KPage,
                0,
                reincarnation_server_cap_space_root,
                cap.addr,
                cap.depth,
                CapSpaceManager::C_V_MT_2_OFFSET,
                1,
            );
        }

        let _ = kernel.r#move(
            root_server_cspace_root,
            c_t0,
            reincarnation_server_cap_space_root,
            page_table_root_cap,
        );

        let c_addr = CapSpaceManager::cap_addr_l012(
            CapSpaceManager::C_MT_0_OFFSET,
            CapSpaceManager::C_MT_1_OFFSET,
            CapSpaceManager::C_V_MT_2_OFFSET,
        );

        let _ = kernel.r#move(
            root_server_cspace_root,
            c_t1,
            reincarnation_server_cap_space_root,
            c_addr,
        );

        {
            const THREAD_ADDRESS_TEMP: usize = 511 * VASpaceManager::PAGE_SIZE_BYTES;
            let _ = kernel.map_page(
                c_t1,
                CapAddr::from_const(CapInit::InitThreadVSpace as usize),
                THREAD_ADDRESS_TEMP,
                CapRights::default(),
                Attributes::Default,
            );
            {
                let pd = ProtectionDomain::new(PD_ADDRESS as *mut ProtectionDomainInner<K>);
                let thread_address = THREAD_ADDRESS_TEMP as *mut Thread<K>;
                // @TODO Move whole block to init ipc_buffer
                let thread = Thread::new(
                    0,
                    CapSpaceManager::C_MT_TCB,
                    pd,
                    MT_IPC_BUFFER_ADDRESS as *mut IPCBuffer,
                );

                unsafe { ptr::write_volatile(thread_address, thread) };
            }
            let _ = kernel.unmap_page(c_t1);
        }

        let _ = kernel.map_page(
            c_t1,
            c_t0,
            MAIN_THREAD_ADDRESS,
            CapRights::default(),
            Attributes::Default,
        );

        let _ = kernel.r#move(
            reincarnation_server_cap_space_root,
            c_addr,
            root_server_cspace_root,
            c_t1,
        );

        let _ = kernel.r#move(
            reincarnation_server_cap_space_root,
            page_table_root_cap,
            root_server_cspace_root,
            c_t0,
        );
    }

    let _ = kernel.set_priority(
        tcb_space_idx,
        CapAddr::from_const(CapInit::InitThreadTCB as usize),
        255,
    );
}

fn map_page_table<K: Kernel>(
    kernel: K,
    root_server_cspace_root: CapAddr,
    reincarnation_server_cap_space_root: CapAddr,
    c_v0: CapAddr,
    c_v1: CapAddr,
    c_page_table_root: CapAddr,
    c_page_table: CapAddr,
    va: usize,
) {
    let _ = kernel.r#move(
        root_server_cspace_root,
        c_v0,
        reincarnation_server_cap_space_root,
        c_page_table_root,
    );

    let _ = kernel.r#move(
        root_server_cspace_root,
        c_v1,
        reincarnation_server_cap_space_root,
        c_page_table,
    );

    let _ = kernel.map_page_table(c_v1, c_v0, va, Attributes::Default);

    let _ = kernel.r#move(
        reincarnation_server_cap_space_root,
        c_page_table,
        root_server_cspace_root,
        c_v1,
    );

    let _ = kernel.r#move(
        reincarnation_server_cap_space_root,
        c_page_table_root,
        root_server_cspace_root,
        c_v0,
    );
}
