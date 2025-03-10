https://github.com/seL4/seL4_libs/blob/master/libsel4vka/include/vka/cspacepath_t.h

#pragma once

#include <inttypes.h>
#include <sel4/sel4.h>

/* --- Debug --- */

#ifdef DEBUG
#include <stdio.h>
#define CSPACEPATHPRINT     printf
#else
#define CSPACEPATHPRINT(...)
#endif

/* cspacepath_t
 * ------------
 *
 * This struct contains all the data to describe a slot in the CSpace.
 * It can be also used to describe a range of slots beginning at 'capPtr'
 * and count 'window'.
 *
  capPtr: The address/pointer to a capability.
 * capDepth: The depth of the address. (Most used value 32-Bit.)
 * root:   Capability pointer to the root CNode in our CSpace.
 * window: Count of caps in the range, beginning with 'capPtr'.
 *
 * !! Now this is where things are getting complicated. !!
 * dest:   Capability pointer to the destination CNode. Resolved relative
 *         to the root parameter.
 * depth:  Number of bits of dest to translate when addressing the
 *         destination CNode.
 * offset: Number of slots into the node at which capabilities start being
 *         placed
 * -------------------------------
 * ??? But what does this mean ???
 * -------------------------------
 * Example: Let's assume a three level cspace layout
 *          ROOT [  |A|              ] 20 = bitlength
 *                   |
 *                   v
 *          A        [  |B|          ]  8 = bitlength
 *                       |
 *                       v
 *          B            [  |x|      ]  4 = bitlength
 *
 *          (where bitlength = guardsize + radix)
 *
 * Let's say we want to address the capability x which lies in the CNode B.
 * That means our destination CNode is B.
 *
 * Imagine capPtr x as a concatenated value of offsets. In other words
 * take the offset value of every level and put them together starting
 * at the root.
 *
 * capPtr x  [rrrr rrrr rrrr rrrr rrrr|aaaa aaaa|bbbb]
 *           |<---------------------->|<------->|<-->|
 * bitlength            20                 8       4
 *
 *
 * In this case to address the CNode B we would get:
 *
 * dest:     [rrrr rrrr rrrr rrrr rrrr|aaaa aaaa]
 *           |<-------------------------------->|
 * depth:               20       +        8       = 28
 *
 * To address the slot where x is, we specify the offset as:
 *
 * offset:   [bbbb]
 *
 */

typedef struct _cspacepath_t {
    seL4_CPtr   capPtr;
    seL4_Word   capDepth;
    seL4_CNode  root;
    seL4_Word   dest;
    seL4_Word   destDepth;
    seL4_Word   offset;
    seL4_Word   window;
} cspacepath_t;

static inline void
cspacepath_swap(cspacepath_t *a, cspacepath_t *b)
{
    cspacepath_t tmp = *a;
    *a = *b;
    *b = tmp;
}

/// --------------------------- Debug --------------------------------
inline static void cspacepath_t_print(const cspacepath_t* src)
{
    (void)(src);
    CSPACEPATHPRINT("capPtr:     0x%" PRIxPTR "\n", src->capPtr);
    CSPACEPATHPRINT("capDepth:   0x%" PRIxPTR "\n", src->capDepth);
    CSPACEPATHPRINT("root:       0x%" PRIxPTR "\n", src->root);
    CSPACEPATHPRINT("dest:       0x%" PRIxPTR "\n", src->dest);
    CSPACEPATHPRINT("destDepth:  0x%" PRIxPTR "\n", src->destDepth);
    CSPACEPATHPRINT("offset:     0x%" PRIxPTR "\n", src->offset);
    CSPACEPATHPRINT("window:     0x%" PRIxPTR "\n", src->window);
}