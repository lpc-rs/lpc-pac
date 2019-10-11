#[doc = "Reader of register DR"]
pub type R = crate::R<u32, super::DR>;
#[doc = "Writer for register DR"]
pub type W = crate::W<u32, super::DR>;
#[doc = "Register DR `reset()`'s with value 0"]
impl crate::ResetValue for super::DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATALOW`"]
pub type DATALOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATALOW`"]
pub struct DATALOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATAHIGH`"]
pub type DATAHIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAHIGH`"]
pub struct DATAHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SPI Bi-directional data port."]
    #[inline(always)]
    pub fn datalow(&self) -> DATALOW_R {
        DATALOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes."]
    #[inline(always)]
    pub fn datahigh(&self) -> DATAHIGH_R {
        DATAHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI Bi-directional data port."]
    #[inline(always)]
    pub fn datalow(&mut self) -> DATALOW_W {
        DATALOW_W { w: self }
    }
    #[doc = "Bits 8:15 - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes."]
    #[inline(always)]
    pub fn datahigh(&mut self) -> DATAHIGH_W {
        DATAHIGH_W { w: self }
    }
}
