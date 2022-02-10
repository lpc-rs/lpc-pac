///Register `STAT` reader
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `STAT` writer
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MCINIT` reader - This bit is set when software successfully writes the Memory Command register, and is cleared by Reset or by writing a 1 to the RESET bit in this register.
pub struct MCINIT_R(crate::FieldReader<bool, bool>);
impl MCINIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCINIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCINIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MCINIT` writer - This bit is set when software successfully writes the Memory Command register, and is cleared by Reset or by writing a 1 to the RESET bit in this register.
pub struct MCINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCINIT_W<'a> {
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
///Field `CMD` reader - This bit is 1 when the Command register is written. It is cleared by a hardware reset, a write to the RESET bit in this register, or the deassertion of CS which indicates that the command has completed communication with the SPI Flash.
pub struct CMD_R(crate::FieldReader<bool, bool>);
impl CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CMD` writer - This bit is 1 when the Command register is written. It is cleared by a hardware reset, a write to the RESET bit in this register, or the deassertion of CS which indicates that the command has completed communication with the SPI Flash.
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Field `RESET` reader - Write a 1 to this bit to abort a current command or memory mode. This bit is cleared when the hardware is ready for a new command to be written to the Command register.
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RESET` writer - Write a 1 to this bit to abort a current command or memory mode. This bit is cleared when the hardware is ready for a new command to be written to the Command register.
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Field `INTRQ` reader - This bit reflects the SPIFI interrupt request. Write a 1 to this bit to clear it. This bit is set when a CMD was previously 1 and has been cleared due to the deassertion of CS.
pub struct INTRQ_R(crate::FieldReader<bool, bool>);
impl INTRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INTRQ` writer - This bit reflects the SPIFI interrupt request. Write a 1 to this bit to clear it. This bit is set when a CMD was previously 1 and has been cleared due to the deassertion of CS.
pub struct INTRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INTRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    ///Bit 0 - This bit is set when software successfully writes the Memory Command register, and is cleared by Reset or by writing a 1 to the RESET bit in this register.
    #[inline(always)]
    pub fn mcinit(&self) -> MCINIT_R {
        MCINIT_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - This bit is 1 when the Command register is written. It is cleared by a hardware reset, a write to the RESET bit in this register, or the deassertion of CS which indicates that the command has completed communication with the SPI Flash.
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - Write a 1 to this bit to abort a current command or memory mode. This bit is cleared when the hardware is ready for a new command to be written to the Command register.
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - This bit reflects the SPIFI interrupt request. Write a 1 to this bit to clear it. This bit is set when a CMD was previously 1 and has been cleared due to the deassertion of CS.
    #[inline(always)]
    pub fn intrq(&self) -> INTRQ_R {
        INTRQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - This bit is set when software successfully writes the Memory Command register, and is cleared by Reset or by writing a 1 to the RESET bit in this register.
    #[inline(always)]
    pub fn mcinit(&mut self) -> MCINIT_W {
        MCINIT_W { w: self }
    }
    ///Bit 1 - This bit is 1 when the Command register is written. It is cleared by a hardware reset, a write to the RESET bit in this register, or the deassertion of CS which indicates that the command has completed communication with the SPI Flash.
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    ///Bit 4 - Write a 1 to this bit to abort a current command or memory mode. This bit is cleared when the hardware is ready for a new command to be written to the Command register.
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    ///Bit 5 - This bit reflects the SPIFI interrupt request. Write a 1 to this bit to clear it. This bit is set when a CMD was previously 1 and has been cleared due to the deassertion of CS.
    #[inline(always)]
    pub fn intrq(&mut self) -> INTRQ_W {
        INTRQ_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SPIFI status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [stat](index.html) module
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
///`read()` method returns [stat::R](R) reader structure
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [stat::W](W) writer structure
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
///`reset()` method sets STAT to value 0x0200_0000
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}
