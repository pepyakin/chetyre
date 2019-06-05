#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CRCCNF {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `LEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LENR {
    #[doc = "CRC calculation disabled."]
    DISABLED,
    #[doc = "One byte long CRC."]
    ONE,
    #[doc = "Two bytes long CRC."]
    TWO,
    #[doc = "Three bytes long CRC."]
    THREE,
}
impl LENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LENR::DISABLED => 0,
            LENR::ONE => 1,
            LENR::TWO => 2,
            LENR::THREE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LENR {
        match value {
            0 => LENR::DISABLED,
            1 => LENR::ONE,
            2 => LENR::TWO,
            3 => LENR::THREE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == LENR::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline]
    pub fn is_two(&self) -> bool {
        *self == LENR::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline]
    pub fn is_three(&self) -> bool {
        *self == LENR::THREE
    }
}
#[doc = "Possible values of the field `SKIP_ADDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKIP_ADDRR {
    #[doc = "Include packet address in CRC calculation."]
    INCLUDE,
    #[doc = "Packet address is skipped in CRC calculation. The CRC calculation will start at the first byte after the address."]
    SKIP,
}
impl SKIP_ADDRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SKIP_ADDRR::INCLUDE => false,
            SKIP_ADDRR::SKIP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SKIP_ADDRR {
        match value {
            false => SKIP_ADDRR::INCLUDE,
            true => SKIP_ADDRR::SKIP,
        }
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == SKIP_ADDRR::INCLUDE
    }
    #[doc = "Checks if the value of the field is `SKIP`"]
    #[inline]
    pub fn is_skip(&self) -> bool {
        *self == SKIP_ADDRR::SKIP
    }
}
#[doc = "Values that can be written to the field `LEN`"]
pub enum LENW {
    #[doc = "CRC calculation disabled."]
    DISABLED,
    #[doc = "One byte long CRC."]
    ONE,
    #[doc = "Two bytes long CRC."]
    TWO,
    #[doc = "Three bytes long CRC."]
    THREE,
}
impl LENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LENW::DISABLED => 0,
            LENW::ONE => 1,
            LENW::TWO => 2,
            LENW::THREE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LENW<'a> {
    w: &'a mut W,
}
impl<'a> _LENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CRC calculation disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LENW::DISABLED)
    }
    #[doc = "One byte long CRC."]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(LENW::ONE)
    }
    #[doc = "Two bytes long CRC."]
    #[inline]
    pub fn two(self) -> &'a mut W {
        self.variant(LENW::TWO)
    }
    #[doc = "Three bytes long CRC."]
    #[inline]
    pub fn three(self) -> &'a mut W {
        self.variant(LENW::THREE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SKIP_ADDR`"]
pub enum SKIP_ADDRW {
    #[doc = "Include packet address in CRC calculation."]
    INCLUDE,
    #[doc = "Packet address is skipped in CRC calculation. The CRC calculation will start at the first byte after the address."]
    SKIP,
}
impl SKIP_ADDRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SKIP_ADDRW::INCLUDE => false,
            SKIP_ADDRW::SKIP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SKIP_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _SKIP_ADDRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SKIP_ADDRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Include packet address in CRC calculation."]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(SKIP_ADDRW::INCLUDE)
    }
    #[doc = "Packet address is skipped in CRC calculation. The CRC calculation will start at the first byte after the address."]
    #[inline]
    pub fn skip(self) -> &'a mut W {
        self.variant(SKIP_ADDRW::SKIP)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - CRC length. Decision point: START task."]
    #[inline]
    pub fn len(&self) -> LENR {
        LENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Leave packet address field out of the CRC calculation. Decision point: START task."]
    #[inline]
    pub fn skip_addr(&self) -> SKIP_ADDRR {
        SKIP_ADDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - CRC length. Decision point: START task."]
    #[inline]
    pub fn len(&mut self) -> _LENW {
        _LENW { w: self }
    }
    #[doc = "Bit 8 - Leave packet address field out of the CRC calculation. Decision point: START task."]
    #[inline]
    pub fn skip_addr(&mut self) -> _SKIP_ADDRW {
        _SKIP_ADDRW { w: self }
    }
}
