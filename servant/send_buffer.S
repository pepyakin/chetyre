/*
 Adapted from [pxt-neopixel][0]

 [0]: https://github.com/Microsoft/pxt-ws2812b/blob/8346f740b179f7eb2f4f491d7f4718ce691ec666/sendBuffer.asm
*/

.syntax unified

/*
send_buf_ws2818(unused, mask, clraddr, setaddr, ptr, length)
*/
.globl send_buf_ws2818
send_buf_ws2818:

    push {r4,r5,r6,r7,lr}

    /* load ptr into r4, and len into r5  */
    ldr r4, [sp, #20]
    ldr r5, [sp, #24]

    cpsid i /* disable irq */

    /*
    at this point:
     r1 - mask
     r2 - clraddr
     r3 - setaddr
     r4 - ptr
     r5 - length
    */
    b .start

.nextbit:               /*            C0 */
    str r1, [r3, #0]    /* pin := hi  C2 */
    tst r6, r0          /*            C3 */
    bne .islate         /*            C4 */
    str r1, [r2, #0]    /* pin := lo  C6 */
.islate:
    lsrs r6, r6, #1     /* r6 >>= 1   C7 */
    bne .justbit        /*            C8 */

    # not just a bit - need new byte
    adds r4, #1         /* r4++       C9 */
    subs r5, #1         /* r5--       C10 */
    bcc .stop           /* if (r5<0) goto .stop  C11 */
.start:
    movs r6, #0x80      /* reset mask C12 */
    nop                 /*            C13 */

.common:               /*             C13 */
    str r1, [r2, #0]   /* pin := lo   C15 */
    /* always re-load byte - it just fits with the cycles better this way */
    ldrb r0, [r4, #0]  /* r0 := *r4   C17 */
    b .nextbit         /*             C20 */

.justbit: /* C10 */
    /* no nops, branch taken is already 3 cycles */
    b .common /* C13 */

.stop:
    str r1, [r2, #0]   /* pin := lo */
    cpsie i            /* enable irq */

    pop {r4, r5, r6,r7,pc}
