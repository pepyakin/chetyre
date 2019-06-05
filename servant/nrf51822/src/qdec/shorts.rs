#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHORTS {
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
#[doc = "Possible values of the field `REPORTRDY_READCLRACC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_READCLRACCR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl REPORTRDY_READCLRACCR {
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
            REPORTRDY_READCLRACCR::DISABLED => false,
            REPORTRDY_READCLRACCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REPORTRDY_READCLRACCR {
        match value {
            false => REPORTRDY_READCLRACCR::DISABLED,
            true => REPORTRDY_READCLRACCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == REPORTRDY_READCLRACCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == REPORTRDY_READCLRACCR::ENABLED
    }
}
#[doc = "Possible values of the field `SAMPLERDY_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDY_STOPR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl SAMPLERDY_STOPR {
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
            SAMPLERDY_STOPR::DISABLED => false,
            SAMPLERDY_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAMPLERDY_STOPR {
        match value {
            false => SAMPLERDY_STOPR::DISABLED,
            true => SAMPLERDY_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SAMPLERDY_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SAMPLERDY_STOPR::ENABLED
    }
}
#[doc = "Values that can be written to the field `REPORTRDY_READCLRACC`"]
pub enum REPORTRDY_READCLRACCW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl REPORTRDY_READCLRACCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REPORTRDY_READCLRACCW::DISABLED => false,
            REPORTRDY_READCLRACCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REPORTRDY_READCLRACCW<'a> {
    w: &'a mut W,
}
impl<'a> _REPORTRDY_READCLRACCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REPORTRDY_READCLRACCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPORTRDY_READCLRACCW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPORTRDY_READCLRACCW::ENABLED)
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
#[doc = "Values that can be written to the field `SAMPLERDY_STOP`"]
pub enum SAMPLERDY_STOPW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl SAMPLERDY_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAMPLERDY_STOPW::DISABLED => false,
            SAMPLERDY_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMPLERDY_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLERDY_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMPLERDY_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_STOPW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_STOPW::ENABLED)
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
    #[doc = "Bit 0 - Short-cut between REPORTRDY event and READCLRACC task."]
    #[inline]
    pub fn reportrdy_readclracc(&self) -> REPORTRDY_READCLRACCR {
        REPORTRDY_READCLRACCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Short-cut between SAMPLERDY event and STOP task."]
    #[inline]
    pub fn samplerdy_stop(&self) -> SAMPLERDY_STOPR {
        SAMPLERDY_STOPR::_from({
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
    #[doc = "Bit 0 - Short-cut between REPORTRDY event and READCLRACC task."]
    #[inline]
    pub fn reportrdy_readclracc(&mut self) -> _REPORTRDY_READCLRACCW {
        _REPORTRDY_READCLRACCW { w: self }
    }
    #[doc = "Bit 1 - Short-cut between SAMPLERDY event and STOP task."]
    #[inline]
    pub fn samplerdy_stop(&mut self) -> _SAMPLERDY_STOPW {
        _SAMPLERDY_STOPW { w: self }
    }
}
