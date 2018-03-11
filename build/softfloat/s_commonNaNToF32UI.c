#include <stdint.h>
#include "platform.h"
#include "specialize.h"

/*----------------------------------------------------------------------------
| Converts the common NaN pointed to by 'aPtr' into a 32-bit floating-point
| NaN, and returns the bit pattern of this value as an unsigned integer.
*----------------------------------------------------------------------------*/
uint_fast32_t softfloat_commonNaNToF32UI( const struct commonNaN *aPtr )
{
    return (uint_fast32_t) aPtr->sign<<31 | 0x7FC00000 | aPtr->v64>>41;
}
