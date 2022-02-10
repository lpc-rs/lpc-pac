///Register `MCR` reader
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MCR` writer
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MR0I` reader - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC.
pub struct MR0I_R(crate::FieldReader<bool, bool>);
impl MR0I_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR0I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR0I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR0I` writer - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC.
pub struct MR0I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0I_W<'a> {
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
///Field `MR0R` reader - Reset on MR0: the TC will be reset if MR0 matches it.
pub struct MR0R_R(crate::FieldReader<bool, bool>);
impl MR0R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR0R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR0R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR0R` writer - Reset on MR0: the TC will be reset if MR0 matches it.
pub struct MR0R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0R_W<'a> {
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
///Field `MR0S` reader - Stop on MR0: the TC and PC will be stopped and TCR\[0\]
///will be set to 0 if MR0 matches the TC.
pub struct MR0S_R(crate::FieldReader<bool, bool>);
impl MR0S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR0S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR0S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR0S` writer - Stop on MR0: the TC and PC will be stopped and TCR\[0\]
///will be set to 0 if MR0 matches the TC.
pub struct MR0S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Field `MR1I` reader - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC.
pub struct MR1I_R(crate::FieldReader<bool, bool>);
impl MR1I_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR1I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR1I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR1I` writer - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC.
pub struct MR1I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1I_W<'a> {
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
///Field `MR1R` reader - Reset on MR1: the TC will be reset if MR1 matches it.
pub struct MR1R_R(crate::FieldReader<bool, bool>);
impl MR1R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR1R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR1R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR1R` writer - Reset on MR1: the TC will be reset if MR1 matches it.
pub struct MR1R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1R_W<'a> {
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
///Field `MR1S` reader - Stop on MR1: the TC and PC will be stopped and TCR\[0\]
///will be set to 0 if MR1 matches the TC.
pub struct MR1S_R(crate::FieldReader<bool, bool>);
impl MR1S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR1S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR1S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR1S` writer - Stop on MR1: the TC and PC will be stopped and TCR\[0\]
///will be set to 0 if MR1 matches the TC.
pub struct MR1S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1S_W<'a> {
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
///Field `MR2I` reader - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC.
pub struct MR2I_R(crate::FieldReader<bool, bool>);
impl MR2I_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR2I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR2I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR2I` writer - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC.
pub struct MR2I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2I_W<'a> {
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
///Field `MR2R` reader - Reset on MR2: the TC will be reset if MR2 matches it.
pub struct MR2R_R(crate::FieldReader<bool, bool>);
impl MR2R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR2R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR2R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR2R` writer - Reset on MR2: the TC will be reset if MR2 matches it.
pub struct MR2R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2R_W<'a> {
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
///Field `MR2S` reader - Stop on MR2: the TC and PC will be stopped and TCR\[0\]
///will be set to 0 if MR2 matches the TC.
pub struct MR2S_R(crate::FieldReader<bool, bool>);
impl MR2S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR2S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR2S` writer - Stop on MR2: the TC and PC will be stopped and TCR\[0\]
///will be set to 0 if MR2 matches the TC.
pub struct MR2S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2S_W<'a> {
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
///Field `MR3I` reader - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC.
pub struct MR3I_R(crate::FieldReader<bool, bool>);
impl MR3I_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR3I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR3I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR3I` writer - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC.
pub struct MR3I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Field `MR3R` reader - Reset on MR3: the TC will be reset if MR3 matches it.
pub struct MR3R_R(crate::FieldReader<bool, bool>);
impl MR3R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR3R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR3R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR3R` writer - Reset on MR3: the TC will be reset if MR3 matches it.
pub struct MR3R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3R_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///Field `MR3S` reader - Stop on MR3: the TC and PC will be stopped and TCR\[0\]
///will be set to 0 if MR3 matches the TC.
pub struct MR3S_R(crate::FieldReader<bool, bool>);
impl MR3S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR3S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR3S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR3S` writer - Stop on MR3: the TC and PC will be stopped and TCR\[0\]
///will be set to 0 if MR3 matches the TC.
pub struct MR3S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Field `MR0RL` reader - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
pub struct MR0RL_R(crate::FieldReader<bool, bool>);
impl MR0RL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR0RL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR0RL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR0RL` writer - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
pub struct MR0RL_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0RL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
///Field `MR1RL` reader - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
pub struct MR1RL_R(crate::FieldReader<bool, bool>);
impl MR1RL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR1RL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR1RL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR1RL` writer - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
pub struct MR1RL_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1RL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
///Field `MR2RL` reader - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
pub struct MR2RL_R(crate::FieldReader<bool, bool>);
impl MR2RL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR2RL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR2RL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR2RL` writer - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
pub struct MR2RL_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2RL_W<'a> {
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
///Field `MR3RL` reader - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
pub struct MR3RL_R(crate::FieldReader<bool, bool>);
impl MR3RL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MR3RL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR3RL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MR3RL` writer - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
pub struct MR3RL_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3RL_W<'a> {
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
    ///Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC.
    #[inline(always)]
    pub fn mr0i(&self) -> MR0I_R {
        MR0I_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it.
    #[inline(always)]
    pub fn mr0r(&self) -> MR0R_R {
        MR0R_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\[0\]
    ///will be set to 0 if MR0 matches the TC.
    #[inline(always)]
    pub fn mr0s(&self) -> MR0S_R {
        MR0S_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC.
    #[inline(always)]
    pub fn mr1i(&self) -> MR1I_R {
        MR1I_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it.
    #[inline(always)]
    pub fn mr1r(&self) -> MR1R_R {
        MR1R_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\[0\]
    ///will be set to 0 if MR1 matches the TC.
    #[inline(always)]
    pub fn mr1s(&self) -> MR1S_R {
        MR1S_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC.
    #[inline(always)]
    pub fn mr2i(&self) -> MR2I_R {
        MR2I_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it.
    #[inline(always)]
    pub fn mr2r(&self) -> MR2R_R {
        MR2R_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\[0\]
    ///will be set to 0 if MR2 matches the TC.
    #[inline(always)]
    pub fn mr2s(&self) -> MR2S_R {
        MR2S_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC.
    #[inline(always)]
    pub fn mr3i(&self) -> MR3I_R {
        MR3I_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it.
    #[inline(always)]
    pub fn mr3r(&self) -> MR3R_R {
        MR3R_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\[0\]
    ///will be set to 0 if MR3 matches the TC.
    #[inline(always)]
    pub fn mr3s(&self) -> MR3S_R {
        MR3S_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 24 - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
    #[inline(always)]
    pub fn mr0rl(&self) -> MR0RL_R {
        MR0RL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
    #[inline(always)]
    pub fn mr1rl(&self) -> MR1RL_R {
        MR1RL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
    #[inline(always)]
    pub fn mr2rl(&self) -> MR2RL_R {
        MR2RL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
    #[inline(always)]
    pub fn mr3rl(&self) -> MR3RL_R {
        MR3RL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC.
    #[inline(always)]
    pub fn mr0i(&mut self) -> MR0I_W {
        MR0I_W { w: self }
    }
    ///Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it.
    #[inline(always)]
    pub fn mr0r(&mut self) -> MR0R_W {
        MR0R_W { w: self }
    }
    ///Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\[0\]
    ///will be set to 0 if MR0 matches the TC.
    #[inline(always)]
    pub fn mr0s(&mut self) -> MR0S_W {
        MR0S_W { w: self }
    }
    ///Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC.
    #[inline(always)]
    pub fn mr1i(&mut self) -> MR1I_W {
        MR1I_W { w: self }
    }
    ///Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it.
    #[inline(always)]
    pub fn mr1r(&mut self) -> MR1R_W {
        MR1R_W { w: self }
    }
    ///Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\[0\]
    ///will be set to 0 if MR1 matches the TC.
    #[inline(always)]
    pub fn mr1s(&mut self) -> MR1S_W {
        MR1S_W { w: self }
    }
    ///Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC.
    #[inline(always)]
    pub fn mr2i(&mut self) -> MR2I_W {
        MR2I_W { w: self }
    }
    ///Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it.
    #[inline(always)]
    pub fn mr2r(&mut self) -> MR2R_W {
        MR2R_W { w: self }
    }
    ///Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\[0\]
    ///will be set to 0 if MR2 matches the TC.
    #[inline(always)]
    pub fn mr2s(&mut self) -> MR2S_W {
        MR2S_W { w: self }
    }
    ///Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC.
    #[inline(always)]
    pub fn mr3i(&mut self) -> MR3I_W {
        MR3I_W { w: self }
    }
    ///Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it.
    #[inline(always)]
    pub fn mr3r(&mut self) -> MR3R_W {
        MR3R_W { w: self }
    }
    ///Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\[0\]
    ///will be set to 0 if MR3 matches the TC.
    #[inline(always)]
    pub fn mr3s(&mut self) -> MR3S_W {
        MR3S_W { w: self }
    }
    ///Bit 24 - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
    #[inline(always)]
    pub fn mr0rl(&mut self) -> MR0RL_W {
        MR0RL_W { w: self }
    }
    ///Bit 25 - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
    #[inline(always)]
    pub fn mr1rl(&mut self) -> MR1RL_W {
        MR1RL_W { w: self }
    }
    ///Bit 26 - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
    #[inline(always)]
    pub fn mr2rl(&mut self) -> MR2RL_W {
        MR2RL_W { w: self }
    }
    ///Bit 27 - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR).
    #[inline(always)]
    pub fn mr3rl(&mut self) -> MR3RL_W {
        MR3RL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Match Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mcr](index.html) module
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mcr::R](R) reader structure
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mcr::W](W) writer structure
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
