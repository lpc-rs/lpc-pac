#[doc = "Register `MAC_LPI_TIMER_CTRL` reader"]
pub struct R(crate::R<MAC_LPI_TIMER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_LPI_TIMER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_LPI_TIMER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_LPI_TIMER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_LPI_TIMER_CTRL` writer"]
pub struct W(crate::W<MAC_LPI_TIMER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_LPI_TIMER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MAC_LPI_TIMER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_LPI_TIMER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWT` reader - LPI TW Timer This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission."]
pub struct TWT_R(crate::FieldReader<u16, u16>);
impl TWT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWT` writer - LPI TW Timer This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission."]
pub struct TWT_W<'a> {
    w: &'a mut W,
}
impl<'a> TWT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `LST` reader - LPI LS Timer This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY."]
pub struct LST_R(crate::FieldReader<u16, u16>);
impl LST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LST_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LST` writer - LPI LS Timer This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY."]
pub struct LST_W<'a> {
    w: &'a mut W,
}
impl<'a> LST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPI Timers Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_lpi_timer_ctrl](index.html) module"]
pub struct MAC_LPI_TIMER_CTRL_SPEC;
impl crate::RegisterSpec for MAC_LPI_TIMER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_lpi_timer_ctrl::R](R) reader structure"]
impl crate::Readable for MAC_LPI_TIMER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_lpi_timer_ctrl::W](W) writer structure"]
impl crate::Writable for MAC_LPI_TIMER_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_LPI_TIMER_CTRL to value 0"]
impl crate::Resettable for MAC_LPI_TIMER_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
