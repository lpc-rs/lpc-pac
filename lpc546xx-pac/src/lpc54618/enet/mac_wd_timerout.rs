///Register `MAC_WD_TIMEROUT` reader
pub struct R(crate::R<MAC_WD_TIMEROUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_WD_TIMEROUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_WD_TIMEROUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_WD_TIMEROUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAC_WD_TIMEROUT` writer
pub struct W(crate::W<MAC_WD_TIMEROUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_WD_TIMEROUT_SPEC>;
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
impl From<crate::W<MAC_WD_TIMEROUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_WD_TIMEROUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WTO` reader - Watchdog Timeout When the PWE bit is set and the WD bit of the MAC Configuration register Table 722 is reset, this field is used as watchdog timeout for a received packet.
pub struct WTO_R(crate::FieldReader<u8, u8>);
impl WTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WTO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WTO` writer - Watchdog Timeout When the PWE bit is set and the WD bit of the MAC Configuration register Table 722 is reset, this field is used as watchdog timeout for a received packet.
pub struct WTO_W<'a> {
    w: &'a mut W,
}
impl<'a> WTO_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
///Field `PWE` reader - Programmable Watchdog Enable When this bit is set and the WD bit of the MAC Configuration register Table 722 is reset, the WTO field is used as watchdog timeout for a received packet.
pub struct PWE_R(crate::FieldReader<bool, bool>);
impl PWE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PWE` writer - Programmable Watchdog Enable When this bit is set and the WD bit of the MAC Configuration register Table 722 is reset, the WTO field is used as watchdog timeout for a received packet.
pub struct PWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWE_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Watchdog Timeout When the PWE bit is set and the WD bit of the MAC Configuration register Table 722 is reset, this field is used as watchdog timeout for a received packet.
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Programmable Watchdog Enable When this bit is set and the WD bit of the MAC Configuration register Table 722 is reset, the WTO field is used as watchdog timeout for a received packet.
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:3 - Watchdog Timeout When the PWE bit is set and the WD bit of the MAC Configuration register Table 722 is reset, this field is used as watchdog timeout for a received packet.
    #[inline(always)]
    pub fn wto(&mut self) -> WTO_W {
        WTO_W { w: self }
    }
    ///Bit 8 - Programmable Watchdog Enable When this bit is set and the WD bit of the MAC Configuration register Table 722 is reset, the WTO field is used as watchdog timeout for a received packet.
    #[inline(always)]
    pub fn pwe(&mut self) -> PWE_W {
        PWE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MAC watchdog Timeout register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_wd_timerout](index.html) module
pub struct MAC_WD_TIMEROUT_SPEC;
impl crate::RegisterSpec for MAC_WD_TIMEROUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_wd_timerout::R](R) reader structure
impl crate::Readable for MAC_WD_TIMEROUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mac_wd_timerout::W](W) writer structure
impl crate::Writable for MAC_WD_TIMEROUT_SPEC {
    type Writer = W;
}
///`reset()` method sets MAC_WD_TIMEROUT to value 0
impl crate::Resettable for MAC_WD_TIMEROUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
