///Register `STATICCONFIG` reader
pub struct R(crate::R<STATICCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `STATICCONFIG` writer
pub struct W(crate::W<STATICCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICCONFIG_SPEC>;
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
impl From<crate::W<STATICCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MW` reader - Memory width.
pub struct MW_R(crate::FieldReader<u8, u8>);
impl MW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MW` writer - Memory width.
pub struct MW_W<'a> {
    w: &'a mut W,
}
impl<'a> MW_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
///Field `PM` reader - Page mode.
pub struct PM_R(crate::FieldReader<bool, bool>);
impl PM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PM` writer - Page mode.
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `PC` reader - Chip select polarity.
pub struct PC_R(crate::FieldReader<bool, bool>);
impl PC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PC` writer - Chip select polarity.
pub struct PC_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Field `PB` reader - Byte lane state.
pub struct PB_R(crate::FieldReader<bool, bool>);
impl PB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PB` writer - Byte lane state.
pub struct PB_W<'a> {
    w: &'a mut W,
}
impl<'a> PB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///Field `EW` reader - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers.
pub struct EW_R(crate::FieldReader<bool, bool>);
impl EW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EW` writer - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers.
pub struct EW_W<'a> {
    w: &'a mut W,
}
impl<'a> EW_W<'a> {
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
///Field `B` reader - Buffer enable \[2\].
pub struct B_R(crate::FieldReader<bool, bool>);
impl B_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `B` writer - Buffer enable \[2\].
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
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
///Field `P` reader - Write protect.
pub struct P_R(crate::FieldReader<bool, bool>);
impl P_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `P` writer - Write protect.
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Memory width.
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new((self.bits & 0x03) as u8)
    }
    ///Bit 3 - Page mode.
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 6 - Chip select polarity.
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Byte lane state.
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers.
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 19 - Buffer enable \[2\].
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Write protect.
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:1 - Memory width.
    #[inline(always)]
    pub fn mw(&mut self) -> MW_W {
        MW_W { w: self }
    }
    ///Bit 3 - Page mode.
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    ///Bit 6 - Chip select polarity.
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W {
        PC_W { w: self }
    }
    ///Bit 7 - Byte lane state.
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W {
        PB_W { w: self }
    }
    ///Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers.
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W {
        EW_W { w: self }
    }
    ///Bit 19 - Buffer enable \[2\].
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
    ///Bit 20 - Write protect.
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Configuration for EMC_CSx
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [staticconfig](index.html) module
pub struct STATICCONFIG_SPEC;
impl crate::RegisterSpec for STATICCONFIG_SPEC {
    type Ux = u32;
}
///`read()` method returns [staticconfig::R](R) reader structure
impl crate::Readable for STATICCONFIG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [staticconfig::W](W) writer structure
impl crate::Writable for STATICCONFIG_SPEC {
    type Writer = W;
}
///`reset()` method sets STATICCONFIG to value 0
impl crate::Resettable for STATICCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
