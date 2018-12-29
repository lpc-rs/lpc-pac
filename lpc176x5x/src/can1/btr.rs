#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BTR {
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
pub struct BRPR {
    bits: u16,
}
impl BRPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SJWR {
    bits: u8,
}
impl SJWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TESG1R {
    bits: u8,
}
impl TESG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TESG2R {
    bits: u8,
}
impl TESG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMR {
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    THE_BUS_IS_SAMPLED_O,
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    THE_BUS_IS_SAMPLED_3,
}
impl SAMR {
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
            SAMR::THE_BUS_IS_SAMPLED_O => false,
            SAMR::THE_BUS_IS_SAMPLED_3 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAMR {
        match value {
            false => SAMR::THE_BUS_IS_SAMPLED_O,
            true => SAMR::THE_BUS_IS_SAMPLED_3,
        }
    }
    #[doc = "Checks if the value of the field is `THE_BUS_IS_SAMPLED_O`"]
    #[inline]
    pub fn is_the_bus_is_sampled_o(&self) -> bool {
        *self == SAMR::THE_BUS_IS_SAMPLED_O
    }
    #[doc = "Checks if the value of the field is `THE_BUS_IS_SAMPLED_3`"]
    #[inline]
    pub fn is_the_bus_is_sampled_3(&self) -> bool {
        *self == SAMR::THE_BUS_IS_SAMPLED_3
    }
}
#[doc = r" Proxy"]
pub struct _BRPW<'a> {
    w: &'a mut W,
}
impl<'a> _BRPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SJWW<'a> {
    w: &'a mut W,
}
impl<'a> _SJWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TESG1W<'a> {
    w: &'a mut W,
}
impl<'a> _TESG1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TESG2W<'a> {
    w: &'a mut W,
}
impl<'a> _TESG2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAM`"]
pub enum SAMW {
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    THE_BUS_IS_SAMPLED_O,
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    THE_BUS_IS_SAMPLED_3,
}
impl SAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAMW::THE_BUS_IS_SAMPLED_O => false,
            SAMW::THE_BUS_IS_SAMPLED_3 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    #[inline]
    pub fn the_bus_is_sampled_o(self) -> &'a mut W {
        self.variant(SAMW::THE_BUS_IS_SAMPLED_O)
    }
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    #[inline]
    pub fn the_bus_is_sampled_3(self) -> &'a mut W {
        self.variant(SAMW::THE_BUS_IS_SAMPLED_3)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline]
    pub fn brp(&self) -> BRPR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BRPR { bits }
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline]
    pub fn sjw(&self) -> SJWR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SJWR { bits }
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline]
    pub fn tesg1(&self) -> TESG1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TESG1R { bits }
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline]
    pub fn tesg2(&self) -> TESG2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TESG2R { bits }
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline]
    pub fn sam(&self) -> SAMR {
        SAMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1835008 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline]
    pub fn brp(&mut self) -> _BRPW {
        _BRPW { w: self }
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline]
    pub fn sjw(&mut self) -> _SJWW {
        _SJWW { w: self }
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline]
    pub fn tesg1(&mut self) -> _TESG1W {
        _TESG1W { w: self }
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline]
    pub fn tesg2(&mut self) -> _TESG2W {
        _TESG2W { w: self }
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline]
    pub fn sam(&mut self) -> _SAMW {
        _SAMW { w: self }
    }
}
