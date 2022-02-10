///Register `PORTMODE` reader
pub struct R(crate::R<PORTMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTMODE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PORTMODE` writer
pub struct W(crate::W<PORTMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTMODE_SPEC>;
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
impl From<crate::W<PORTMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTMODE_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ID0` reader - Port 0 ID pin value.
pub struct ID0_R(crate::FieldReader<bool, bool>);
impl ID0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ID0` writer - Port 0 ID pin value.
pub struct ID0_W<'a> {
    w: &'a mut W,
}
impl<'a> ID0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Field `ID0_EN` reader - Port 0 ID pin pull-up enable.
pub struct ID0_EN_R(crate::FieldReader<bool, bool>);
impl ID0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ID0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ID0_EN` writer - Port 0 ID pin pull-up enable.
pub struct ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ID0_EN_W<'a> {
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
///Field `DEV_ENABLE` reader - If this bit is set to one, one of the ports will behave as a USB device.
pub struct DEV_ENABLE_R(crate::FieldReader<bool, bool>);
impl DEV_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DEV_ENABLE` writer - If this bit is set to one, one of the ports will behave as a USB device.
pub struct DEV_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Field `SW_CTRL_PDCOM` reader - This bit indicates if the PHY power-down input is controlled by software or by hardware.
pub struct SW_CTRL_PDCOM_R(crate::FieldReader<bool, bool>);
impl SW_CTRL_PDCOM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_CTRL_PDCOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_CTRL_PDCOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SW_CTRL_PDCOM` writer - This bit indicates if the PHY power-down input is controlled by software or by hardware.
pub struct SW_CTRL_PDCOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_CTRL_PDCOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///Field `SW_PDCOM` reader - This bit is only used when SW_CTRL_PDCOM is set to 1b.
pub struct SW_PDCOM_R(crate::FieldReader<bool, bool>);
impl SW_PDCOM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_PDCOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_PDCOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SW_PDCOM` writer - This bit is only used when SW_CTRL_PDCOM is set to 1b.
pub struct SW_PDCOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_PDCOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    ///Bit 0 - Port 0 ID pin value.
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 8 - Port 0 ID pin pull-up enable.
    #[inline(always)]
    pub fn id0_en(&self) -> ID0_EN_R {
        ID0_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 16 - If this bit is set to one, one of the ports will behave as a USB device.
    #[inline(always)]
    pub fn dev_enable(&self) -> DEV_ENABLE_R {
        DEV_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 18 - This bit indicates if the PHY power-down input is controlled by software or by hardware.
    #[inline(always)]
    pub fn sw_ctrl_pdcom(&self) -> SW_CTRL_PDCOM_R {
        SW_CTRL_PDCOM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - This bit is only used when SW_CTRL_PDCOM is set to 1b.
    #[inline(always)]
    pub fn sw_pdcom(&self) -> SW_PDCOM_R {
        SW_PDCOM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Port 0 ID pin value.
    #[inline(always)]
    pub fn id0(&mut self) -> ID0_W {
        ID0_W { w: self }
    }
    ///Bit 8 - Port 0 ID pin pull-up enable.
    #[inline(always)]
    pub fn id0_en(&mut self) -> ID0_EN_W {
        ID0_EN_W { w: self }
    }
    ///Bit 16 - If this bit is set to one, one of the ports will behave as a USB device.
    #[inline(always)]
    pub fn dev_enable(&mut self) -> DEV_ENABLE_W {
        DEV_ENABLE_W { w: self }
    }
    ///Bit 18 - This bit indicates if the PHY power-down input is controlled by software or by hardware.
    #[inline(always)]
    pub fn sw_ctrl_pdcom(&mut self) -> SW_CTRL_PDCOM_W {
        SW_CTRL_PDCOM_W { w: self }
    }
    ///Bit 19 - This bit is only used when SW_CTRL_PDCOM is set to 1b.
    #[inline(always)]
    pub fn sw_pdcom(&mut self) -> SW_PDCOM_W {
        SW_PDCOM_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Controls the port if it is attached to the host block or the device block
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [portmode](index.html) module
pub struct PORTMODE_SPEC;
impl crate::RegisterSpec for PORTMODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [portmode::R](R) reader structure
impl crate::Readable for PORTMODE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [portmode::W](W) writer structure
impl crate::Writable for PORTMODE_SPEC {
    type Writer = W;
}
///`reset()` method sets PORTMODE to value 0x0004_0000
impl crate::Resettable for PORTMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0000
    }
}
