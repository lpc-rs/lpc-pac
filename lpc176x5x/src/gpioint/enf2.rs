#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENF2 {
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
pub struct P2_0EFR {
    bits: bool,
}
impl P2_0EFR {
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
pub struct P2_1EFR {
    bits: bool,
}
impl P2_1EFR {
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
pub struct P2_2EFR {
    bits: bool,
}
impl P2_2EFR {
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
pub struct P2_3EFR {
    bits: bool,
}
impl P2_3EFR {
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
pub struct P2_4EFR {
    bits: bool,
}
impl P2_4EFR {
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
pub struct P2_5EFR {
    bits: bool,
}
impl P2_5EFR {
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
pub struct P2_6EFR {
    bits: bool,
}
impl P2_6EFR {
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
pub struct P2_7EFR {
    bits: bool,
}
impl P2_7EFR {
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
pub struct P2_8EFR {
    bits: bool,
}
impl P2_8EFR {
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
pub struct P2_9EFR {
    bits: bool,
}
impl P2_9EFR {
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
pub struct P2_10EFR {
    bits: bool,
}
impl P2_10EFR {
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
pub struct P2_11EFR {
    bits: bool,
}
impl P2_11EFR {
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
pub struct P2_12EFR {
    bits: bool,
}
impl P2_12EFR {
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
pub struct P2_13EFR {
    bits: bool,
}
impl P2_13EFR {
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
pub struct _P2_0EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_0EFW<'a> {
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
pub struct _P2_1EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_1EFW<'a> {
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
pub struct _P2_2EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_2EFW<'a> {
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
pub struct _P2_3EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_3EFW<'a> {
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
pub struct _P2_4EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_4EFW<'a> {
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
pub struct _P2_5EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_5EFW<'a> {
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
pub struct _P2_6EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_6EFW<'a> {
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
pub struct _P2_7EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_7EFW<'a> {
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
pub struct _P2_8EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_8EFW<'a> {
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
pub struct _P2_9EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_9EFW<'a> {
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
pub struct _P2_10EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_10EFW<'a> {
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
pub struct _P2_11EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_11EFW<'a> {
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
pub struct _P2_12EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_12EFW<'a> {
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
pub struct _P2_13EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_13EFW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable falling edge interrupt for P2\\[0\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_0ef(&self) -> P2_0EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_0EFR { bits }
    }
    #[doc = "Bit 1 - Enable falling edge interrupt for P2\\[1\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_1ef(&self) -> P2_1EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_1EFR { bits }
    }
    #[doc = "Bit 2 - Enable falling edge interrupt for P2\\[2\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_2ef(&self) -> P2_2EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_2EFR { bits }
    }
    #[doc = "Bit 3 - Enable falling edge interrupt for P2\\[3\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_3ef(&self) -> P2_3EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_3EFR { bits }
    }
    #[doc = "Bit 4 - Enable falling edge interrupt for P2\\[4\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_4ef(&self) -> P2_4EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_4EFR { bits }
    }
    #[doc = "Bit 5 - Enable falling edge interrupt for P2\\[5\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_5ef(&self) -> P2_5EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_5EFR { bits }
    }
    #[doc = "Bit 6 - Enable falling edge interrupt for P2\\[6\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_6ef(&self) -> P2_6EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_6EFR { bits }
    }
    #[doc = "Bit 7 - Enable falling edge interrupt for P2\\[7\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_7ef(&self) -> P2_7EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_7EFR { bits }
    }
    #[doc = "Bit 8 - Enable falling edge interrupt for P2\\[8\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_8ef(&self) -> P2_8EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_8EFR { bits }
    }
    #[doc = "Bit 9 - Enable falling edge interrupt for P2\\[9\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_9ef(&self) -> P2_9EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_9EFR { bits }
    }
    #[doc = "Bit 10 - Enable falling edge interrupt for P2\\[10\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_10ef(&self) -> P2_10EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_10EFR { bits }
    }
    #[doc = "Bit 11 - Enable falling edge interrupt for P2\\[11\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_11ef(&self) -> P2_11EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_11EFR { bits }
    }
    #[doc = "Bit 12 - Enable falling edge interrupt for P2\\[12\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_12ef(&self) -> P2_12EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_12EFR { bits }
    }
    #[doc = "Bit 13 - Enable falling edge interrupt for P2\\[13\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_13ef(&self) -> P2_13EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_13EFR { bits }
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
    #[doc = "Bit 0 - Enable falling edge interrupt for P2\\[0\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_0ef(&mut self) -> _P2_0EFW {
        _P2_0EFW { w: self }
    }
    #[doc = "Bit 1 - Enable falling edge interrupt for P2\\[1\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_1ef(&mut self) -> _P2_1EFW {
        _P2_1EFW { w: self }
    }
    #[doc = "Bit 2 - Enable falling edge interrupt for P2\\[2\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_2ef(&mut self) -> _P2_2EFW {
        _P2_2EFW { w: self }
    }
    #[doc = "Bit 3 - Enable falling edge interrupt for P2\\[3\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_3ef(&mut self) -> _P2_3EFW {
        _P2_3EFW { w: self }
    }
    #[doc = "Bit 4 - Enable falling edge interrupt for P2\\[4\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_4ef(&mut self) -> _P2_4EFW {
        _P2_4EFW { w: self }
    }
    #[doc = "Bit 5 - Enable falling edge interrupt for P2\\[5\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_5ef(&mut self) -> _P2_5EFW {
        _P2_5EFW { w: self }
    }
    #[doc = "Bit 6 - Enable falling edge interrupt for P2\\[6\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_6ef(&mut self) -> _P2_6EFW {
        _P2_6EFW { w: self }
    }
    #[doc = "Bit 7 - Enable falling edge interrupt for P2\\[7\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_7ef(&mut self) -> _P2_7EFW {
        _P2_7EFW { w: self }
    }
    #[doc = "Bit 8 - Enable falling edge interrupt for P2\\[8\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_8ef(&mut self) -> _P2_8EFW {
        _P2_8EFW { w: self }
    }
    #[doc = "Bit 9 - Enable falling edge interrupt for P2\\[9\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_9ef(&mut self) -> _P2_9EFW {
        _P2_9EFW { w: self }
    }
    #[doc = "Bit 10 - Enable falling edge interrupt for P2\\[10\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_10ef(&mut self) -> _P2_10EFW {
        _P2_10EFW { w: self }
    }
    #[doc = "Bit 11 - Enable falling edge interrupt for P2\\[11\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_11ef(&mut self) -> _P2_11EFW {
        _P2_11EFW { w: self }
    }
    #[doc = "Bit 12 - Enable falling edge interrupt for P2\\[12\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_12ef(&mut self) -> _P2_12EFW {
        _P2_12EFW { w: self }
    }
    #[doc = "Bit 13 - Enable falling edge interrupt for P2\\[13\\]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_13ef(&mut self) -> _P2_13EFW {
        _P2_13EFW { w: self }
    }
}
