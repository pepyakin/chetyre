#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RBPCONF {
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
#[doc = "Possible values of the field `PR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR0R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PR0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PR0R::DISABLED => 255,
            PR0R::ENABLED => 0,
            PR0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PR0R {
        match value {
            255 => PR0R::DISABLED,
            0 => PR0R::ENABLED,
            i => PR0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PR0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PR0R::ENABLED
    }
}
#[doc = "Possible values of the field `PALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PALLR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PALLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PALLR::DISABLED => 255,
            PALLR::ENABLED => 0,
            PALLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PALLR {
        match value {
            255 => PALLR::DISABLED,
            0 => PALLR::ENABLED,
            i => PALLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PALLR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PALLR::ENABLED
    }
}
#[doc = "Values that can be written to the field `PR0`"]
pub enum PR0W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl PR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PR0W::DISABLED => 255,
            PR0W::ENABLED => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PR0W<'a> {
    w: &'a mut W,
}
impl<'a> _PR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PR0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PR0W::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PR0W::ENABLED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PALL`"]
pub enum PALLW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl PALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PALLW::DISABLED => 255,
            PALLW::ENABLED => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PALLW<'a> {
    w: &'a mut W,
}
impl<'a> _PALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PALLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PALLW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PALLW::ENABLED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
    #[inline]
    pub fn pr0(&self) -> PR0R {
        PR0R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Readback protect all code in the device."]
    #[inline]
    pub fn pall(&self) -> PALLR {
        PALLR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:7 - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
    #[inline]
    pub fn pr0(&mut self) -> _PR0W {
        _PR0W { w: self }
    }
    #[doc = "Bits 8:15 - Readback protect all code in the device."]
    #[inline]
    pub fn pall(&mut self) -> _PALLW {
        _PALLW { w: self }
    }
}
