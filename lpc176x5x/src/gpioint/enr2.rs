#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENR2 {
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
pub struct P2_0ERR {
    bits: bool,
}
impl P2_0ERR {
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
pub struct P2_1ERR {
    bits: bool,
}
impl P2_1ERR {
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
pub struct P2_2ERR {
    bits: bool,
}
impl P2_2ERR {
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
pub struct P2_3ERR {
    bits: bool,
}
impl P2_3ERR {
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
pub struct P2_4ERR {
    bits: bool,
}
impl P2_4ERR {
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
pub struct P2_5ERR {
    bits: bool,
}
impl P2_5ERR {
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
pub struct P2_6ERR {
    bits: bool,
}
impl P2_6ERR {
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
pub struct P2_7ERR {
    bits: bool,
}
impl P2_7ERR {
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
pub struct P2_8ERR {
    bits: bool,
}
impl P2_8ERR {
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
pub struct P2_9ERR {
    bits: bool,
}
impl P2_9ERR {
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
pub struct P2_10ERR {
    bits: bool,
}
impl P2_10ERR {
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
pub struct P2_11ERR {
    bits: bool,
}
impl P2_11ERR {
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
pub struct P2_12ERR {
    bits: bool,
}
impl P2_12ERR {
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
pub struct P2_13ERR {
    bits: bool,
}
impl P2_13ERR {
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
pub struct _P2_0ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_0ERW<'a> {
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
pub struct _P2_1ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_1ERW<'a> {
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
pub struct _P2_2ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_2ERW<'a> {
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
pub struct _P2_3ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_3ERW<'a> {
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
pub struct _P2_4ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_4ERW<'a> {
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
pub struct _P2_5ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_5ERW<'a> {
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
pub struct _P2_6ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_6ERW<'a> {
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
pub struct _P2_7ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_7ERW<'a> {
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
pub struct _P2_8ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_8ERW<'a> {
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
pub struct _P2_9ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_9ERW<'a> {
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
pub struct _P2_10ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_10ERW<'a> {
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
pub struct _P2_11ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_11ERW<'a> {
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
pub struct _P2_12ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_12ERW<'a> {
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
pub struct _P2_13ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_13ERW<'a> {
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
    #[doc = "Bit 0 - Enable rising edge interrupt for P2\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_0er(&self) -> P2_0ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_0ERR { bits }
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P2\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_1er(&self) -> P2_1ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_1ERR { bits }
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P2\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_2er(&self) -> P2_2ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_2ERR { bits }
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P2\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_3er(&self) -> P2_3ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_3ERR { bits }
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P2\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_4er(&self) -> P2_4ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_4ERR { bits }
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P2\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_5er(&self) -> P2_5ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_5ERR { bits }
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P2\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_6er(&self) -> P2_6ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_6ERR { bits }
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P2\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_7er(&self) -> P2_7ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_7ERR { bits }
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P2\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_8er(&self) -> P2_8ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_8ERR { bits }
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P2\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_9er(&self) -> P2_9ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_9ERR { bits }
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P2\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_10er(&self) -> P2_10ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_10ERR { bits }
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P2\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_11er(&self) -> P2_11ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_11ERR { bits }
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P2\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_12er(&self) -> P2_12ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_12ERR { bits }
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P2\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_13er(&self) -> P2_13ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_13ERR { bits }
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
    #[doc = "Bit 0 - Enable rising edge interrupt for P2\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_0er(&mut self) -> _P2_0ERW {
        _P2_0ERW { w: self }
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P2\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_1er(&mut self) -> _P2_1ERW {
        _P2_1ERW { w: self }
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P2\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_2er(&mut self) -> _P2_2ERW {
        _P2_2ERW { w: self }
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P2\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_3er(&mut self) -> _P2_3ERW {
        _P2_3ERW { w: self }
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P2\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_4er(&mut self) -> _P2_4ERW {
        _P2_4ERW { w: self }
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P2\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_5er(&mut self) -> _P2_5ERW {
        _P2_5ERW { w: self }
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P2\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_6er(&mut self) -> _P2_6ERW {
        _P2_6ERW { w: self }
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P2\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_7er(&mut self) -> _P2_7ERW {
        _P2_7ERW { w: self }
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P2\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_8er(&mut self) -> _P2_8ERW {
        _P2_8ERW { w: self }
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P2\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_9er(&mut self) -> _P2_9ERW {
        _P2_9ERW { w: self }
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P2\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_10er(&mut self) -> _P2_10ERW {
        _P2_10ERW { w: self }
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P2\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_11er(&mut self) -> _P2_11ERW {
        _P2_11ERW { w: self }
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P2\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_12er(&mut self) -> _P2_12ERW {
        _P2_12ERW { w: self }
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P2\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_13er(&mut self) -> _P2_13ERW {
        _P2_13ERW { w: self }
    }
}
