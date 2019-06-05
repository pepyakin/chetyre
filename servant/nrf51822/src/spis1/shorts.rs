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
#[doc = "Possible values of the field `END_ACQUIRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_ACQUIRER {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl END_ACQUIRER {
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
            END_ACQUIRER::DISABLED => false,
            END_ACQUIRER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> END_ACQUIRER {
        match value {
            false => END_ACQUIRER::DISABLED,
            true => END_ACQUIRER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == END_ACQUIRER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == END_ACQUIRER::ENABLED
    }
}
#[doc = "Values that can be written to the field `END_ACQUIRE`"]
pub enum END_ACQUIREW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl END_ACQUIREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            END_ACQUIREW::DISABLED => false,
            END_ACQUIREW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _END_ACQUIREW<'a> {
    w: &'a mut W,
}
impl<'a> _END_ACQUIREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: END_ACQUIREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_ACQUIREW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_ACQUIREW::ENABLED)
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
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 2 - Shortcut between END event and the ACQUIRE task."]
    #[inline]
    pub fn end_acquire(&self) -> END_ACQUIRER {
        END_ACQUIRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 2 - Shortcut between END event and the ACQUIRE task."]
    #[inline]
    pub fn end_acquire(&mut self) -> _END_ACQUIREW {
        _END_ACQUIREW { w: self }
    }
}
