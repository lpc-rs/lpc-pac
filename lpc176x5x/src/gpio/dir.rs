#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DIR {
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
pub struct PINDIR0R {
    bits: bool,
}
impl PINDIR0R {
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
pub struct PINDIR1R {
    bits: bool,
}
impl PINDIR1R {
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
pub struct PINDIR2R {
    bits: bool,
}
impl PINDIR2R {
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
pub struct PINDIR3R {
    bits: bool,
}
impl PINDIR3R {
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
pub struct PINDIR4R {
    bits: bool,
}
impl PINDIR4R {
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
pub struct PINDIR5R {
    bits: bool,
}
impl PINDIR5R {
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
pub struct PINDIR6R {
    bits: bool,
}
impl PINDIR6R {
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
pub struct PINDIR7R {
    bits: bool,
}
impl PINDIR7R {
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
pub struct PINDIR8R {
    bits: bool,
}
impl PINDIR8R {
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
pub struct PINDIR9R {
    bits: bool,
}
impl PINDIR9R {
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
pub struct PINDIR10R {
    bits: bool,
}
impl PINDIR10R {
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
pub struct PINDIR11R {
    bits: bool,
}
impl PINDIR11R {
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
pub struct PINDIR12R {
    bits: bool,
}
impl PINDIR12R {
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
pub struct PINDIR13R {
    bits: bool,
}
impl PINDIR13R {
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
pub struct PINDIR14R {
    bits: bool,
}
impl PINDIR14R {
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
pub struct PINDIR15R {
    bits: bool,
}
impl PINDIR15R {
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
pub struct PINDIR16R {
    bits: bool,
}
impl PINDIR16R {
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
pub struct PINDIR17R {
    bits: bool,
}
impl PINDIR17R {
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
pub struct PINDIR18R {
    bits: bool,
}
impl PINDIR18R {
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
pub struct PINDIR19R {
    bits: bool,
}
impl PINDIR19R {
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
pub struct PINDIR20R {
    bits: bool,
}
impl PINDIR20R {
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
pub struct PINDIR21R {
    bits: bool,
}
impl PINDIR21R {
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
pub struct PINDIR22R {
    bits: bool,
}
impl PINDIR22R {
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
pub struct PINDIR23R {
    bits: bool,
}
impl PINDIR23R {
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
pub struct PINDIR24R {
    bits: bool,
}
impl PINDIR24R {
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
pub struct PINDIR25R {
    bits: bool,
}
impl PINDIR25R {
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
pub struct PINDIR26R {
    bits: bool,
}
impl PINDIR26R {
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
pub struct PINDIR27R {
    bits: bool,
}
impl PINDIR27R {
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
pub struct PINDIR28R {
    bits: bool,
}
impl PINDIR28R {
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
pub struct PINDIR29R {
    bits: bool,
}
impl PINDIR29R {
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
pub struct PINDIR30R {
    bits: bool,
}
impl PINDIR30R {
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
pub struct PINDIR31R {
    bits: bool,
}
impl PINDIR31R {
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
pub struct _PINDIR0W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR0W<'a> {
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
pub struct _PINDIR1W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR1W<'a> {
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
pub struct _PINDIR2W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR2W<'a> {
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
pub struct _PINDIR3W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR3W<'a> {
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
pub struct _PINDIR4W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR4W<'a> {
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
pub struct _PINDIR5W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR5W<'a> {
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
pub struct _PINDIR6W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR6W<'a> {
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
pub struct _PINDIR7W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR7W<'a> {
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
pub struct _PINDIR8W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR8W<'a> {
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
pub struct _PINDIR9W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR9W<'a> {
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
pub struct _PINDIR10W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR10W<'a> {
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
pub struct _PINDIR11W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR11W<'a> {
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
pub struct _PINDIR12W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR12W<'a> {
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
pub struct _PINDIR13W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR13W<'a> {
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
pub struct _PINDIR14W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR14W<'a> {
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
pub struct _PINDIR15W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR15W<'a> {
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
pub struct _PINDIR16W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR16W<'a> {
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
pub struct _PINDIR17W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR17W<'a> {
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
pub struct _PINDIR18W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR18W<'a> {
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
pub struct _PINDIR19W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR19W<'a> {
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
pub struct _PINDIR20W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR20W<'a> {
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
pub struct _PINDIR21W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR21W<'a> {
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
pub struct _PINDIR22W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR22W<'a> {
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
pub struct _PINDIR23W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR23W<'a> {
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
pub struct _PINDIR24W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR24W<'a> {
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
pub struct _PINDIR25W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR25W<'a> {
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
pub struct _PINDIR26W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR26W<'a> {
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
pub struct _PINDIR27W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR27W<'a> {
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
pub struct _PINDIR28W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR28W<'a> {
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
pub struct _PINDIR29W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR29W<'a> {
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
pub struct _PINDIR30W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR30W<'a> {
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
pub struct _PINDIR31W<'a> {
    w: &'a mut W,
}
impl<'a> _PINDIR31W<'a> {
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
    #[doc = "Bit 0 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir0(&self) -> PINDIR0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR0R { bits }
    }
    #[doc = "Bit 1 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir1(&self) -> PINDIR1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR1R { bits }
    }
    #[doc = "Bit 2 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir2(&self) -> PINDIR2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR2R { bits }
    }
    #[doc = "Bit 3 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir3(&self) -> PINDIR3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR3R { bits }
    }
    #[doc = "Bit 4 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir4(&self) -> PINDIR4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR4R { bits }
    }
    #[doc = "Bit 5 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir5(&self) -> PINDIR5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR5R { bits }
    }
    #[doc = "Bit 6 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir6(&self) -> PINDIR6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR6R { bits }
    }
    #[doc = "Bit 7 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir7(&self) -> PINDIR7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR7R { bits }
    }
    #[doc = "Bit 8 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir8(&self) -> PINDIR8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR8R { bits }
    }
    #[doc = "Bit 9 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir9(&self) -> PINDIR9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR9R { bits }
    }
    #[doc = "Bit 10 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir10(&self) -> PINDIR10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR10R { bits }
    }
    #[doc = "Bit 11 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir11(&self) -> PINDIR11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR11R { bits }
    }
    #[doc = "Bit 12 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir12(&self) -> PINDIR12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR12R { bits }
    }
    #[doc = "Bit 13 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir13(&self) -> PINDIR13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR13R { bits }
    }
    #[doc = "Bit 14 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir14(&self) -> PINDIR14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR14R { bits }
    }
    #[doc = "Bit 15 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir15(&self) -> PINDIR15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR15R { bits }
    }
    #[doc = "Bit 16 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir16(&self) -> PINDIR16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR16R { bits }
    }
    #[doc = "Bit 17 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir17(&self) -> PINDIR17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR17R { bits }
    }
    #[doc = "Bit 18 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir18(&self) -> PINDIR18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR18R { bits }
    }
    #[doc = "Bit 19 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir19(&self) -> PINDIR19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR19R { bits }
    }
    #[doc = "Bit 20 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir20(&self) -> PINDIR20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR20R { bits }
    }
    #[doc = "Bit 21 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir21(&self) -> PINDIR21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR21R { bits }
    }
    #[doc = "Bit 22 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir22(&self) -> PINDIR22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR22R { bits }
    }
    #[doc = "Bit 23 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir23(&self) -> PINDIR23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR23R { bits }
    }
    #[doc = "Bit 24 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir24(&self) -> PINDIR24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR24R { bits }
    }
    #[doc = "Bit 25 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir25(&self) -> PINDIR25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR25R { bits }
    }
    #[doc = "Bit 26 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir26(&self) -> PINDIR26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR26R { bits }
    }
    #[doc = "Bit 27 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir27(&self) -> PINDIR27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR27R { bits }
    }
    #[doc = "Bit 28 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir28(&self) -> PINDIR28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR28R { bits }
    }
    #[doc = "Bit 29 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir29(&self) -> PINDIR29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR29R { bits }
    }
    #[doc = "Bit 30 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir30(&self) -> PINDIR30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR30R { bits }
    }
    #[doc = "Bit 31 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir31(&self) -> PINDIR31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINDIR31R { bits }
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
    #[doc = "Bit 0 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir0(&mut self) -> _PINDIR0W {
        _PINDIR0W { w: self }
    }
    #[doc = "Bit 1 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir1(&mut self) -> _PINDIR1W {
        _PINDIR1W { w: self }
    }
    #[doc = "Bit 2 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir2(&mut self) -> _PINDIR2W {
        _PINDIR2W { w: self }
    }
    #[doc = "Bit 3 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir3(&mut self) -> _PINDIR3W {
        _PINDIR3W { w: self }
    }
    #[doc = "Bit 4 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir4(&mut self) -> _PINDIR4W {
        _PINDIR4W { w: self }
    }
    #[doc = "Bit 5 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir5(&mut self) -> _PINDIR5W {
        _PINDIR5W { w: self }
    }
    #[doc = "Bit 6 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir6(&mut self) -> _PINDIR6W {
        _PINDIR6W { w: self }
    }
    #[doc = "Bit 7 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir7(&mut self) -> _PINDIR7W {
        _PINDIR7W { w: self }
    }
    #[doc = "Bit 8 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir8(&mut self) -> _PINDIR8W {
        _PINDIR8W { w: self }
    }
    #[doc = "Bit 9 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir9(&mut self) -> _PINDIR9W {
        _PINDIR9W { w: self }
    }
    #[doc = "Bit 10 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir10(&mut self) -> _PINDIR10W {
        _PINDIR10W { w: self }
    }
    #[doc = "Bit 11 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir11(&mut self) -> _PINDIR11W {
        _PINDIR11W { w: self }
    }
    #[doc = "Bit 12 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir12(&mut self) -> _PINDIR12W {
        _PINDIR12W { w: self }
    }
    #[doc = "Bit 13 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir13(&mut self) -> _PINDIR13W {
        _PINDIR13W { w: self }
    }
    #[doc = "Bit 14 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir14(&mut self) -> _PINDIR14W {
        _PINDIR14W { w: self }
    }
    #[doc = "Bit 15 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir15(&mut self) -> _PINDIR15W {
        _PINDIR15W { w: self }
    }
    #[doc = "Bit 16 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir16(&mut self) -> _PINDIR16W {
        _PINDIR16W { w: self }
    }
    #[doc = "Bit 17 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir17(&mut self) -> _PINDIR17W {
        _PINDIR17W { w: self }
    }
    #[doc = "Bit 18 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir18(&mut self) -> _PINDIR18W {
        _PINDIR18W { w: self }
    }
    #[doc = "Bit 19 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir19(&mut self) -> _PINDIR19W {
        _PINDIR19W { w: self }
    }
    #[doc = "Bit 20 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir20(&mut self) -> _PINDIR20W {
        _PINDIR20W { w: self }
    }
    #[doc = "Bit 21 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir21(&mut self) -> _PINDIR21W {
        _PINDIR21W { w: self }
    }
    #[doc = "Bit 22 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir22(&mut self) -> _PINDIR22W {
        _PINDIR22W { w: self }
    }
    #[doc = "Bit 23 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir23(&mut self) -> _PINDIR23W {
        _PINDIR23W { w: self }
    }
    #[doc = "Bit 24 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir24(&mut self) -> _PINDIR24W {
        _PINDIR24W { w: self }
    }
    #[doc = "Bit 25 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir25(&mut self) -> _PINDIR25W {
        _PINDIR25W { w: self }
    }
    #[doc = "Bit 26 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir26(&mut self) -> _PINDIR26W {
        _PINDIR26W { w: self }
    }
    #[doc = "Bit 27 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir27(&mut self) -> _PINDIR27W {
        _PINDIR27W { w: self }
    }
    #[doc = "Bit 28 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir28(&mut self) -> _PINDIR28W {
        _PINDIR28W { w: self }
    }
    #[doc = "Bit 29 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir29(&mut self) -> _PINDIR29W {
        _PINDIR29W { w: self }
    }
    #[doc = "Bit 30 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir30(&mut self) -> _PINDIR30W {
        _PINDIR30W { w: self }
    }
    #[doc = "Bit 31 - Fast GPIO Direction PORTx control bits. Bit 0 in DIRx controls pin Px\\[0\\], bit 31 in DIRx controls pin Px\\[31\\]. 0 = Controlled pin is input. 1 = Controlled pin is output."]
    #[inline]
    pub fn pindir31(&mut self) -> _PINDIR31W {
        _PINDIR31W { w: self }
    }
}
