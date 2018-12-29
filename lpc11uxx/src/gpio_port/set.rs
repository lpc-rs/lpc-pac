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
pub struct SETP0R {
    bits: bool,
}
impl SETP0R {
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
pub struct SETP1R {
    bits: bool,
}
impl SETP1R {
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
pub struct SETP2R {
    bits: bool,
}
impl SETP2R {
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
pub struct SETP3R {
    bits: bool,
}
impl SETP3R {
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
pub struct SETP4R {
    bits: bool,
}
impl SETP4R {
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
pub struct SETP5R {
    bits: bool,
}
impl SETP5R {
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
pub struct SETP6R {
    bits: bool,
}
impl SETP6R {
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
pub struct SETP7R {
    bits: bool,
}
impl SETP7R {
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
pub struct SETP8R {
    bits: bool,
}
impl SETP8R {
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
pub struct SETP9R {
    bits: bool,
}
impl SETP9R {
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
pub struct SETP10R {
    bits: bool,
}
impl SETP10R {
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
pub struct SETP11R {
    bits: bool,
}
impl SETP11R {
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
pub struct SETP12R {
    bits: bool,
}
impl SETP12R {
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
pub struct SETP13R {
    bits: bool,
}
impl SETP13R {
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
pub struct SETP14R {
    bits: bool,
}
impl SETP14R {
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
pub struct SETP15R {
    bits: bool,
}
impl SETP15R {
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
pub struct SETP16R {
    bits: bool,
}
impl SETP16R {
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
pub struct SETP17R {
    bits: bool,
}
impl SETP17R {
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
pub struct SETP18R {
    bits: bool,
}
impl SETP18R {
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
pub struct SETP19R {
    bits: bool,
}
impl SETP19R {
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
pub struct SETP20R {
    bits: bool,
}
impl SETP20R {
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
pub struct SETP21R {
    bits: bool,
}
impl SETP21R {
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
pub struct SETP22R {
    bits: bool,
}
impl SETP22R {
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
pub struct SETP23R {
    bits: bool,
}
impl SETP23R {
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
pub struct SETP24R {
    bits: bool,
}
impl SETP24R {
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
pub struct SETP25R {
    bits: bool,
}
impl SETP25R {
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
pub struct SETP26R {
    bits: bool,
}
impl SETP26R {
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
pub struct SETP27R {
    bits: bool,
}
impl SETP27R {
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
pub struct SETP28R {
    bits: bool,
}
impl SETP28R {
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
pub struct SETP29R {
    bits: bool,
}
impl SETP29R {
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
pub struct SETP30R {
    bits: bool,
}
impl SETP30R {
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
pub struct SETP31R {
    bits: bool,
}
impl SETP31R {
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
pub struct _SETP0W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP0W<'a> {
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
pub struct _SETP1W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP1W<'a> {
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
pub struct _SETP2W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP2W<'a> {
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
pub struct _SETP3W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP3W<'a> {
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
pub struct _SETP4W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP4W<'a> {
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
pub struct _SETP5W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP5W<'a> {
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
pub struct _SETP6W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP6W<'a> {
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
pub struct _SETP7W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP7W<'a> {
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
pub struct _SETP8W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP8W<'a> {
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
pub struct _SETP9W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP9W<'a> {
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
pub struct _SETP10W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP10W<'a> {
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
pub struct _SETP11W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP11W<'a> {
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
pub struct _SETP12W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP12W<'a> {
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
pub struct _SETP13W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP13W<'a> {
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
pub struct _SETP14W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP14W<'a> {
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
pub struct _SETP15W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP15W<'a> {
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
pub struct _SETP16W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP16W<'a> {
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
pub struct _SETP17W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP17W<'a> {
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
pub struct _SETP18W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP18W<'a> {
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
pub struct _SETP19W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP19W<'a> {
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
pub struct _SETP20W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP20W<'a> {
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
pub struct _SETP21W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP21W<'a> {
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
pub struct _SETP22W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP22W<'a> {
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
pub struct _SETP23W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP23W<'a> {
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
pub struct _SETP24W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP24W<'a> {
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
pub struct _SETP25W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP25W<'a> {
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
pub struct _SETP26W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP26W<'a> {
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
pub struct _SETP27W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP27W<'a> {
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
pub struct _SETP28W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP28W<'a> {
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
pub struct _SETP29W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP29W<'a> {
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
pub struct _SETP30W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP30W<'a> {
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
pub struct _SETP31W<'a> {
    w: &'a mut W,
}
impl<'a> _SETP31W<'a> {
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
    #[doc = "Bit 0 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp0(&self) -> SETP0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP0R { bits }
    }
    #[doc = "Bit 1 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp1(&self) -> SETP1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP1R { bits }
    }
    #[doc = "Bit 2 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp2(&self) -> SETP2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP2R { bits }
    }
    #[doc = "Bit 3 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp3(&self) -> SETP3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP3R { bits }
    }
    #[doc = "Bit 4 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp4(&self) -> SETP4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP4R { bits }
    }
    #[doc = "Bit 5 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp5(&self) -> SETP5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP5R { bits }
    }
    #[doc = "Bit 6 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp6(&self) -> SETP6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP6R { bits }
    }
    #[doc = "Bit 7 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp7(&self) -> SETP7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP7R { bits }
    }
    #[doc = "Bit 8 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp8(&self) -> SETP8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP8R { bits }
    }
    #[doc = "Bit 9 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp9(&self) -> SETP9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP9R { bits }
    }
    #[doc = "Bit 10 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp10(&self) -> SETP10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP10R { bits }
    }
    #[doc = "Bit 11 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp11(&self) -> SETP11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP11R { bits }
    }
    #[doc = "Bit 12 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp12(&self) -> SETP12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP12R { bits }
    }
    #[doc = "Bit 13 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp13(&self) -> SETP13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP13R { bits }
    }
    #[doc = "Bit 14 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp14(&self) -> SETP14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP14R { bits }
    }
    #[doc = "Bit 15 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp15(&self) -> SETP15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP15R { bits }
    }
    #[doc = "Bit 16 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp16(&self) -> SETP16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP16R { bits }
    }
    #[doc = "Bit 17 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp17(&self) -> SETP17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP17R { bits }
    }
    #[doc = "Bit 18 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp18(&self) -> SETP18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP18R { bits }
    }
    #[doc = "Bit 19 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp19(&self) -> SETP19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP19R { bits }
    }
    #[doc = "Bit 20 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp20(&self) -> SETP20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP20R { bits }
    }
    #[doc = "Bit 21 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp21(&self) -> SETP21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP21R { bits }
    }
    #[doc = "Bit 22 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp22(&self) -> SETP22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP22R { bits }
    }
    #[doc = "Bit 23 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp23(&self) -> SETP23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP23R { bits }
    }
    #[doc = "Bit 24 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp24(&self) -> SETP24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP24R { bits }
    }
    #[doc = "Bit 25 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp25(&self) -> SETP25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP25R { bits }
    }
    #[doc = "Bit 26 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp26(&self) -> SETP26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP26R { bits }
    }
    #[doc = "Bit 27 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp27(&self) -> SETP27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP27R { bits }
    }
    #[doc = "Bit 28 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp28(&self) -> SETP28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP28R { bits }
    }
    #[doc = "Bit 29 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp29(&self) -> SETP29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP29R { bits }
    }
    #[doc = "Bit 30 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp30(&self) -> SETP30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP30R { bits }
    }
    #[doc = "Bit 31 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp31(&self) -> SETP31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETP31R { bits }
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
    #[doc = "Bit 0 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp0(&mut self) -> _SETP0W {
        _SETP0W { w: self }
    }
    #[doc = "Bit 1 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp1(&mut self) -> _SETP1W {
        _SETP1W { w: self }
    }
    #[doc = "Bit 2 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp2(&mut self) -> _SETP2W {
        _SETP2W { w: self }
    }
    #[doc = "Bit 3 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp3(&mut self) -> _SETP3W {
        _SETP3W { w: self }
    }
    #[doc = "Bit 4 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp4(&mut self) -> _SETP4W {
        _SETP4W { w: self }
    }
    #[doc = "Bit 5 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp5(&mut self) -> _SETP5W {
        _SETP5W { w: self }
    }
    #[doc = "Bit 6 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp6(&mut self) -> _SETP6W {
        _SETP6W { w: self }
    }
    #[doc = "Bit 7 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp7(&mut self) -> _SETP7W {
        _SETP7W { w: self }
    }
    #[doc = "Bit 8 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp8(&mut self) -> _SETP8W {
        _SETP8W { w: self }
    }
    #[doc = "Bit 9 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp9(&mut self) -> _SETP9W {
        _SETP9W { w: self }
    }
    #[doc = "Bit 10 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp10(&mut self) -> _SETP10W {
        _SETP10W { w: self }
    }
    #[doc = "Bit 11 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp11(&mut self) -> _SETP11W {
        _SETP11W { w: self }
    }
    #[doc = "Bit 12 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp12(&mut self) -> _SETP12W {
        _SETP12W { w: self }
    }
    #[doc = "Bit 13 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp13(&mut self) -> _SETP13W {
        _SETP13W { w: self }
    }
    #[doc = "Bit 14 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp14(&mut self) -> _SETP14W {
        _SETP14W { w: self }
    }
    #[doc = "Bit 15 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp15(&mut self) -> _SETP15W {
        _SETP15W { w: self }
    }
    #[doc = "Bit 16 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp16(&mut self) -> _SETP16W {
        _SETP16W { w: self }
    }
    #[doc = "Bit 17 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp17(&mut self) -> _SETP17W {
        _SETP17W { w: self }
    }
    #[doc = "Bit 18 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp18(&mut self) -> _SETP18W {
        _SETP18W { w: self }
    }
    #[doc = "Bit 19 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp19(&mut self) -> _SETP19W {
        _SETP19W { w: self }
    }
    #[doc = "Bit 20 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp20(&mut self) -> _SETP20W {
        _SETP20W { w: self }
    }
    #[doc = "Bit 21 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp21(&mut self) -> _SETP21W {
        _SETP21W { w: self }
    }
    #[doc = "Bit 22 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp22(&mut self) -> _SETP22W {
        _SETP22W { w: self }
    }
    #[doc = "Bit 23 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp23(&mut self) -> _SETP23W {
        _SETP23W { w: self }
    }
    #[doc = "Bit 24 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp24(&mut self) -> _SETP24W {
        _SETP24W { w: self }
    }
    #[doc = "Bit 25 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp25(&mut self) -> _SETP25W {
        _SETP25W { w: self }
    }
    #[doc = "Bit 26 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp26(&mut self) -> _SETP26W {
        _SETP26W { w: self }
    }
    #[doc = "Bit 27 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp27(&mut self) -> _SETP27W {
        _SETP27W { w: self }
    }
    #[doc = "Bit 28 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp28(&mut self) -> _SETP28W {
        _SETP28W { w: self }
    }
    #[doc = "Bit 29 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp29(&mut self) -> _SETP29W {
        _SETP29W { w: self }
    }
    #[doc = "Bit 30 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp30(&mut self) -> _SETP30W {
        _SETP30W { w: self }
    }
    #[doc = "Bit 31 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline]
    pub fn setp31(&mut self) -> _SETP31W {
        _SETP31W { w: self }
    }
}
