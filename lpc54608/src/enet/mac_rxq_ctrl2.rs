#[doc = "Reader of register MAC_RXQ_CTRL2"]
pub type R = crate::R<u32, super::MAC_RXQ_CTRL2>;
#[doc = "Writer for register MAC_RXQ_CTRL2"]
pub type W = crate::W<u32, super::MAC_RXQ_CTRL2>;
#[doc = "Register MAC_RXQ_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_RXQ_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSRQ0`"]
pub type PSRQ0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSRQ0`"]
pub struct PSRQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PSRQ1`"]
pub type PSRQ1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSRQ1`"]
pub struct PSRQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PSRQ2`"]
pub type PSRQ2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSRQ2`"]
pub struct PSRQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PSRQ3`"]
pub type PSRQ3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSRQ3`"]
pub struct PSRQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRQ3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priorities Selected in the Receive Queue 0."]
    #[inline(always)]
    pub fn psrq0(&self) -> PSRQ0_R {
        PSRQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priorities Selected in the Receive Queue 1."]
    #[inline(always)]
    pub fn psrq1(&self) -> PSRQ1_R {
        PSRQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Priorities Selected in the Receive Queue 2."]
    #[inline(always)]
    pub fn psrq2(&self) -> PSRQ2_R {
        PSRQ2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Priorities Selected in the Receive Queue 3."]
    #[inline(always)]
    pub fn psrq3(&self) -> PSRQ3_R {
        PSRQ3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priorities Selected in the Receive Queue 0."]
    #[inline(always)]
    pub fn psrq0(&mut self) -> PSRQ0_W {
        PSRQ0_W { w: self }
    }
    #[doc = "Bits 8:15 - Priorities Selected in the Receive Queue 1."]
    #[inline(always)]
    pub fn psrq1(&mut self) -> PSRQ1_W {
        PSRQ1_W { w: self }
    }
    #[doc = "Bits 16:23 - Priorities Selected in the Receive Queue 2."]
    #[inline(always)]
    pub fn psrq2(&mut self) -> PSRQ2_W {
        PSRQ2_W { w: self }
    }
    #[doc = "Bits 24:31 - Priorities Selected in the Receive Queue 3."]
    #[inline(always)]
    pub fn psrq3(&mut self) -> PSRQ3_W {
        PSRQ3_W { w: self }
    }
}
