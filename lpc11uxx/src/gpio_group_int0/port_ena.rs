#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PORT_ENA {
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
pub struct ENA_0R {
    bits: bool,
}
impl ENA_0R {
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
pub struct ENA_1R {
    bits: bool,
}
impl ENA_1R {
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
pub struct ENA_2R {
    bits: bool,
}
impl ENA_2R {
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
pub struct ENA_3R {
    bits: bool,
}
impl ENA_3R {
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
pub struct ENA_4R {
    bits: bool,
}
impl ENA_4R {
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
pub struct ENA_5R {
    bits: bool,
}
impl ENA_5R {
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
pub struct ENA_6R {
    bits: bool,
}
impl ENA_6R {
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
pub struct ENA_7R {
    bits: bool,
}
impl ENA_7R {
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
pub struct ENA_8R {
    bits: bool,
}
impl ENA_8R {
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
pub struct ENA_9R {
    bits: bool,
}
impl ENA_9R {
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
pub struct ENA_10R {
    bits: bool,
}
impl ENA_10R {
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
pub struct ENA_11R {
    bits: bool,
}
impl ENA_11R {
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
pub struct ENA_12R {
    bits: bool,
}
impl ENA_12R {
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
pub struct ENA_13R {
    bits: bool,
}
impl ENA_13R {
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
pub struct ENA_14R {
    bits: bool,
}
impl ENA_14R {
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
pub struct ENA_15R {
    bits: bool,
}
impl ENA_15R {
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
pub struct ENA_16R {
    bits: bool,
}
impl ENA_16R {
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
pub struct ENA_17R {
    bits: bool,
}
impl ENA_17R {
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
pub struct ENA_18R {
    bits: bool,
}
impl ENA_18R {
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
pub struct ENA_19R {
    bits: bool,
}
impl ENA_19R {
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
pub struct ENA_20R {
    bits: bool,
}
impl ENA_20R {
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
pub struct ENA_21R {
    bits: bool,
}
impl ENA_21R {
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
pub struct ENA_22R {
    bits: bool,
}
impl ENA_22R {
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
pub struct ENA_23R {
    bits: bool,
}
impl ENA_23R {
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
pub struct ENA_24R {
    bits: bool,
}
impl ENA_24R {
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
pub struct ENA_25R {
    bits: bool,
}
impl ENA_25R {
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
pub struct ENA_26R {
    bits: bool,
}
impl ENA_26R {
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
pub struct ENA_27R {
    bits: bool,
}
impl ENA_27R {
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
pub struct ENA_28R {
    bits: bool,
}
impl ENA_28R {
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
pub struct ENA_29R {
    bits: bool,
}
impl ENA_29R {
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
pub struct ENA_30R {
    bits: bool,
}
impl ENA_30R {
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
pub struct ENA_31R {
    bits: bool,
}
impl ENA_31R {
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
pub struct _ENA_0W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_0W<'a> {
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
pub struct _ENA_1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_1W<'a> {
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
pub struct _ENA_2W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_2W<'a> {
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
pub struct _ENA_3W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_3W<'a> {
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
pub struct _ENA_4W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_4W<'a> {
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
pub struct _ENA_5W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_5W<'a> {
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
pub struct _ENA_6W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_6W<'a> {
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
pub struct _ENA_7W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_7W<'a> {
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
pub struct _ENA_8W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_8W<'a> {
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
pub struct _ENA_9W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_9W<'a> {
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
pub struct _ENA_10W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_10W<'a> {
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
pub struct _ENA_11W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_11W<'a> {
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
pub struct _ENA_12W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_12W<'a> {
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
pub struct _ENA_13W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_13W<'a> {
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
pub struct _ENA_14W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_14W<'a> {
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
pub struct _ENA_15W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_15W<'a> {
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
pub struct _ENA_16W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_16W<'a> {
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
pub struct _ENA_17W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_17W<'a> {
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
pub struct _ENA_18W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_18W<'a> {
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
pub struct _ENA_19W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_19W<'a> {
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
pub struct _ENA_20W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_20W<'a> {
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
pub struct _ENA_21W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_21W<'a> {
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
pub struct _ENA_22W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_22W<'a> {
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
pub struct _ENA_23W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_23W<'a> {
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
pub struct _ENA_24W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_24W<'a> {
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
pub struct _ENA_25W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_25W<'a> {
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
pub struct _ENA_26W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_26W<'a> {
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
pub struct _ENA_27W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_27W<'a> {
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
pub struct _ENA_28W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_28W<'a> {
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
pub struct _ENA_29W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_29W<'a> {
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
pub struct _ENA_30W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_30W<'a> {
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
pub struct _ENA_31W<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_31W<'a> {
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
    #[doc = "Bit 0 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_0(&self) -> ENA_0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_0R { bits }
    }
    #[doc = "Bit 1 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_1(&self) -> ENA_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_1R { bits }
    }
    #[doc = "Bit 2 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_2(&self) -> ENA_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_2R { bits }
    }
    #[doc = "Bit 3 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_3(&self) -> ENA_3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_3R { bits }
    }
    #[doc = "Bit 4 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_4(&self) -> ENA_4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_4R { bits }
    }
    #[doc = "Bit 5 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_5(&self) -> ENA_5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_5R { bits }
    }
    #[doc = "Bit 6 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_6(&self) -> ENA_6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_6R { bits }
    }
    #[doc = "Bit 7 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_7(&self) -> ENA_7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_7R { bits }
    }
    #[doc = "Bit 8 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_8(&self) -> ENA_8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_8R { bits }
    }
    #[doc = "Bit 9 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_9(&self) -> ENA_9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_9R { bits }
    }
    #[doc = "Bit 10 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_10(&self) -> ENA_10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_10R { bits }
    }
    #[doc = "Bit 11 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_11(&self) -> ENA_11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_11R { bits }
    }
    #[doc = "Bit 12 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_12(&self) -> ENA_12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_12R { bits }
    }
    #[doc = "Bit 13 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_13(&self) -> ENA_13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_13R { bits }
    }
    #[doc = "Bit 14 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_14(&self) -> ENA_14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_14R { bits }
    }
    #[doc = "Bit 15 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_15(&self) -> ENA_15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_15R { bits }
    }
    #[doc = "Bit 16 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_16(&self) -> ENA_16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_16R { bits }
    }
    #[doc = "Bit 17 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_17(&self) -> ENA_17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_17R { bits }
    }
    #[doc = "Bit 18 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_18(&self) -> ENA_18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_18R { bits }
    }
    #[doc = "Bit 19 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_19(&self) -> ENA_19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_19R { bits }
    }
    #[doc = "Bit 20 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_20(&self) -> ENA_20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_20R { bits }
    }
    #[doc = "Bit 21 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_21(&self) -> ENA_21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_21R { bits }
    }
    #[doc = "Bit 22 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_22(&self) -> ENA_22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_22R { bits }
    }
    #[doc = "Bit 23 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_23(&self) -> ENA_23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_23R { bits }
    }
    #[doc = "Bit 24 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_24(&self) -> ENA_24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_24R { bits }
    }
    #[doc = "Bit 25 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_25(&self) -> ENA_25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_25R { bits }
    }
    #[doc = "Bit 26 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_26(&self) -> ENA_26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_26R { bits }
    }
    #[doc = "Bit 27 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_27(&self) -> ENA_27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_27R { bits }
    }
    #[doc = "Bit 28 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_28(&self) -> ENA_28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_28R { bits }
    }
    #[doc = "Bit 29 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_29(&self) -> ENA_29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_29R { bits }
    }
    #[doc = "Bit 30 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_30(&self) -> ENA_30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_30R { bits }
    }
    #[doc = "Bit 31 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_31(&self) -> ENA_31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENA_31R { bits }
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
    #[doc = "Bit 0 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_0(&mut self) -> _ENA_0W {
        _ENA_0W { w: self }
    }
    #[doc = "Bit 1 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_1(&mut self) -> _ENA_1W {
        _ENA_1W { w: self }
    }
    #[doc = "Bit 2 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_2(&mut self) -> _ENA_2W {
        _ENA_2W { w: self }
    }
    #[doc = "Bit 3 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_3(&mut self) -> _ENA_3W {
        _ENA_3W { w: self }
    }
    #[doc = "Bit 4 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_4(&mut self) -> _ENA_4W {
        _ENA_4W { w: self }
    }
    #[doc = "Bit 5 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_5(&mut self) -> _ENA_5W {
        _ENA_5W { w: self }
    }
    #[doc = "Bit 6 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_6(&mut self) -> _ENA_6W {
        _ENA_6W { w: self }
    }
    #[doc = "Bit 7 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_7(&mut self) -> _ENA_7W {
        _ENA_7W { w: self }
    }
    #[doc = "Bit 8 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_8(&mut self) -> _ENA_8W {
        _ENA_8W { w: self }
    }
    #[doc = "Bit 9 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_9(&mut self) -> _ENA_9W {
        _ENA_9W { w: self }
    }
    #[doc = "Bit 10 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_10(&mut self) -> _ENA_10W {
        _ENA_10W { w: self }
    }
    #[doc = "Bit 11 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_11(&mut self) -> _ENA_11W {
        _ENA_11W { w: self }
    }
    #[doc = "Bit 12 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_12(&mut self) -> _ENA_12W {
        _ENA_12W { w: self }
    }
    #[doc = "Bit 13 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_13(&mut self) -> _ENA_13W {
        _ENA_13W { w: self }
    }
    #[doc = "Bit 14 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_14(&mut self) -> _ENA_14W {
        _ENA_14W { w: self }
    }
    #[doc = "Bit 15 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_15(&mut self) -> _ENA_15W {
        _ENA_15W { w: self }
    }
    #[doc = "Bit 16 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_16(&mut self) -> _ENA_16W {
        _ENA_16W { w: self }
    }
    #[doc = "Bit 17 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_17(&mut self) -> _ENA_17W {
        _ENA_17W { w: self }
    }
    #[doc = "Bit 18 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_18(&mut self) -> _ENA_18W {
        _ENA_18W { w: self }
    }
    #[doc = "Bit 19 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_19(&mut self) -> _ENA_19W {
        _ENA_19W { w: self }
    }
    #[doc = "Bit 20 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_20(&mut self) -> _ENA_20W {
        _ENA_20W { w: self }
    }
    #[doc = "Bit 21 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_21(&mut self) -> _ENA_21W {
        _ENA_21W { w: self }
    }
    #[doc = "Bit 22 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_22(&mut self) -> _ENA_22W {
        _ENA_22W { w: self }
    }
    #[doc = "Bit 23 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_23(&mut self) -> _ENA_23W {
        _ENA_23W { w: self }
    }
    #[doc = "Bit 24 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_24(&mut self) -> _ENA_24W {
        _ENA_24W { w: self }
    }
    #[doc = "Bit 25 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_25(&mut self) -> _ENA_25W {
        _ENA_25W { w: self }
    }
    #[doc = "Bit 26 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_26(&mut self) -> _ENA_26W {
        _ENA_26W { w: self }
    }
    #[doc = "Bit 27 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_27(&mut self) -> _ENA_27W {
        _ENA_27W { w: self }
    }
    #[doc = "Bit 28 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_28(&mut self) -> _ENA_28W {
        _ENA_28W { w: self }
    }
    #[doc = "Bit 29 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_29(&mut self) -> _ENA_29W {
        _ENA_29W { w: self }
    }
    #[doc = "Bit 30 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_30(&mut self) -> _ENA_30W {
        _ENA_30W { w: self }
    }
    #[doc = "Bit 31 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline]
    pub fn ena_31(&mut self) -> _ENA_31W {
        _ENA_31W { w: self }
    }
}
