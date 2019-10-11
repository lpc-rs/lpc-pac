#[doc = "Reader of register MSTDAT"]
pub type R = crate::R<u32, super::MSTDAT>;
#[doc = "Writer for register MSTDAT"]
pub type W = crate::W<u32, super::MSTDAT>;
#[doc = "Register MSTDAT `reset()`'s with value 0"]
impl crate::ResetValue for super::MSTDAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
