#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFCLKSTAT {
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
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
    #[doc = "Internal 32KiHz RC oscillator running and generating the LFCLK clock."]
    RC,
    #[doc = "External 32KiHz crystal oscillator running and generating the LFCLK clock."]
    XTAL,
    #[doc = "Internal 32KiHz synthesizer from the HFCLK running and generating the LFCLK clock."]
    SYNTH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCR::RC => 0,
            SRCR::XTAL => 1,
            SRCR::SYNTH => 2,
            SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCR {
        match value {
            0 => SRCR::RC,
            1 => SRCR::XTAL,
            2 => SRCR::SYNTH,
            i => SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline]
    pub fn is_rc(&self) -> bool {
        *self == SRCR::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline]
    pub fn is_xtal(&self) -> bool {
        *self == SRCR::XTAL
    }
    #[doc = "Checks if the value of the field is `SYNTH`"]
    #[inline]
    pub fn is_synth(&self) -> bool {
        *self == SRCR::SYNTH
    }
}
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "LFCLK clock not running."]
    NOTRUNNING,
    #[doc = "LFCLK clock running."]
    RUNNING,
}
impl STATER {
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
            STATER::NOTRUNNING => false,
            STATER::RUNNING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STATER {
        match value {
            false => STATER::NOTRUNNING,
            true => STATER::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline]
    pub fn is_not_running(&self) -> bool {
        *self == STATER::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline]
    pub fn is_running(&self) -> bool {
        *self == STATER::RUNNING
    }
}
#[doc = "Values that can be written to the field `SRC`"]
pub enum SRCW {
    #[doc = "Internal 32KiHz RC oscillator running and generating the LFCLK clock."]
    RC,
    #[doc = "External 32KiHz crystal oscillator running and generating the LFCLK clock."]
    XTAL,
    #[doc = "Internal 32KiHz synthesizer from the HFCLK running and generating the LFCLK clock."]
    SYNTH,
}
impl SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCW::RC => 0,
            SRCW::XTAL => 1,
            SRCW::SYNTH => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal 32KiHz RC oscillator running and generating the LFCLK clock."]
    #[inline]
    pub fn rc(self) -> &'a mut W {
        self.variant(SRCW::RC)
    }
    #[doc = "External 32KiHz crystal oscillator running and generating the LFCLK clock."]
    #[inline]
    pub fn xtal(self) -> &'a mut W {
        self.variant(SRCW::XTAL)
    }
    #[doc = "Internal 32KiHz synthesizer from the HFCLK running and generating the LFCLK clock."]
    #[inline]
    pub fn synth(self) -> &'a mut W {
        self.variant(SRCW::SYNTH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STATE`"]
pub enum STATEW {
    #[doc = "LFCLK clock not running."]
    NOTRUNNING,
    #[doc = "LFCLK clock running."]
    RUNNING,
}
impl STATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STATEW::NOTRUNNING => false,
            STATEW::RUNNING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATEW<'a> {
    w: &'a mut W,
}
impl<'a> _STATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LFCLK clock not running."]
    #[inline]
    pub fn not_running(self) -> &'a mut W {
        self.variant(STATEW::NOTRUNNING)
    }
    #[doc = "LFCLK clock running."]
    #[inline]
    pub fn running(self) -> &'a mut W {
        self.variant(STATEW::RUNNING)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - Active clock source for the LF clock."]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - State for the LF clock."]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - Active clock source for the LF clock."]
    #[inline]
    pub fn src(&mut self) -> _SRCW {
        _SRCW { w: self }
    }
    #[doc = "Bit 16 - State for the LF clock."]
    #[inline]
    pub fn state(&mut self) -> _STATEW {
        _STATEW { w: self }
    }
}
