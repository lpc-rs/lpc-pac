#[doc = "Reader of register ISO_PTD_BASE_ADDR"]
pub type R = crate::R<u32, super::ISO_PTD_BASE_ADDR>;
#[doc = "Writer for register ISO_PTD_BASE_ADDR"]
pub type W = crate::W<u32, super::ISO_PTD_BASE_ADDR>;
#[doc = "Register ISO_PTD_BASE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISO_PTD_BASE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISO_FIRST`"]
pub type ISO_FIRST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISO_FIRST`"]
pub struct ISO_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_FIRST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `ISO_BASE`"]
pub type ISO_BASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISO_BASE`"]
pub struct ISO_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub fn iso_first(&self) -> ISO_FIRST_R {
        ISO_FIRST_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub fn iso_base(&self) -> ISO_BASE_R {
        ISO_BASE_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub fn iso_first(&mut self) -> ISO_FIRST_W {
        ISO_FIRST_W { w: self }
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub fn iso_base(&mut self) -> ISO_BASE_W {
        ISO_BASE_W { w: self }
    }
}
