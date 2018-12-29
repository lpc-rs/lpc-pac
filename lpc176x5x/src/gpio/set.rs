#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SET {
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
#[doc = r" Value of the field"]
pub struct PINSET0R {
    bits: bool,
}
impl PINSET0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET1R {
    bits: bool,
}
impl PINSET1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET2R {
    bits: bool,
}
impl PINSET2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET3R {
    bits: bool,
}
impl PINSET3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET4R {
    bits: bool,
}
impl PINSET4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET5R {
    bits: bool,
}
impl PINSET5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET6R {
    bits: bool,
}
impl PINSET6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET7R {
    bits: bool,
}
impl PINSET7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET8R {
    bits: bool,
}
impl PINSET8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET9R {
    bits: bool,
}
impl PINSET9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET10R {
    bits: bool,
}
impl PINSET10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET11R {
    bits: bool,
}
impl PINSET11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET12R {
    bits: bool,
}
impl PINSET12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET13R {
    bits: bool,
}
impl PINSET13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET14R {
    bits: bool,
}
impl PINSET14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET15R {
    bits: bool,
}
impl PINSET15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET16R {
    bits: bool,
}
impl PINSET16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET17R {
    bits: bool,
}
impl PINSET17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET18R {
    bits: bool,
}
impl PINSET18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET19R {
    bits: bool,
}
impl PINSET19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET20R {
    bits: bool,
}
impl PINSET20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET21R {
    bits: bool,
}
impl PINSET21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET22R {
    bits: bool,
}
impl PINSET22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET23R {
    bits: bool,
}
impl PINSET23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET24R {
    bits: bool,
}
impl PINSET24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET25R {
    bits: bool,
}
impl PINSET25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET26R {
    bits: bool,
}
impl PINSET26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET27R {
    bits: bool,
}
impl PINSET27R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET28R {
    bits: bool,
}
impl PINSET28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET29R {
    bits: bool,
}
impl PINSET29R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET30R {
    bits: bool,
}
impl PINSET30R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PINSET31R {
    bits: bool,
}
impl PINSET31R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _PINSET0W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET0W<'a> {
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
#[doc = r" Proxy"]
pub struct _PINSET1W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET1W<'a> {
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
#[doc = r" Proxy"]
pub struct _PINSET2W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET2W<'a> {
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
#[doc = r" Proxy"]
pub struct _PINSET3W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET3W<'a> {
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
#[doc = r" Proxy"]
pub struct _PINSET4W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET4W<'a> {
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
#[doc = r" Proxy"]
pub struct _PINSET5W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET5W<'a> {
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
#[doc = r" Proxy"]
pub struct _PINSET6W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET6W<'a> {
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
#[doc = r" Proxy"]
pub struct _PINSET7W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET7W<'a> {
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
#[doc = r" Proxy"]
pub struct _PINSET8W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET8W<'a> {
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
#[doc = r" Proxy"]
pub struct _PINSET9W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET9W<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET10W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET10W<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET11W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET11W<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET12W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET12W<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET13W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET13W<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET14W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET14W<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET15W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET15W<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET16W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET16W<'a> {
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
#[doc = r" Proxy"]
pub struct _PINSET17W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET17W<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET18W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET18W<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET19W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET19W<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET20W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET20W<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET21W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET21W<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET22W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET22W<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET23W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET23W<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET24W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET24W<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET25W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET25W<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET26W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET26W<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET27W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET27W<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET28W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET28W<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET29W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET29W<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET30W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET30W<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PINSET31W<'a> {
    w: &'a mut W,
}
impl<'a> _PINSET31W<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset0(&self) -> PINSET0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET0R { bits }
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset1(&self) -> PINSET1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET1R { bits }
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset2(&self) -> PINSET2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET2R { bits }
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset3(&self) -> PINSET3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET3R { bits }
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset4(&self) -> PINSET4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET4R { bits }
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset5(&self) -> PINSET5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET5R { bits }
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset6(&self) -> PINSET6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET6R { bits }
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset7(&self) -> PINSET7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET7R { bits }
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset8(&self) -> PINSET8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET8R { bits }
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset9(&self) -> PINSET9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET9R { bits }
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset10(&self) -> PINSET10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET10R { bits }
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset11(&self) -> PINSET11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET11R { bits }
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset12(&self) -> PINSET12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET12R { bits }
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset13(&self) -> PINSET13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET13R { bits }
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset14(&self) -> PINSET14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET14R { bits }
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset15(&self) -> PINSET15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET15R { bits }
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset16(&self) -> PINSET16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET16R { bits }
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset17(&self) -> PINSET17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET17R { bits }
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset18(&self) -> PINSET18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET18R { bits }
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset19(&self) -> PINSET19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET19R { bits }
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset20(&self) -> PINSET20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET20R { bits }
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset21(&self) -> PINSET21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET21R { bits }
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset22(&self) -> PINSET22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET22R { bits }
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset23(&self) -> PINSET23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET23R { bits }
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset24(&self) -> PINSET24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET24R { bits }
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset25(&self) -> PINSET25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET25R { bits }
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset26(&self) -> PINSET26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET26R { bits }
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset27(&self) -> PINSET27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET27R { bits }
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset28(&self) -> PINSET28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET28R { bits }
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset29(&self) -> PINSET29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET29R { bits }
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset30(&self) -> PINSET30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET30R { bits }
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset31(&self) -> PINSET31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINSET31R { bits }
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
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset0(&mut self) -> _PINSET0W {
        _PINSET0W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset1(&mut self) -> _PINSET1W {
        _PINSET1W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset2(&mut self) -> _PINSET2W {
        _PINSET2W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset3(&mut self) -> _PINSET3W {
        _PINSET3W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset4(&mut self) -> _PINSET4W {
        _PINSET4W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset5(&mut self) -> _PINSET5W {
        _PINSET5W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset6(&mut self) -> _PINSET6W {
        _PINSET6W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset7(&mut self) -> _PINSET7W {
        _PINSET7W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset8(&mut self) -> _PINSET8W {
        _PINSET8W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset9(&mut self) -> _PINSET9W {
        _PINSET9W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset10(&mut self) -> _PINSET10W {
        _PINSET10W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset11(&mut self) -> _PINSET11W {
        _PINSET11W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset12(&mut self) -> _PINSET12W {
        _PINSET12W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset13(&mut self) -> _PINSET13W {
        _PINSET13W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset14(&mut self) -> _PINSET14W {
        _PINSET14W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset15(&mut self) -> _PINSET15W {
        _PINSET15W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset16(&mut self) -> _PINSET16W {
        _PINSET16W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset17(&mut self) -> _PINSET17W {
        _PINSET17W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset18(&mut self) -> _PINSET18W {
        _PINSET18W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset19(&mut self) -> _PINSET19W {
        _PINSET19W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset20(&mut self) -> _PINSET20W {
        _PINSET20W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset21(&mut self) -> _PINSET21W {
        _PINSET21W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset22(&mut self) -> _PINSET22W {
        _PINSET22W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset23(&mut self) -> _PINSET23W {
        _PINSET23W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset24(&mut self) -> _PINSET24W {
        _PINSET24W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset25(&mut self) -> _PINSET25W {
        _PINSET25W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset26(&mut self) -> _PINSET26W {
        _PINSET26W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset27(&mut self) -> _PINSET27W {
        _PINSET27W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset28(&mut self) -> _PINSET28W {
        _PINSET28W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset29(&mut self) -> _PINSET29W {
        _PINSET29W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset30(&mut self) -> _PINSET30W {
        _PINSET30W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in SETx controls pin Px\\[0\\], bit 31 in SETx controls pin Px\\[31\\]. 0 = Controlled pin output is unchanged. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinset31(&mut self) -> _PINSET31W {
        _PINSET31W { w: self }
    }
}
