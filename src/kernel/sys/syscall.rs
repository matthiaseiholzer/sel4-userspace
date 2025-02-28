use super::riscv_page::seL4_RISCV_Page;
use super::riscv_page_table::seL4_RISCV_PageTable;
use super::types::{
    seL4_Bool, seL4_CNode, seL4_CPtr, seL4_Error, seL4_RISCV_Page_GetAddress,
    seL4_RISCV_VMAttributes, seL4_TCB, seL4_Untyped, seL4_Word,
};
use super::types::{seL4_CapRights, seL4_UserContext};
use super::types::{seL4_Uint32, seL4_Uint8};

extern "C" {
    pub fn seL4_DebugCapIdentify(cap: seL4_CPtr) -> seL4_Uint32;

    pub fn seL4_RISCV_PageTableValue() -> usize;

    pub fn seL4_TCB_Suspend(_service: seL4_TCB) -> seL4_Error;

    pub fn seL4_TCB_Resume(_service: seL4_TCB) -> seL4_Error;

    pub fn seL4_TCB_SetMCPriority(
        _service: seL4_TCB,
        authority: seL4_TCB,
        mcp: seL4_Word,
    ) -> seL4_Error;

    pub fn seL4_TCB_SetPriority(
        _service: seL4_TCB,
        authority: seL4_TCB,
        priority: seL4_Word,
    ) -> seL4_Error;

    pub fn seL4_TCB_SetIPCBuffer(
        _service: seL4_TCB,
        buffer: seL4_Word,
        bufferFrame: seL4_CPtr,
    ) -> seL4_Error;

    pub fn seL4_DebugDumpScheduler();

    pub fn seL4_DebugNameThread(tcb: seL4_CPtr, name: *const char);

    pub fn seL4_Untyped_Retype(
        _service: seL4_Untyped,
        r#type: seL4_Word,
        size_bits: seL4_Word,
        root: seL4_CNode,
        node_index: seL4_Word,
        node_depth: seL4_Word,
        node_offset: seL4_Word,
        num_objects: seL4_Word,
    ) -> seL4_Error;

    /**
     * @xmlonly <manual name="Read Registers" label="tcb_readregisters"/> @endxmlonly
     * @brief @xmlonly Read a thread's registers into the first <texttt text="count"/> fields of a given
     * seL4_UserContext @endxmlonly
     *
     * @param[in] _service Capability to the TCB which is being operated on.
     * @param[in] suspend_source The invocation should also suspend the source thread.
     * @param[in] arch_flags Architecture dependent flags. These have no meaning on x86, ARM, and RISC-V.
     * @param[in] count The number of registers to read.
     * @param[out] regs The structure to read the registers into.
     * @return @xmlonly <errorenumdesc/> @endxmlonly
     * @retval seL4_IllegalOperation The  @xmlonly <texttt text="_service"/> @endxmlonly  is a CPtr to a capability of the wrong type.
     * Or,  @xmlonly <texttt text="_service"/> @endxmlonly  is the current thread's TCB.
     * @retval seL4_InvalidCapability The  @xmlonly <texttt text="_service"/> @endxmlonly  is a CPtr to a capability of the wrong type.
     * @retval seL4_RangeError The  @xmlonly <texttt text="count"/> @endxmlonly  requested too few or too many registers.
     */
    pub fn seL4_TCB_ReadRegisters(
        _service: seL4_TCB,
        suspend_source: seL4_Bool,
        arch_flags: seL4_Uint8,
        count: seL4_Word,
        regs: *mut seL4_UserContext,
    ) -> seL4_Error;

    /**
     * @xmlonly <manual name="Write Registers" label="tcb_writeregisters"/> @endxmlonly
     * @brief @xmlonly Set a thread's registers to the first <texttt text="count"/> fields of a given seL4_UserContext @endxmlonly
     * @param[in] _service Capability to the TCB which is being operated on.
     * @param[in] resume_target The invocation should also resume the destination thread.
     * @param[in] arch_flags Architecture dependent flags. These have no meaning on x86, ARM, and RISC-V.
     * @param[in] count The number of registers to be set.
     * @param[in] regs Data structure containing the new register values.
     * @return @xmlonly <errorenumdesc/> @endxmlonly
     * @retval seL4_IllegalOperation The  @xmlonly <texttt text="_service"/> @endxmlonly  is a CPtr to a capability of the wrong type.
     * Or,  @xmlonly <texttt text="_service"/> @endxmlonly  is the current thread's TCB.
     * @retval seL4_InvalidCapability The  @xmlonly <texttt text="_service"/> @endxmlonly  is a CPtr to a capability of the wrong type.
     */
    pub fn seL4_TCB_WriteRegisters(
        _service: seL4_TCB,
        suspend_source: seL4_Bool,
        arch_flags: seL4_Uint8,
        count: seL4_Word,
        regs: *const seL4_UserContext,
    ) -> seL4_Error;

    pub fn seL4_TCB_Configure(
        _service: seL4_TCB,
        fault_ep: seL4_Word,
        cspace_root: seL4_CNode,
        cspace_root_data: seL4_Word,
        vspace_root: seL4_CPtr,
        vspace_root_data: seL4_Word,
        buffer: seL4_Word,
        bufferFrame: seL4_CPtr,
    ) -> seL4_Error;

    pub fn seL4_TCB_SetSpace(
        _service: seL4_TCB,
        fault_ep: seL4_Word, // CPTR to the endpoint which receives IPCs when this thread faults. On MCS this cap gets copied into the TCB.
        cspace_root: seL4_CNode, // The new CSpace root.
        cspace_root_data: seL4_Word, // Optionally set the guard and guard size of the new root CNode. If set to zero, this parameter has no effect.
        vspace_root: seL4_CPtr,      // The new VSpace root.
        vspace_root_data: seL4_Word,
    ) -> seL4_Error;

    pub fn seL4_CNode_Mint(
        _service: seL4_CNode,
        dest_index: seL4_Word,
        dest_depth: seL4_Uint8,
        src_root: seL4_CNode,
        src_index: seL4_Word,
        src_depth: seL4_Uint8,
        rights: seL4_CapRights,
        badge: seL4_Word,
    ) -> seL4_Error;

    ///
    /// Copy a capability, setting its access rights whilst doing so
    /// @param[in] _service CPTR to the CNode that forms the root of the destination CSpace. Must be at a depth equivalent to the wordsize.
    /// @param[in] dest_index CPTR to the destination slot. Resolved from the root of the destination CSpace.
    /// @param[in] dest_depth Number of bits of dest_index to resolve to find the destination slot.
    /// @param[in] src_root CPTR to the CNode that forms the root of the source CSpace. Must be at a depth equivalent to the wordsize.
    /// @param[in] src_index CPTR to the source slot. Resolved from the root of the source CSpace.
    /// @param[in] src_depth Number of bits of src_index to resolve to find the source slot.
    /// @param[in] rights The rights inherited by the new capability. @xmlonly <docref>Possible values for this type are given in <autoref label="sec:cap_rights"/>  .</docref> @endxmlonly
    /// @return @xmlonly <errorenumdesc/> @endxmlonly
    /// * @retval seL4_DeleteFirst The destination slot contains a capability.
    /// @retval seL4_FailedLookup The index or depth of the source or destination is invalid  @xmlonly <docref>(see <autoref label="s:cspace-addressing"/>)</docref> @endxmlonly .
    /// Or,  @xmlonly <texttt text="src_root"/> @endxmlonly  is a CPtr to a capability of the wrong type.
    /// Or, the source slot is empty.
    /// @retval seL4_IllegalOperation The  @xmlonly <texttt text="_service"/> @endxmlonly  is a CPtr to a capability of the wrong type.
    /// Or, the source capability cannot be derived  @xmlonly <docref>(see <autoref label="sec:cap_derivation"/>)</docref> @endxmlonly .
    /// @retval seL4_InvalidCapability The  @xmlonly <texttt text="_service"/> @endxmlonly  is a CPtr to a capability of the wrong type.
    /// @retval seL4_RangeError The  @xmlonly <texttt text="dest_depth"/> @endxmlonly  or  @xmlonly <texttt text="src_depth"/> @endxmlonly  is invalid  @xmlonly <docref>(see <autoref label="s:cspace-addressing"/>)</docref> @endxmlonly .
    ///@retval seL4_RevokeFirst The source capability cannot be derived  @xmlonly <docref>(see <autoref label="sec:cap_derivation"/>)</docref> @endxmlonly .
    pub fn seL4_CNode_Copy(
        _service: seL4_CNode,
        dest_index: seL4_Word,
        dest_depth: seL4_Uint8,
        src_root: seL4_CNode,
        src_index: seL4_Word,
        src_depth: seL4_Uint8,
        rights: seL4_CapRights,
    ) -> seL4_Error;

    pub fn seL4_CNode_Move(
        _service: seL4_CNode,
        dest_index: seL4_Word,
        dest_depth: seL4_Uint8,
        src_root: seL4_CNode,
        src_index: seL4_Word,
        src_depth: seL4_Uint8,
    ) -> seL4_Error;

    pub fn seL4_CNode_Mutate(
        _service: seL4_CNode,
        dest_index: seL4_Word,
        dest_depth: seL4_Uint8,
        src_root: seL4_CNode,
        src_index: seL4_Word,
        src_depth: seL4_Uint8,
        badge: seL4_Word,
    ) -> seL4_Error;

    pub fn seL4_CNode_Delete(
        _service: seL4_CNode,
        index: seL4_Word,
        depth: seL4_Uint8,
    ) -> seL4_Error;

    pub fn seL4_RISCV_Page_Map(
        _service: seL4_RISCV_Page,
        vspace: seL4_RISCV_PageTable,
        vaddr: seL4_Word,
        rights: seL4_CapRights,
        attr: seL4_RISCV_VMAttributes,
    ) -> seL4_Error;

    pub fn seL4_RISCV_Page_Unmap(_service: seL4_RISCV_Page) -> seL4_Error;

    pub fn seL4_RISCV_Page_GetAddress(_service: seL4_RISCV_Page) -> seL4_RISCV_Page_GetAddress;

    pub fn seL4_RISCV_PageTable_Map(
        _service: seL4_RISCV_PageTable,
        vspace: seL4_RISCV_PageTable,
        vaddr: seL4_Word,
        attr: seL4_RISCV_VMAttributes,
    ) -> seL4_Error;

    pub fn seL4_RISCV_ASIDPool_Assign(_service: seL4_CPtr, vspace: seL4_CPtr) -> seL4_Error;

    pub fn riscv_sys_send_null(sys: seL4_Word, src: seL4_Word, info_arg: seL4_Word);

    pub fn riscv_sys_recv(
        sys: seL4_Word,
        src: seL4_Word,
        out_badge: &mut seL4_Word,
        out_info: &mut seL4_Word,
        out_mr0: &mut seL4_Word,
        out_mr1: &mut seL4_Word,
        out_mr2: &mut seL4_Word,
        out_mr3: &mut seL4_Word,
        reply: seL4_Word,
    );

    pub fn riscv_sys_send_recv(
        sys: seL4_Word,
        dest: seL4_Word,
        out_badge: &mut seL4_Word,
        info_arg: seL4_Word,
        out_info: &mut seL4_Word,
        in_out_mr0: &mut seL4_Word,
        in_out_mr1: &mut seL4_Word,
        in_out_mr2: &mut seL4_Word,
        in_out_mr3: &mut seL4_Word,
        reply: seL4_Word,
    );

    pub fn riscv_sys_reply(
        sys: seL4_Word,
        info_arg: seL4_Word,
        mr0: seL4_Word,
        mr1: seL4_Word,
        mr2: seL4_Word,
        mr3: seL4_Word,
    );

    //pub fn riscv_sys_recv(sys: usize, src: usize, out_badge: &mut usize, out_info: &mut usize, out_mr0: &mut usize, out_mr1: &mut usize, out_mr2: &mut usize, out_mr3: &mut usize, reply: usize);
}
