#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMACREQSEL {
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
pub struct DMASEL08R {
    bits: bool,
}
impl DMASEL08R {
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
pub struct DMASEL09R {
    bits: bool,
}
impl DMASEL09R {
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
pub struct DMASEL10R {
    bits: bool,
}
impl DMASEL10R {
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
pub struct DMASEL11R {
    bits: bool,
}
impl DMASEL11R {
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
pub struct DMASEL12R {
    bits: bool,
}
impl DMASEL12R {
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
pub struct DMASEL13R {
    bits: bool,
}
impl DMASEL13R {
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
pub struct DMASEL14R {
    bits: bool,
}
impl DMASEL14R {
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
pub struct DMASEL15R {
    bits: bool,
}
impl DMASEL15R {
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
pub struct _DMASEL08W<'a> {
    w: &'a mut W,
}
impl<'a> _DMASEL08W<'a> {
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
pub struct _DMASEL09W<'a> {
    w: &'a mut W,
}
impl<'a> _DMASEL09W<'a> {
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
pub struct _DMASEL10W<'a> {
    w: &'a mut W,
}
impl<'a> _DMASEL10W<'a> {
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
pub struct _DMASEL11W<'a> {
    w: &'a mut W,
}
impl<'a> _DMASEL11W<'a> {
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
pub struct _DMASEL12W<'a> {
    w: &'a mut W,
}
impl<'a> _DMASEL12W<'a> {
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
pub struct _DMASEL13W<'a> {
    w: &'a mut W,
}
impl<'a> _DMASEL13W<'a> {
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
pub struct _DMASEL14W<'a> {
    w: &'a mut W,
}
impl<'a> _DMASEL14W<'a> {
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
pub struct _DMASEL15W<'a> {
    w: &'a mut W,
}
impl<'a> _DMASEL15W<'a> {
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
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
    #[inline]
    pub fn dmasel08(&self) -> DMASEL08R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMASEL08R { bits }
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
    #[inline]
    pub fn dmasel09(&self) -> DMASEL09R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMASEL09R { bits }
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
    #[inline]
    pub fn dmasel10(&self) -> DMASEL10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMASEL10R { bits }
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
    #[inline]
    pub fn dmasel11(&self) -> DMASEL11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMASEL11R { bits }
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
    #[inline]
    pub fn dmasel12(&self) -> DMASEL12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMASEL12R { bits }
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
    #[inline]
    pub fn dmasel13(&self) -> DMASEL13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMASEL13R { bits }
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
    #[inline]
    pub fn dmasel14(&self) -> DMASEL14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMASEL14R { bits }
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
    #[inline]
    pub fn dmasel15(&self) -> DMASEL15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMASEL15R { bits }
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
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 8: 0 - uart0 tx 1 - Timer 0 match 0 is selected."]
    #[inline]
    pub fn dmasel08(&mut self) -> _DMASEL08W {
        _DMASEL08W { w: self }
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 9: 0 - uart0 rx 1 - Timer 0 match 1 is selected."]
    #[inline]
    pub fn dmasel09(&mut self) -> _DMASEL09W {
        _DMASEL09W { w: self }
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 10: 0 - uart1 tx is selected. 1 - Timer 1 match 0 is selected."]
    #[inline]
    pub fn dmasel10(&mut self) -> _DMASEL10W {
        _DMASEL10W { w: self }
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 11: 0 - uart1 rx is selected. 1 - Timer 1 match 1 is selected."]
    #[inline]
    pub fn dmasel11(&mut self) -> _DMASEL11W {
        _DMASEL11W { w: self }
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 12: 0 - uart2 tx is selected. 1 - Timer 2 match 0 is selected."]
    #[inline]
    pub fn dmasel12(&mut self) -> _DMASEL12W {
        _DMASEL12W { w: self }
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 13: 0 - uart2 rx is selected. 1 - Timer 2 match 1 is selected."]
    #[inline]
    pub fn dmasel13(&mut self) -> _DMASEL13W {
        _DMASEL13W { w: self }
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 14: 0 - uart3 tx is selected. 1 - I2S channel 0 is selected."]
    #[inline]
    pub fn dmasel14(&mut self) -> _DMASEL14W {
        _DMASEL14W { w: self }
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 15: 0 - uart3 rx is selected. 1 - I2S channel 1 is selected."]
    #[inline]
    pub fn dmasel15(&mut self) -> _DMASEL15W {
        _DMASEL15W { w: self }
    }
}
