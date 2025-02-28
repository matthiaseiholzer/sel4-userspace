pub enum CapInit {
    Null = 0,                 /* null cap */
    InitThreadTCB = 1,        /* initial thread's TCB cap */
    InitThreadCNode = 2,      /* initial thread's root CNode cap */
    InitThreadVSpace = 3,     /* initial thread's VSpace cap */
    IRQControl = 4,           /* global IRQ controller cap */
    ASIDControl = 5,          /* global ASID controller cap */
    InitThreadASIDPool = 6,   /* initial thread's ASID pool cap */
    IOPortControl = 7,        /* global IO port control cap (null cap if not supported) */
    IOSpace = 8,              /* global IO space cap (null cap if no IOMMU support) */
    BootInfoFrame = 9,        /* bootinfo frame cap */
    InitThreadIPCBuffer = 10, /* initial thread's IPC buffer frame cap */
    Domain = 11,              /* global domain controller cap */
    SMMUSIDControl = 12,      /* global SMMU SID controller cap, null cap if not supported*/
    SMMUCBControl = 13,       /* global SMMU CB controller cap, null cap if not supported*/
    InitThreadSC = 14,        /* initial thread's scheduling context cap */
    NumInitialCaps = 15,
}
