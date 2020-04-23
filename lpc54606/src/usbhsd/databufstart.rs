#[doc = "Reader of register DATABUFSTART"]
pub type R = crate::R<u32, super::DATABUFSTART>;
#[doc = "Writer for register DATABUFSTART"]
pub type W = crate::W<u32, super::DATABUFSTART>;
#[doc = "Register DATABUFSTART `reset()`'s with value 0x4100_0000"]
impl crate::ResetValue for super::DATABUFSTART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4100_0000
    }
}
#[doc = "Reader of field `DA_BUF`"]
pub type DA_BUF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DA_BUF`"]
pub struct DA_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_BUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Start address of the memory page where all endpoint data buffers are located."]
    #[inline(always)]
    pub fn da_buf(&self) -> DA_BUF_R {
        DA_BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of the memory page where all endpoint data buffers are located."]
    #[inline(always)]
    pub fn da_buf(&mut self) -> DA_BUF_W {
        DA_BUF_W { w: self }
    }
}
