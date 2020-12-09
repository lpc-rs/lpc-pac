#[doc = "Reader of register DATA_PAYLOAD_BASE_ADDR"]
pub type R = crate::R<u32, super::DATA_PAYLOAD_BASE_ADDR>;
#[doc = "Writer for register DATA_PAYLOAD_BASE_ADDR"]
pub type W = crate::W<u32, super::DATA_PAYLOAD_BASE_ADDR>;
#[doc = "Register DATA_PAYLOAD_BASE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_PAYLOAD_BASE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAT_BASE`"]
pub type DAT_BASE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DAT_BASE`"]
pub struct DAT_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    pub fn dat_base(&self) -> DAT_BASE_R {
        DAT_BASE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    pub fn dat_base(&mut self) -> DAT_BASE_W {
        DAT_BASE_W { w: self }
    }
}
