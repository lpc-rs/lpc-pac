#[doc = "Reader of register MAC_LPI_TIMER_CTRL"]
pub type R = crate::R<u32, super::MAC_LPI_TIMER_CTRL>;
#[doc = "Writer for register MAC_LPI_TIMER_CTRL"]
pub type W = crate::W<u32, super::MAC_LPI_TIMER_CTRL>;
#[doc = "Register MAC_LPI_TIMER_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_LPI_TIMER_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TWT`"]
pub type TWT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TWT`"]
pub struct TWT_W<'a> {
    w: &'a mut W,
}
impl<'a> TWT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `LST`"]
pub type LST_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LST`"]
pub struct LST_W<'a> {
    w: &'a mut W,
}
impl<'a> LST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - LPI TW Timer This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission."]
    #[inline(always)]
    pub fn twt(&self) -> TWT_R {
        TWT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - LPI LS Timer This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY."]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPI TW Timer This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission."]
    #[inline(always)]
    pub fn twt(&mut self) -> TWT_W {
        TWT_W { w: self }
    }
    #[doc = "Bits 16:25 - LPI LS Timer This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY."]
    #[inline(always)]
    pub fn lst(&mut self) -> LST_W {
        LST_W { w: self }
    }
}
