#[doc = "Reader of register MODCFG"]
pub type R = crate::R<u32, super::MODCFG>;
#[doc = "Writer for register MODCFG"]
pub type W = crate::W<u32, super::MODCFG>;
#[doc = "Register MODCFG `reset()`'s with value 0x01f4"]
impl crate::ResetValue for super::MODCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01f4
    }
}
#[doc = "Reader of field `NOC`"]
pub type NOC_R = crate::R<u8, u8>;
#[doc = "Reader of field `NOB`"]
pub type NOB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    pub fn noc(&self) -> NOC_R {
        NOC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (31 bits wide on this device.)"]
    #[inline(always)]
    pub fn nob(&self) -> NOB_R {
        NOB_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {}
