#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TEST {
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
#[doc = "Possible values of the field `CONST_CARRIER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONST_CARRIERR {
    #[doc = "Constant carrier disabled."]
    DISABLED,
    #[doc = "Constant carrier enabled."]
    ENABLED,
}
impl CONST_CARRIERR {
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
            CONST_CARRIERR::DISABLED => false,
            CONST_CARRIERR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONST_CARRIERR {
        match value {
            false => CONST_CARRIERR::DISABLED,
            true => CONST_CARRIERR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CONST_CARRIERR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CONST_CARRIERR::ENABLED
    }
}
#[doc = "Possible values of the field `PLL_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_LOCKR {
    #[doc = "PLL lock disabled."]
    DISABLED,
    #[doc = "PLL lock enabled."]
    ENABLED,
}
impl PLL_LOCKR {
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
            PLL_LOCKR::DISABLED => false,
            PLL_LOCKR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL_LOCKR {
        match value {
            false => PLL_LOCKR::DISABLED,
            true => PLL_LOCKR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PLL_LOCKR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PLL_LOCKR::ENABLED
    }
}
#[doc = "Values that can be written to the field `CONST_CARRIER`"]
pub enum CONST_CARRIERW {
    #[doc = "Constant carrier disabled."]
    DISABLED,
    #[doc = "Constant carrier enabled."]
    ENABLED,
}
impl CONST_CARRIERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONST_CARRIERW::DISABLED => false,
            CONST_CARRIERW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONST_CARRIERW<'a> {
    w: &'a mut W,
}
impl<'a> _CONST_CARRIERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONST_CARRIERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Constant carrier disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CONST_CARRIERW::DISABLED)
    }
    #[doc = "Constant carrier enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CONST_CARRIERW::ENABLED)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLL_LOCK`"]
pub enum PLL_LOCKW {
    #[doc = "PLL lock disabled."]
    DISABLED,
    #[doc = "PLL lock enabled."]
    ENABLED,
}
impl PLL_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL_LOCKW::DISABLED => false,
            PLL_LOCKW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL lock disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLL_LOCKW::DISABLED)
    }
    #[doc = "PLL lock enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLL_LOCKW::ENABLED)
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
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Constant carrier. Decision point: TXEN task."]
    #[inline]
    pub fn const_carrier(&self) -> CONST_CARRIERR {
        CONST_CARRIERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PLL lock. Decision point: TXEN or RXEN task."]
    #[inline]
    pub fn pll_lock(&self) -> PLL_LOCKR {
        PLL_LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Constant carrier. Decision point: TXEN task."]
    #[inline]
    pub fn const_carrier(&mut self) -> _CONST_CARRIERW {
        _CONST_CARRIERW { w: self }
    }
    #[doc = "Bit 1 - PLL lock. Decision point: TXEN or RXEN task."]
    #[inline]
    pub fn pll_lock(&mut self) -> _PLL_LOCKW {
        _PLL_LOCKW { w: self }
    }
}
