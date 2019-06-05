#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PATCHEN {
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
#[doc = "Possible values of the field `PATCH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PATCH0R {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH0R {
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
            PATCH0R::DISABLED => false,
            PATCH0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PATCH0R {
        match value {
            false => PATCH0R::DISABLED,
            true => PATCH0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PATCH0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PATCH0R::ENABLED
    }
}
#[doc = "Possible values of the field `PATCH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PATCH1R {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH1R {
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
            PATCH1R::DISABLED => false,
            PATCH1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PATCH1R {
        match value {
            false => PATCH1R::DISABLED,
            true => PATCH1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PATCH1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PATCH1R::ENABLED
    }
}
#[doc = "Possible values of the field `PATCH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PATCH2R {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH2R {
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
            PATCH2R::DISABLED => false,
            PATCH2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PATCH2R {
        match value {
            false => PATCH2R::DISABLED,
            true => PATCH2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PATCH2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PATCH2R::ENABLED
    }
}
#[doc = "Possible values of the field `PATCH3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PATCH3R {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH3R {
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
            PATCH3R::DISABLED => false,
            PATCH3R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PATCH3R {
        match value {
            false => PATCH3R::DISABLED,
            true => PATCH3R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PATCH3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PATCH3R::ENABLED
    }
}
#[doc = "Possible values of the field `PATCH4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PATCH4R {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH4R {
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
            PATCH4R::DISABLED => false,
            PATCH4R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PATCH4R {
        match value {
            false => PATCH4R::DISABLED,
            true => PATCH4R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PATCH4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PATCH4R::ENABLED
    }
}
#[doc = "Possible values of the field `PATCH5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PATCH5R {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH5R {
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
            PATCH5R::DISABLED => false,
            PATCH5R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PATCH5R {
        match value {
            false => PATCH5R::DISABLED,
            true => PATCH5R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PATCH5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PATCH5R::ENABLED
    }
}
#[doc = "Possible values of the field `PATCH6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PATCH6R {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH6R {
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
            PATCH6R::DISABLED => false,
            PATCH6R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PATCH6R {
        match value {
            false => PATCH6R::DISABLED,
            true => PATCH6R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PATCH6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PATCH6R::ENABLED
    }
}
#[doc = "Possible values of the field `PATCH7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PATCH7R {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH7R {
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
            PATCH7R::DISABLED => false,
            PATCH7R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PATCH7R {
        match value {
            false => PATCH7R::DISABLED,
            true => PATCH7R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PATCH7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PATCH7R::ENABLED
    }
}
#[doc = "Values that can be written to the field `PATCH0`"]
pub enum PATCH0W {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PATCH0W::DISABLED => false,
            PATCH0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PATCH0W<'a> {
    w: &'a mut W,
}
impl<'a> _PATCH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PATCH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Patch disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PATCH0W::DISABLED)
    }
    #[doc = "Patch enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PATCH0W::ENABLED)
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
#[doc = "Values that can be written to the field `PATCH1`"]
pub enum PATCH1W {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PATCH1W::DISABLED => false,
            PATCH1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PATCH1W<'a> {
    w: &'a mut W,
}
impl<'a> _PATCH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PATCH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Patch disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PATCH1W::DISABLED)
    }
    #[doc = "Patch enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PATCH1W::ENABLED)
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
#[doc = "Values that can be written to the field `PATCH2`"]
pub enum PATCH2W {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PATCH2W::DISABLED => false,
            PATCH2W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PATCH2W<'a> {
    w: &'a mut W,
}
impl<'a> _PATCH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PATCH2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Patch disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PATCH2W::DISABLED)
    }
    #[doc = "Patch enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PATCH2W::ENABLED)
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
#[doc = "Values that can be written to the field `PATCH3`"]
pub enum PATCH3W {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PATCH3W::DISABLED => false,
            PATCH3W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PATCH3W<'a> {
    w: &'a mut W,
}
impl<'a> _PATCH3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PATCH3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Patch disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PATCH3W::DISABLED)
    }
    #[doc = "Patch enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PATCH3W::ENABLED)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PATCH4`"]
pub enum PATCH4W {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PATCH4W::DISABLED => false,
            PATCH4W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PATCH4W<'a> {
    w: &'a mut W,
}
impl<'a> _PATCH4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PATCH4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Patch disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PATCH4W::DISABLED)
    }
    #[doc = "Patch enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PATCH4W::ENABLED)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PATCH5`"]
pub enum PATCH5W {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PATCH5W::DISABLED => false,
            PATCH5W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PATCH5W<'a> {
    w: &'a mut W,
}
impl<'a> _PATCH5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PATCH5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Patch disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PATCH5W::DISABLED)
    }
    #[doc = "Patch enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PATCH5W::ENABLED)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PATCH6`"]
pub enum PATCH6W {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PATCH6W::DISABLED => false,
            PATCH6W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PATCH6W<'a> {
    w: &'a mut W,
}
impl<'a> _PATCH6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PATCH6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Patch disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PATCH6W::DISABLED)
    }
    #[doc = "Patch enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PATCH6W::ENABLED)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PATCH7`"]
pub enum PATCH7W {
    #[doc = "Patch disabled."]
    DISABLED,
    #[doc = "Patch enabled."]
    ENABLED,
}
impl PATCH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PATCH7W::DISABLED => false,
            PATCH7W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PATCH7W<'a> {
    w: &'a mut W,
}
impl<'a> _PATCH7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PATCH7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Patch disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PATCH7W::DISABLED)
    }
    #[doc = "Patch enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PATCH7W::ENABLED)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Patch 0 enabled."]
    #[inline]
    pub fn patch0(&self) -> PATCH0R {
        PATCH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Patch 1 enabled."]
    #[inline]
    pub fn patch1(&self) -> PATCH1R {
        PATCH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Patch 2 enabled."]
    #[inline]
    pub fn patch2(&self) -> PATCH2R {
        PATCH2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Patch 3 enabled."]
    #[inline]
    pub fn patch3(&self) -> PATCH3R {
        PATCH3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Patch 4 enabled."]
    #[inline]
    pub fn patch4(&self) -> PATCH4R {
        PATCH4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Patch 5 enabled."]
    #[inline]
    pub fn patch5(&self) -> PATCH5R {
        PATCH5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Patch 6 enabled."]
    #[inline]
    pub fn patch6(&self) -> PATCH6R {
        PATCH6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Patch 7 enabled."]
    #[inline]
    pub fn patch7(&self) -> PATCH7R {
        PATCH7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Patch 0 enabled."]
    #[inline]
    pub fn patch0(&mut self) -> _PATCH0W {
        _PATCH0W { w: self }
    }
    #[doc = "Bit 1 - Patch 1 enabled."]
    #[inline]
    pub fn patch1(&mut self) -> _PATCH1W {
        _PATCH1W { w: self }
    }
    #[doc = "Bit 2 - Patch 2 enabled."]
    #[inline]
    pub fn patch2(&mut self) -> _PATCH2W {
        _PATCH2W { w: self }
    }
    #[doc = "Bit 3 - Patch 3 enabled."]
    #[inline]
    pub fn patch3(&mut self) -> _PATCH3W {
        _PATCH3W { w: self }
    }
    #[doc = "Bit 4 - Patch 4 enabled."]
    #[inline]
    pub fn patch4(&mut self) -> _PATCH4W {
        _PATCH4W { w: self }
    }
    #[doc = "Bit 5 - Patch 5 enabled."]
    #[inline]
    pub fn patch5(&mut self) -> _PATCH5W {
        _PATCH5W { w: self }
    }
    #[doc = "Bit 6 - Patch 6 enabled."]
    #[inline]
    pub fn patch6(&mut self) -> _PATCH6W {
        _PATCH6W { w: self }
    }
    #[doc = "Bit 7 - Patch 7 enabled."]
    #[inline]
    pub fn patch7(&mut self) -> _PATCH7W {
        _PATCH7W { w: self }
    }
}
