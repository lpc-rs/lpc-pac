#[doc = "Reader of register ATL_PTD_BASE_ADDR"]
pub type R = crate::R<u32, super::ATL_PTD_BASE_ADDR>;
#[doc = "Writer for register ATL_PTD_BASE_ADDR"]
pub type W = crate::W<u32, super::ATL_PTD_BASE_ADDR>;
#[doc = "Register ATL_PTD_BASE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ATL_PTD_BASE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ATL_CUR`"]
pub type ATL_CUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATL_CUR`"]
pub struct ATL_CUR_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_CUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ATL_BASE`"]
pub type ATL_BASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ATL_BASE`"]
pub struct ATL_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:8 - This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
    #[inline(always)]
    pub fn atl_cur(&self) -> ATL_CUR_R {
        ATL_CUR_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:31 - Base address to be used by the hardware to find the start of the ATL list."]
    #[inline(always)]
    pub fn atl_base(&self) -> ATL_BASE_R {
        ATL_BASE_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:8 - This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
    #[inline(always)]
    pub fn atl_cur(&mut self) -> ATL_CUR_W {
        ATL_CUR_W { w: self }
    }
    #[doc = "Bits 9:31 - Base address to be used by the hardware to find the start of the ATL list."]
    #[inline(always)]
    pub fn atl_base(&mut self) -> ATL_BASE_W {
        ATL_BASE_W { w: self }
    }
}
