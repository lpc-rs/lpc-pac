#[doc = "Register `MMCTRL` reader"]
pub struct R(crate::R<MMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCTRL` writer"]
pub struct W(crate::W<MMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCTRL_SPEC>;
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
impl From<crate::W<MMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Monitor mode enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MM_ENA_A {
    #[doc = "0: Monitor mode disabled."]
    DISABLED = 0,
    #[doc = "1: The I2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I 2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    ENABLED = 1,
}
impl From<MM_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: MM_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MM_ENA` reader - Monitor mode enable."]
pub struct MM_ENA_R(crate::FieldReader<bool, MM_ENA_A>);
impl MM_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MM_ENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MM_ENA_A {
        match self.bits {
            false => MM_ENA_A::DISABLED,
            true => MM_ENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MM_ENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MM_ENA_A::ENABLED
    }
}
impl core::ops::Deref for MM_ENA_R {
    type Target = crate::FieldReader<bool, MM_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MM_ENA` writer - Monitor mode enable."]
pub struct MM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MM_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MM_ENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Monitor mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MM_ENA_A::DISABLED)
    }
    #[doc = "The I2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I 2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MM_ENA_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "SCL output enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_SCL_A {
    #[doc = "0: When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    HIGH = 0,
    #[doc = "1: When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    NORMAL = 1,
}
impl From<ENA_SCL_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_SCL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA_SCL` reader - SCL output enable."]
pub struct ENA_SCL_R(crate::FieldReader<bool, ENA_SCL_A>);
impl ENA_SCL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_SCL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_SCL_A {
        match self.bits {
            false => ENA_SCL_A::HIGH,
            true => ENA_SCL_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == ENA_SCL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == ENA_SCL_A::NORMAL
    }
}
impl core::ops::Deref for ENA_SCL_R {
    type Target = crate::FieldReader<bool, ENA_SCL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_SCL` writer - SCL output enable."]
pub struct ENA_SCL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_SCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_SCL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ENA_SCL_A::HIGH)
    }
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ENA_SCL_A::NORMAL)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Select interrupt register match.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCH_ALL_A {
    #[doc = "0: When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    MATCH = 0,
    #[doc = "1: When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    ANYADDRESS = 1,
}
impl From<MATCH_ALL_A> for bool {
    #[inline(always)]
    fn from(variant: MATCH_ALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MATCH_ALL` reader - Select interrupt register match."]
pub struct MATCH_ALL_R(crate::FieldReader<bool, MATCH_ALL_A>);
impl MATCH_ALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MATCH_ALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MATCH_ALL_A {
        match self.bits {
            false => MATCH_ALL_A::MATCH,
            true => MATCH_ALL_A::ANYADDRESS,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        **self == MATCH_ALL_A::MATCH
    }
    #[doc = "Checks if the value of the field is `ANYADDRESS`"]
    #[inline(always)]
    pub fn is_anyaddress(&self) -> bool {
        **self == MATCH_ALL_A::ANYADDRESS
    }
}
impl core::ops::Deref for MATCH_ALL_R {
    type Target = crate::FieldReader<bool, MATCH_ALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCH_ALL` writer - Select interrupt register match."]
pub struct MATCH_ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_ALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MATCH_ALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(MATCH_ALL_A::MATCH)
    }
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    #[inline(always)]
    pub fn anyaddress(self) -> &'a mut W {
        self.variant(MATCH_ALL_A::ANYADDRESS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    pub fn mm_ena(&self) -> MM_ENA_R {
        MM_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    pub fn ena_scl(&self) -> ENA_SCL_R {
        ENA_SCL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    pub fn match_all(&self) -> MATCH_ALL_R {
        MATCH_ALL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    pub fn mm_ena(&mut self) -> MM_ENA_W {
        MM_ENA_W { w: self }
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    pub fn ena_scl(&mut self) -> ENA_SCL_W {
        ENA_SCL_W { w: self }
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    pub fn match_all(&mut self) -> MATCH_ALL_W {
        MATCH_ALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Monitor mode control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctrl](index.html) module"]
pub struct MMCTRL_SPEC;
impl crate::RegisterSpec for MMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctrl::R](R) reader structure"]
impl crate::Readable for MMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmctrl::W](W) writer structure"]
impl crate::Writable for MMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCTRL to value 0"]
impl crate::Resettable for MMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
