#[doc = "Reader of register BTR"]
pub type R = crate::R<u32, super::BTR>;
#[doc = "Writer for register BTR"]
pub type W = crate::W<u32, super::BTR>;
#[doc = "Register BTR `reset()`'s with value 0x001c_0000"]
impl crate::ResetValue for super::BTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x001c_0000
    }
}
#[doc = "Reader of field `BRP`"]
pub type BRP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BRP`"]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `SJW`"]
pub type SJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SJW`"]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `TESG1`"]
pub type TESG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TESG1`"]
pub struct TESG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TESG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TESG2`"]
pub type TESG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TESG2`"]
pub struct TESG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TESG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAM_A {
    #[doc = "0: The bus is sampled once (recommended for high speed buses)"]
    THE_BUS_IS_SAMPLED_O = 0,
    #[doc = "1: The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    THE_BUS_IS_SAMPLED_3 = 1,
}
impl From<SAM_A> for bool {
    #[inline(always)]
    fn from(variant: SAM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAM`"]
pub type SAM_R = crate::R<bool, SAM_A>;
impl SAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAM_A {
        match self.bits {
            false => SAM_A::THE_BUS_IS_SAMPLED_O,
            true => SAM_A::THE_BUS_IS_SAMPLED_3,
        }
    }
    #[doc = "Checks if the value of the field is `THE_BUS_IS_SAMPLED_O`"]
    #[inline(always)]
    pub fn is_the_bus_is_sampled_o(&self) -> bool {
        *self == SAM_A::THE_BUS_IS_SAMPLED_O
    }
    #[doc = "Checks if the value of the field is `THE_BUS_IS_SAMPLED_3`"]
    #[inline(always)]
    pub fn is_the_bus_is_sampled_3(&self) -> bool {
        *self == SAM_A::THE_BUS_IS_SAMPLED_3
    }
}
#[doc = "Write proxy for field `SAM`"]
pub struct SAM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    #[inline(always)]
    pub fn the_bus_is_sampled_o(self) -> &'a mut W {
        self.variant(SAM_A::THE_BUS_IS_SAMPLED_O)
    }
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    #[inline(always)]
    pub fn the_bus_is_sampled_3(self) -> &'a mut W {
        self.variant(SAM_A::THE_BUS_IS_SAMPLED_3)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn tesg1(&self) -> TESG1_R {
        TESG1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline(always)]
    pub fn tesg2(&self) -> TESG2_R {
        TESG2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn tesg1(&mut self) -> TESG1_W {
        TESG1_W { w: self }
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline(always)]
    pub fn tesg2(&mut self) -> TESG2_W {
        TESG2_W { w: self }
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline(always)]
    pub fn sam(&mut self) -> SAM_W {
        SAM_W { w: self }
    }
}
