#[doc = "Reader of register OSR"]
pub type R = crate::R<u32, super::OSR>;
#[doc = "Writer for register OSR"]
pub type W = crate::W<u32, super::OSR>;
#[doc = "Register OSR `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::OSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `OSRVAL`"]
pub type OSRVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSRVAL`"]
pub struct OSRVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSRVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit."]
    #[inline(always)]
    pub fn osrval(&self) -> OSRVAL_R {
        OSRVAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Oversample Selection Value. 0 to 3 = not supported 0x4 = 5 function clocks are used to transmit and receive each data bit. 0x5 = 6 function clocks are used to transmit and receive each data bit. 0xF= 16 function clocks are used to transmit and receive each data bit."]
    #[inline(always)]
    pub fn osrval(&mut self) -> OSRVAL_W {
        OSRVAL_W { w: self }
    }
}
