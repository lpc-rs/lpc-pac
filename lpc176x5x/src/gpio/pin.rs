#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIN {
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
pub struct PINVAL0R {
    bits: bool,
}
impl PINVAL0R {
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
pub struct PINVAL1R {
    bits: bool,
}
impl PINVAL1R {
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
pub struct PINVAL2R {
    bits: bool,
}
impl PINVAL2R {
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
pub struct PINVAL3R {
    bits: bool,
}
impl PINVAL3R {
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
pub struct PINVAL4R {
    bits: bool,
}
impl PINVAL4R {
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
pub struct PINVAL5R {
    bits: bool,
}
impl PINVAL5R {
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
pub struct PINVAL6R {
    bits: bool,
}
impl PINVAL6R {
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
pub struct PINVAL7R {
    bits: bool,
}
impl PINVAL7R {
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
pub struct PINVAL8R {
    bits: bool,
}
impl PINVAL8R {
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
pub struct PINVAL9R {
    bits: bool,
}
impl PINVAL9R {
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
pub struct PINVAL10R {
    bits: bool,
}
impl PINVAL10R {
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
pub struct PINVAL11R {
    bits: bool,
}
impl PINVAL11R {
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
pub struct PINVAL12R {
    bits: bool,
}
impl PINVAL12R {
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
pub struct PINVAL13R {
    bits: bool,
}
impl PINVAL13R {
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
pub struct PINVAL14R {
    bits: bool,
}
impl PINVAL14R {
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
pub struct PINVAL15R {
    bits: bool,
}
impl PINVAL15R {
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
pub struct PINVAL16R {
    bits: bool,
}
impl PINVAL16R {
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
pub struct PINVAL17R {
    bits: bool,
}
impl PINVAL17R {
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
pub struct PINVAL18R {
    bits: bool,
}
impl PINVAL18R {
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
pub struct PINVAL19R {
    bits: bool,
}
impl PINVAL19R {
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
pub struct PINVAL20R {
    bits: bool,
}
impl PINVAL20R {
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
pub struct PINVAL21R {
    bits: bool,
}
impl PINVAL21R {
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
pub struct PINVAL22R {
    bits: bool,
}
impl PINVAL22R {
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
pub struct PINVAL23R {
    bits: bool,
}
impl PINVAL23R {
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
pub struct PINVAL24R {
    bits: bool,
}
impl PINVAL24R {
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
pub struct PINVAL25R {
    bits: bool,
}
impl PINVAL25R {
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
pub struct PINVAL26R {
    bits: bool,
}
impl PINVAL26R {
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
pub struct PINVAL27R {
    bits: bool,
}
impl PINVAL27R {
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
pub struct PINVAL28R {
    bits: bool,
}
impl PINVAL28R {
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
pub struct PINVAL29R {
    bits: bool,
}
impl PINVAL29R {
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
pub struct PINVAL30R {
    bits: bool,
}
impl PINVAL30R {
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
pub struct PINVAL31R {
    bits: bool,
}
impl PINVAL31R {
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
pub struct _PINVAL0W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL0W<'a> {
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
pub struct _PINVAL1W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL1W<'a> {
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
pub struct _PINVAL2W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL2W<'a> {
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
pub struct _PINVAL3W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL3W<'a> {
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
pub struct _PINVAL4W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL4W<'a> {
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
pub struct _PINVAL5W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL5W<'a> {
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
pub struct _PINVAL6W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL6W<'a> {
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
pub struct _PINVAL7W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL7W<'a> {
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
pub struct _PINVAL8W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL8W<'a> {
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
pub struct _PINVAL9W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL9W<'a> {
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
pub struct _PINVAL10W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL10W<'a> {
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
pub struct _PINVAL11W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL11W<'a> {
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
pub struct _PINVAL12W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL12W<'a> {
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
pub struct _PINVAL13W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL13W<'a> {
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
pub struct _PINVAL14W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL14W<'a> {
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
pub struct _PINVAL15W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL15W<'a> {
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
pub struct _PINVAL16W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL16W<'a> {
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
pub struct _PINVAL17W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL17W<'a> {
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
pub struct _PINVAL18W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL18W<'a> {
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
pub struct _PINVAL19W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL19W<'a> {
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
pub struct _PINVAL20W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL20W<'a> {
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
pub struct _PINVAL21W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL21W<'a> {
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
pub struct _PINVAL22W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL22W<'a> {
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
pub struct _PINVAL23W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL23W<'a> {
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
pub struct _PINVAL24W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL24W<'a> {
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
pub struct _PINVAL25W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL25W<'a> {
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
pub struct _PINVAL26W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL26W<'a> {
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
pub struct _PINVAL27W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL27W<'a> {
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
pub struct _PINVAL28W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL28W<'a> {
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
pub struct _PINVAL29W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL29W<'a> {
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
pub struct _PINVAL30W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL30W<'a> {
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
pub struct _PINVAL31W<'a> {
    w: &'a mut W,
}
impl<'a> _PINVAL31W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval0(&self) -> PINVAL0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL0R { bits }
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval1(&self) -> PINVAL1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL1R { bits }
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval2(&self) -> PINVAL2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL2R { bits }
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval3(&self) -> PINVAL3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL3R { bits }
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval4(&self) -> PINVAL4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL4R { bits }
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval5(&self) -> PINVAL5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL5R { bits }
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval6(&self) -> PINVAL6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL6R { bits }
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval7(&self) -> PINVAL7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL7R { bits }
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval8(&self) -> PINVAL8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL8R { bits }
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval9(&self) -> PINVAL9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL9R { bits }
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval10(&self) -> PINVAL10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL10R { bits }
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval11(&self) -> PINVAL11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL11R { bits }
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval12(&self) -> PINVAL12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL12R { bits }
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval13(&self) -> PINVAL13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL13R { bits }
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval14(&self) -> PINVAL14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL14R { bits }
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval15(&self) -> PINVAL15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL15R { bits }
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval16(&self) -> PINVAL16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL16R { bits }
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval17(&self) -> PINVAL17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL17R { bits }
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval18(&self) -> PINVAL18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL18R { bits }
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval19(&self) -> PINVAL19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL19R { bits }
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval20(&self) -> PINVAL20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL20R { bits }
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval21(&self) -> PINVAL21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL21R { bits }
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval22(&self) -> PINVAL22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL22R { bits }
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval23(&self) -> PINVAL23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL23R { bits }
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval24(&self) -> PINVAL24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL24R { bits }
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval25(&self) -> PINVAL25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL25R { bits }
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval26(&self) -> PINVAL26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL26R { bits }
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval27(&self) -> PINVAL27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL27R { bits }
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval28(&self) -> PINVAL28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL28R { bits }
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval29(&self) -> PINVAL29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL29R { bits }
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval30(&self) -> PINVAL30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL30R { bits }
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval31(&self) -> PINVAL31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINVAL31R { bits }
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
    #[doc = "Bit 0 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval0(&mut self) -> _PINVAL0W {
        _PINVAL0W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval1(&mut self) -> _PINVAL1W {
        _PINVAL1W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval2(&mut self) -> _PINVAL2W {
        _PINVAL2W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval3(&mut self) -> _PINVAL3W {
        _PINVAL3W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval4(&mut self) -> _PINVAL4W {
        _PINVAL4W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval5(&mut self) -> _PINVAL5W {
        _PINVAL5W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval6(&mut self) -> _PINVAL6W {
        _PINVAL6W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval7(&mut self) -> _PINVAL7W {
        _PINVAL7W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval8(&mut self) -> _PINVAL8W {
        _PINVAL8W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval9(&mut self) -> _PINVAL9W {
        _PINVAL9W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval10(&mut self) -> _PINVAL10W {
        _PINVAL10W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval11(&mut self) -> _PINVAL11W {
        _PINVAL11W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval12(&mut self) -> _PINVAL12W {
        _PINVAL12W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval13(&mut self) -> _PINVAL13W {
        _PINVAL13W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval14(&mut self) -> _PINVAL14W {
        _PINVAL14W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval15(&mut self) -> _PINVAL15W {
        _PINVAL15W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval16(&mut self) -> _PINVAL16W {
        _PINVAL16W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval17(&mut self) -> _PINVAL17W {
        _PINVAL17W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval18(&mut self) -> _PINVAL18W {
        _PINVAL18W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval19(&mut self) -> _PINVAL19W {
        _PINVAL19W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval20(&mut self) -> _PINVAL20W {
        _PINVAL20W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval21(&mut self) -> _PINVAL21W {
        _PINVAL21W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval22(&mut self) -> _PINVAL22W {
        _PINVAL22W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval23(&mut self) -> _PINVAL23W {
        _PINVAL23W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval24(&mut self) -> _PINVAL24W {
        _PINVAL24W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval25(&mut self) -> _PINVAL25W {
        _PINVAL25W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval26(&mut self) -> _PINVAL26W {
        _PINVAL26W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval27(&mut self) -> _PINVAL27W {
        _PINVAL27W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval28(&mut self) -> _PINVAL28W {
        _PINVAL28W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval29(&mut self) -> _PINVAL29W {
        _PINVAL29W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval30(&mut self) -> _PINVAL30W {
        _PINVAL30W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO output value Set bits. Bit 0 in PINx corresponds to pin Px\\[0\\], bit 31 in PINx corresponds to pin Px\\[31\\]. 0 = Controlled pin output is set to LOW. 1 = Controlled pin output is set to HIGH."]
    #[inline]
    pub fn pinval31(&mut self) -> _PINVAL31W {
        _PINVAL31W { w: self }
    }
}
