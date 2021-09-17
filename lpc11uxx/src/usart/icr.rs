#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IrDA mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDAEN_A {
    #[doc = "0: IrDA mode is disabled, USARTn acts as a standard USART."]
    DISABLED = 0,
    #[doc = "1: IrDA mode is enabled."]
    ENABLED = 1,
}
impl From<IRDAEN_A> for bool {
    #[inline(always)]
    fn from(variant: IRDAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDAEN` reader - IrDA mode enable"]
pub struct IRDAEN_R(crate::FieldReader<bool, IRDAEN_A>);
impl IRDAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRDAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRDAEN_A {
        match self.bits {
            false => IRDAEN_A::DISABLED,
            true => IRDAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IRDAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IRDAEN_A::ENABLED
    }
}
impl core::ops::Deref for IRDAEN_R {
    type Target = crate::FieldReader<bool, IRDAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRDAEN` writer - IrDA mode enable"]
pub struct IRDAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRDAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IrDA mode is disabled, USARTn acts as a standard USART."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IRDAEN_A::DISABLED)
    }
    #[doc = "IrDA mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IRDAEN_A::ENABLED)
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
#[doc = "Serial input inverter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRDAINV_A {
    #[doc = "0: The serial input is not inverted."]
    INVERTED = 0,
    #[doc = "1: The serial input is inverted. This has no effect on the serial output."]
    NOT_INVERTED = 1,
}
impl From<IRDAINV_A> for bool {
    #[inline(always)]
    fn from(variant: IRDAINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDAINV` reader - Serial input inverter"]
pub struct IRDAINV_R(crate::FieldReader<bool, IRDAINV_A>);
impl IRDAINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRDAINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRDAINV_A {
        match self.bits {
            false => IRDAINV_A::INVERTED,
            true => IRDAINV_A::NOT_INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == IRDAINV_A::INVERTED
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == IRDAINV_A::NOT_INVERTED
    }
}
impl core::ops::Deref for IRDAINV_R {
    type Target = crate::FieldReader<bool, IRDAINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRDAINV` writer - Serial input inverter"]
pub struct IRDAINV_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDAINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRDAINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The serial input is not inverted."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(IRDAINV_A::INVERTED)
    }
    #[doc = "The serial input is inverted. This has no effect on the serial output."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(IRDAINV_A::NOT_INVERTED)
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
#[doc = "IrDA fixed pulse width mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXPULSEEN_A {
    #[doc = "0: IrDA fixed pulse width mode disabled."]
    DISABLED = 0,
    #[doc = "1: IrDA fixed pulse width mode enabled."]
    ENABLED = 1,
}
impl From<FIXPULSEEN_A> for bool {
    #[inline(always)]
    fn from(variant: FIXPULSEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIXPULSEEN` reader - IrDA fixed pulse width mode."]
pub struct FIXPULSEEN_R(crate::FieldReader<bool, FIXPULSEEN_A>);
impl FIXPULSEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIXPULSEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXPULSEEN_A {
        match self.bits {
            false => FIXPULSEEN_A::DISABLED,
            true => FIXPULSEEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FIXPULSEEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FIXPULSEEN_A::ENABLED
    }
}
impl core::ops::Deref for FIXPULSEEN_R {
    type Target = crate::FieldReader<bool, FIXPULSEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIXPULSEEN` writer - IrDA fixed pulse width mode."]
pub struct FIXPULSEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXPULSEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIXPULSEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IrDA fixed pulse width mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIXPULSEEN_A::DISABLED)
    }
    #[doc = "IrDA fixed pulse width mode enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIXPULSEEN_A::ENABLED)
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
#[doc = "Configures the pulse width when FixPulseEn = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PULSEDIV_A {
    #[doc = "0: 3 / (16 x baud rate)"]
    _3_DIV_16_X_BAUD_RATE = 0,
    #[doc = "1: 2 x TPCLK"]
    _2_X_TPCLK = 1,
    #[doc = "2: 4 x TPCLK"]
    _4_X_TPCLK = 2,
    #[doc = "3: 8 x TPCLK"]
    _8_X_TPCLK = 3,
    #[doc = "4: 16 x TPCLK"]
    _16_X_TPCLK = 4,
    #[doc = "5: 32 x TPCLK"]
    _32_X_TPCLK = 5,
    #[doc = "6: 64 x TPCLK"]
    _64_X_TPCLK = 6,
    #[doc = "7: 128 x TPCLK"]
    _128_X_TPCLK = 7,
}
impl From<PULSEDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PULSEDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PULSEDIV` reader - Configures the pulse width when FixPulseEn = 1."]
pub struct PULSEDIV_R(crate::FieldReader<u8, PULSEDIV_A>);
impl PULSEDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PULSEDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULSEDIV_A {
        match self.bits {
            0 => PULSEDIV_A::_3_DIV_16_X_BAUD_RATE,
            1 => PULSEDIV_A::_2_X_TPCLK,
            2 => PULSEDIV_A::_4_X_TPCLK,
            3 => PULSEDIV_A::_8_X_TPCLK,
            4 => PULSEDIV_A::_16_X_TPCLK,
            5 => PULSEDIV_A::_32_X_TPCLK,
            6 => PULSEDIV_A::_64_X_TPCLK,
            7 => PULSEDIV_A::_128_X_TPCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_3_DIV_16_X_BAUD_RATE`"]
    #[inline(always)]
    pub fn is_3_div_16_x_baud_rate(&self) -> bool {
        **self == PULSEDIV_A::_3_DIV_16_X_BAUD_RATE
    }
    #[doc = "Checks if the value of the field is `_2_X_TPCLK`"]
    #[inline(always)]
    pub fn is_2_x_tpclk(&self) -> bool {
        **self == PULSEDIV_A::_2_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_4_X_TPCLK`"]
    #[inline(always)]
    pub fn is_4_x_tpclk(&self) -> bool {
        **self == PULSEDIV_A::_4_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_8_X_TPCLK`"]
    #[inline(always)]
    pub fn is_8_x_tpclk(&self) -> bool {
        **self == PULSEDIV_A::_8_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_16_X_TPCLK`"]
    #[inline(always)]
    pub fn is_16_x_tpclk(&self) -> bool {
        **self == PULSEDIV_A::_16_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_32_X_TPCLK`"]
    #[inline(always)]
    pub fn is_32_x_tpclk(&self) -> bool {
        **self == PULSEDIV_A::_32_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_64_X_TPCLK`"]
    #[inline(always)]
    pub fn is_64_x_tpclk(&self) -> bool {
        **self == PULSEDIV_A::_64_X_TPCLK
    }
    #[doc = "Checks if the value of the field is `_128_X_TPCLK`"]
    #[inline(always)]
    pub fn is_128_x_tpclk(&self) -> bool {
        **self == PULSEDIV_A::_128_X_TPCLK
    }
}
impl core::ops::Deref for PULSEDIV_R {
    type Target = crate::FieldReader<u8, PULSEDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULSEDIV` writer - Configures the pulse width when FixPulseEn = 1."]
pub struct PULSEDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSEDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PULSEDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "3 / (16 x baud rate)"]
    #[inline(always)]
    pub fn _3_div_16_x_baud_rate(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_3_DIV_16_X_BAUD_RATE)
    }
    #[doc = "2 x TPCLK"]
    #[inline(always)]
    pub fn _2_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_2_X_TPCLK)
    }
    #[doc = "4 x TPCLK"]
    #[inline(always)]
    pub fn _4_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_4_X_TPCLK)
    }
    #[doc = "8 x TPCLK"]
    #[inline(always)]
    pub fn _8_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_8_X_TPCLK)
    }
    #[doc = "16 x TPCLK"]
    #[inline(always)]
    pub fn _16_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_16_X_TPCLK)
    }
    #[doc = "32 x TPCLK"]
    #[inline(always)]
    pub fn _32_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_32_X_TPCLK)
    }
    #[doc = "64 x TPCLK"]
    #[inline(always)]
    pub fn _64_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_64_X_TPCLK)
    }
    #[doc = "128 x TPCLK"]
    #[inline(always)]
    pub fn _128_x_tpclk(self) -> &'a mut W {
        self.variant(PULSEDIV_A::_128_X_TPCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IrDA mode enable"]
    #[inline(always)]
    pub fn irdaen(&self) -> IRDAEN_R {
        IRDAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Serial input inverter"]
    #[inline(always)]
    pub fn irdainv(&self) -> IRDAINV_R {
        IRDAINV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IrDA fixed pulse width mode."]
    #[inline(always)]
    pub fn fixpulseen(&self) -> FIXPULSEEN_R {
        FIXPULSEEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Configures the pulse width when FixPulseEn = 1."]
    #[inline(always)]
    pub fn pulsediv(&self) -> PULSEDIV_R {
        PULSEDIV_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA mode enable"]
    #[inline(always)]
    pub fn irdaen(&mut self) -> IRDAEN_W {
        IRDAEN_W { w: self }
    }
    #[doc = "Bit 1 - Serial input inverter"]
    #[inline(always)]
    pub fn irdainv(&mut self) -> IRDAINV_W {
        IRDAINV_W { w: self }
    }
    #[doc = "Bit 2 - IrDA fixed pulse width mode."]
    #[inline(always)]
    pub fn fixpulseen(&mut self) -> FIXPULSEEN_W {
        FIXPULSEEN_W { w: self }
    }
    #[doc = "Bits 3:5 - Configures the pulse width when FixPulseEn = 1."]
    #[inline(always)]
    pub fn pulsediv(&mut self) -> PULSEDIV_W {
        PULSEDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IrDA Control Register. Enables and configures the IrDA (remote control) mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
