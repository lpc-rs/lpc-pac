///Register `MAC_MDIO_ADDR` reader
pub struct R(crate::R<MAC_MDIO_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_MDIO_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_MDIO_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_MDIO_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAC_MDIO_ADDR` writer
pub struct W(crate::W<MAC_MDIO_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_MDIO_ADDR_SPEC>;
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
impl From<crate::W<MAC_MDIO_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_MDIO_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MB` reader - MII busy.
pub struct MB_R(crate::FieldReader<bool, bool>);
impl MB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MB` writer - MII busy.
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
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
///Field `MOC` reader - MII Operation Command.
pub struct MOC_R(crate::FieldReader<u8, u8>);
impl MOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MOC` writer - MII Operation Command.
pub struct MOC_W<'a> {
    w: &'a mut W,
}
impl<'a> MOC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Field `CR` reader - CSR Clock Range.
pub struct CR_R(crate::FieldReader<u8, u8>);
impl CR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CR` writer - CSR Clock Range.
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///Field `NTC` reader - Number of Training Clocks This field controls the number of trailing clock cycles generated on MDC after the end of transmission of MDIO frame.
pub struct NTC_R(crate::FieldReader<u8, u8>);
impl NTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NTC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NTC` writer - Number of Training Clocks This field controls the number of trailing clock cycles generated on MDC after the end of transmission of MDIO frame.
pub struct NTC_W<'a> {
    w: &'a mut W,
}
impl<'a> NTC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
///Field `RDA` reader - Register/Device Address These bits select the PHY register in selected PHY device.
pub struct RDA_R(crate::FieldReader<u8, u8>);
impl RDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RDA` writer - Register/Device Address These bits select the PHY register in selected PHY device.
pub struct RDA_W<'a> {
    w: &'a mut W,
}
impl<'a> RDA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
///Field `PA` reader - Physical Layer Address This field indicates which PHY devices (out of 32 devices) the MAC is accessing.
pub struct PA_R(crate::FieldReader<u8, u8>);
impl PA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PA` writer - Physical Layer Address This field indicates which PHY devices (out of 32 devices) the MAC is accessing.
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | ((value as u32 & 0x1f) << 21);
        self.w
    }
}
///Field `BTB` reader - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC will inform the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted).
pub struct BTB_R(crate::FieldReader<bool, bool>);
impl BTB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BTB` writer - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC will inform the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted).
pub struct BTB_W<'a> {
    w: &'a mut W,
}
impl<'a> BTB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
///Field `PSE` reader - Preamble Suppression Enable When this bit is set, the SMA will suppress the 32-bit preamble and transmit MDIO frames with only 1 preamble bit.
pub struct PSE_R(crate::FieldReader<bool, bool>);
impl PSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PSE` writer - Preamble Suppression Enable When this bit is set, the SMA will suppress the 32-bit preamble and transmit MDIO frames with only 1 preamble bit.
pub struct PSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    ///Bit 0 - MII busy.
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 2:3 - MII Operation Command.
    #[inline(always)]
    pub fn moc(&self) -> MOC_R {
        MOC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 8:11 - CSR Clock Range.
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - Number of Training Clocks This field controls the number of trailing clock cycles generated on MDC after the end of transmission of MDIO frame.
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    ///Bits 16:20 - Register/Device Address These bits select the PHY register in selected PHY device.
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - Physical Layer Address This field indicates which PHY devices (out of 32 devices) the MAC is accessing.
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bit 26 - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC will inform the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted).
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - Preamble Suppression Enable When this bit is set, the SMA will suppress the 32-bit preamble and transmit MDIO frames with only 1 preamble bit.
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - MII busy.
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
    ///Bits 2:3 - MII Operation Command.
    #[inline(always)]
    pub fn moc(&mut self) -> MOC_W {
        MOC_W { w: self }
    }
    ///Bits 8:11 - CSR Clock Range.
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    ///Bits 12:14 - Number of Training Clocks This field controls the number of trailing clock cycles generated on MDC after the end of transmission of MDIO frame.
    #[inline(always)]
    pub fn ntc(&mut self) -> NTC_W {
        NTC_W { w: self }
    }
    ///Bits 16:20 - Register/Device Address These bits select the PHY register in selected PHY device.
    #[inline(always)]
    pub fn rda(&mut self) -> RDA_W {
        RDA_W { w: self }
    }
    ///Bits 21:25 - Physical Layer Address This field indicates which PHY devices (out of 32 devices) the MAC is accessing.
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    ///Bit 26 - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC will inform the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted).
    #[inline(always)]
    pub fn btb(&mut self) -> BTB_W {
        BTB_W { w: self }
    }
    ///Bit 27 - Preamble Suppression Enable When this bit is set, the SMA will suppress the 32-bit preamble and transmit MDIO frames with only 1 preamble bit.
    #[inline(always)]
    pub fn pse(&mut self) -> PSE_W {
        PSE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MIDO address Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_mdio_addr](index.html) module
pub struct MAC_MDIO_ADDR_SPEC;
impl crate::RegisterSpec for MAC_MDIO_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_mdio_addr::R](R) reader structure
impl crate::Readable for MAC_MDIO_ADDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mac_mdio_addr::W](W) writer structure
impl crate::Writable for MAC_MDIO_ADDR_SPEC {
    type Writer = W;
}
///`reset()` method sets MAC_MDIO_ADDR to value 0
impl crate::Resettable for MAC_MDIO_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
