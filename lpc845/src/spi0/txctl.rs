#[doc = "Register `TXCTL` reader"]
pub struct R(crate::R<TXCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCTL` writer"]
pub struct W(crate::W<TXCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCTL_SPEC>;
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
impl From<crate::W<TXCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXSSEL0_N` reader - Transmit Slave Select 0."]
pub struct TXSSEL0_N_R(crate::FieldReader<bool, bool>);
impl TXSSEL0_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSSEL0_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSSEL0_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSSEL0_N` writer - Transmit Slave Select 0."]
pub struct TXSSEL0_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL0_N_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TXSSEL1_N` reader - Transmit Slave Select 1."]
pub struct TXSSEL1_N_R(crate::FieldReader<bool, bool>);
impl TXSSEL1_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSSEL1_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSSEL1_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSSEL1_N` writer - Transmit Slave Select 1."]
pub struct TXSSEL1_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL1_N_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `TXSSEL2_N` reader - Transmit Slave Select 2."]
pub struct TXSSEL2_N_R(crate::FieldReader<bool, bool>);
impl TXSSEL2_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSSEL2_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSSEL2_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSSEL2_N` writer - Transmit Slave Select 2."]
pub struct TXSSEL2_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL2_N_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `TXSSEL3_N` reader - Transmit Slave Select 3."]
pub struct TXSSEL3_N_R(crate::FieldReader<bool, bool>);
impl TXSSEL3_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSSEL3_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSSEL3_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSSEL3_N` writer - Transmit Slave Select 3."]
pub struct TXSSEL3_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL3_N_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `EOT` reader - End of Transfer."]
pub struct EOT_R(crate::FieldReader<bool, bool>);
impl EOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOT` writer - End of Transfer."]
pub struct EOT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `EOF` reader - End of Frame."]
pub struct EOF_R(crate::FieldReader<bool, bool>);
impl EOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOF` writer - End of Frame."]
pub struct EOF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `RXIGNORE` reader - Receive Ignore."]
pub struct RXIGNORE_R(crate::FieldReader<bool, bool>);
impl RXIGNORE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIGNORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIGNORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIGNORE` writer - Receive Ignore."]
pub struct RXIGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIGNORE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `LEN` reader - Data transfer Length."]
pub struct LEN_R(crate::FieldReader<u8, u8>);
impl LEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEN` writer - Data transfer Length."]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Transmit Slave Select 0."]
    #[inline(always)]
    pub fn txssel0_n(&self) -> TXSSEL0_N_R {
        TXSSEL0_N_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit Slave Select 1."]
    #[inline(always)]
    pub fn txssel1_n(&self) -> TXSSEL1_N_R {
        TXSSEL1_N_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit Slave Select 2."]
    #[inline(always)]
    pub fn txssel2_n(&self) -> TXSSEL2_N_R {
        TXSSEL2_N_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmit Slave Select 3."]
    #[inline(always)]
    pub fn txssel3_n(&self) -> TXSSEL3_N_R {
        TXSSEL3_N_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - End of Transfer."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - End of Frame."]
    #[inline(always)]
    pub fn eof(&self) -> EOF_R {
        EOF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Receive Ignore."]
    #[inline(always)]
    pub fn rxignore(&self) -> RXIGNORE_R {
        RXIGNORE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Data transfer Length."]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Transmit Slave Select 0."]
    #[inline(always)]
    pub fn txssel0_n(&mut self) -> TXSSEL0_N_W {
        TXSSEL0_N_W { w: self }
    }
    #[doc = "Bit 17 - Transmit Slave Select 1."]
    #[inline(always)]
    pub fn txssel1_n(&mut self) -> TXSSEL1_N_W {
        TXSSEL1_N_W { w: self }
    }
    #[doc = "Bit 18 - Transmit Slave Select 2."]
    #[inline(always)]
    pub fn txssel2_n(&mut self) -> TXSSEL2_N_W {
        TXSSEL2_N_W { w: self }
    }
    #[doc = "Bit 19 - Transmit Slave Select 3."]
    #[inline(always)]
    pub fn txssel3_n(&mut self) -> TXSSEL3_N_W {
        TXSSEL3_N_W { w: self }
    }
    #[doc = "Bit 20 - End of Transfer."]
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W {
        EOT_W { w: self }
    }
    #[doc = "Bit 21 - End of Frame."]
    #[inline(always)]
    pub fn eof(&mut self) -> EOF_W {
        EOF_W { w: self }
    }
    #[doc = "Bit 22 - Receive Ignore."]
    #[inline(always)]
    pub fn rxignore(&mut self) -> RXIGNORE_W {
        RXIGNORE_W { w: self }
    }
    #[doc = "Bits 24:27 - Data transfer Length."]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txctl](index.html) module"]
pub struct TXCTL_SPEC;
impl crate::RegisterSpec for TXCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txctl::R](R) reader structure"]
impl crate::Readable for TXCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txctl::W](W) writer structure"]
impl crate::Writable for TXCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCTL to value 0"]
impl crate::Resettable for TXCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
