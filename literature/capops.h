https://github.com/seL4/seL4_libs/blob/master/libsel4vka/include/vka/capops.h

static inline int vka_cnode_copy(const cspacepath_t *dest, const cspacepath_t *src, seL4_CapRights_t rights)
{
    return seL4_CNode_Copy(
               /* _service */      dest->root,
               /* dest_index */    dest->capPtr,
               /* destDepth */    dest->capDepth,
               /* src_root */      src->root,
               /* src_index */     src->capPtr,
               /* src_depth */     src->capDepth,
               /* rights */        rights
           );
}