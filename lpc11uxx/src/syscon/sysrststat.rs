#[doc = "Register `SYSRSTSTAT` reader"]
pub struct R(crate::R<SYSRSTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSRSTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSRSTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSRSTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSRSTSTAT` writer"]
pub struct W(crate::W<SYSRSTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSRSTSTAT_SPEC>;
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
impl From<crate::W<SYSRSTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSRSTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "POR reset status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_A {
    #[doc = "0: No POR detected"]
    NO_POR_DETECTED = 0,
    #[doc = "1: POR detected. Writing a one clears this reset."]
    POR_DETECTED_WRITIN = 1,
}
impl From<POR_A> for bool {
    #[inline(always)]
    fn from(variant: POR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POR` reader - POR reset status"]
pub struct POR_R(crate::FieldReader<bool, POR_A>);
impl POR_R {
    pub(crate) fn new(bits: bool) -> Self {
        POR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_A {
        match self.bits {
            false => POR_A::NO_POR_DETECTED,
            true => POR_A::POR_DETECTED_WRITIN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_POR_DETECTED`"]
    #[inline(always)]
    pub fn is_no_por_detected(&self) -> bool {
        **self == POR_A::NO_POR_DETECTED
    }
    #[doc = "Checks if the value of the field is `POR_DETECTED_WRITIN`"]
    #[inline(always)]
    pub fn is_por_detected_writin(&self) -> bool {
        **self == POR_A::POR_DETECTED_WRITIN
    }
}
impl core::ops::Deref for POR_R {
    type Target = crate::FieldReader<bool, POR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR` writer - POR reset status"]
pub struct POR_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No POR detected"]
    #[inline(always)]
    pub fn no_por_detected(self) -> &'a mut W {
        self.variant(POR_A::NO_POR_DETECTED)
    }
    #[doc = "POR detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn por_detected_writin(self) -> &'a mut W {
        self.variant(POR_A::POR_DETECTED_WRITIN)
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
#[doc = "External reset status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRST_A {
    #[doc = "0: No reset event detected."]
    NO_RESET_EVENT_DETEC = 0,
    #[doc = "1: Reset detected. Writing a one clears this reset."]
    RESET_DETECTED_WRIT = 1,
}
impl From<EXTRST_A> for bool {
    #[inline(always)]
    fn from(variant: EXTRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTRST` reader - External reset status"]
pub struct EXTRST_R(crate::FieldReader<bool, EXTRST_A>);
impl EXTRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTRST_A {
        match self.bits {
            false => EXTRST_A::NO_RESET_EVENT_DETEC,
            true => EXTRST_A::RESET_DETECTED_WRIT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET_EVENT_DETEC`"]
    #[inline(always)]
    pub fn is_no_reset_event_detec(&self) -> bool {
        **self == EXTRST_A::NO_RESET_EVENT_DETEC
    }
    #[doc = "Checks if the value of the field is `RESET_DETECTED_WRIT`"]
    #[inline(always)]
    pub fn is_reset_detected_writ(&self) -> bool {
        **self == EXTRST_A::RESET_DETECTED_WRIT
    }
}
impl core::ops::Deref for EXTRST_R {
    type Target = crate::FieldReader<bool, EXTRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTRST` writer - External reset status"]
pub struct EXTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset event detected."]
    #[inline(always)]
    pub fn no_reset_event_detec(self) -> &'a mut W {
        self.variant(EXTRST_A::NO_RESET_EVENT_DETEC)
    }
    #[doc = "Reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn reset_detected_writ(self) -> &'a mut W {
        self.variant(EXTRST_A::RESET_DETECTED_WRIT)
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
#[doc = "Status of the Watchdog reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: No WDT reset detected"]
    NO_RESET = 0,
    #[doc = "1: WDT reset detected. Writing a one clears this reset."]
    RESET_CLEAR = 1,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` reader - Status of the Watchdog reset"]
pub struct WDT_R(crate::FieldReader<bool, WDT_A>);
impl WDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::NO_RESET,
            true => WDT_A::RESET_CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == WDT_A::NO_RESET
    }
    #[doc = "Checks if the value of the field is `RESET_CLEAR`"]
    #[inline(always)]
    pub fn is_reset_clear(&self) -> bool {
        **self == WDT_A::RESET_CLEAR
    }
}
impl core::ops::Deref for WDT_R {
    type Target = crate::FieldReader<bool, WDT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT` writer - Status of the Watchdog reset"]
pub struct WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No WDT reset detected"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(WDT_A::NO_RESET)
    }
    #[doc = "WDT reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn reset_clear(self) -> &'a mut W {
        self.variant(WDT_A::RESET_CLEAR)
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
#[doc = "Status of the Brown-out detect reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_A {
    #[doc = "0: No BOD reset detected"]
    NO_RESET = 0,
    #[doc = "1: BOD reset detected. Writing a one clears this reset."]
    RESET_CLEAR = 1,
}
impl From<BOD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOD` reader - Status of the Brown-out detect reset"]
pub struct BOD_R(crate::FieldReader<bool, BOD_A>);
impl BOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_A {
        match self.bits {
            false => BOD_A::NO_RESET,
            true => BOD_A::RESET_CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == BOD_A::NO_RESET
    }
    #[doc = "Checks if the value of the field is `RESET_CLEAR`"]
    #[inline(always)]
    pub fn is_reset_clear(&self) -> bool {
        **self == BOD_A::RESET_CLEAR
    }
}
impl core::ops::Deref for BOD_R {
    type Target = crate::FieldReader<bool, BOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD` writer - Status of the Brown-out detect reset"]
pub struct BOD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No BOD reset detected"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(BOD_A::NO_RESET)
    }
    #[doc = "BOD reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn reset_clear(self) -> &'a mut W {
        self.variant(BOD_A::RESET_CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Status of the software system reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRST_A {
    #[doc = "0: No System reset detected"]
    NO_SYSTEM_RESET_DETE = 0,
    #[doc = "1: System reset detected. Writing a one clears this reset."]
    SYSTEM_RESET_DETECTE = 1,
}
impl From<SYSRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRST` reader - Status of the software system reset"]
pub struct SYSRST_R(crate::FieldReader<bool, SYSRST_A>);
impl SYSRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRST_A {
        match self.bits {
            false => SYSRST_A::NO_SYSTEM_RESET_DETE,
            true => SYSRST_A::SYSTEM_RESET_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYSTEM_RESET_DETE`"]
    #[inline(always)]
    pub fn is_no_system_reset_dete(&self) -> bool {
        **self == SYSRST_A::NO_SYSTEM_RESET_DETE
    }
    #[doc = "Checks if the value of the field is `SYSTEM_RESET_DETECTE`"]
    #[inline(always)]
    pub fn is_system_reset_detecte(&self) -> bool {
        **self == SYSRST_A::SYSTEM_RESET_DETECTE
    }
}
impl core::ops::Deref for SYSRST_R {
    type Target = crate::FieldReader<bool, SYSRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRST` writer - Status of the software system reset"]
pub struct SYSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No System reset detected"]
    #[inline(always)]
    pub fn no_system_reset_dete(self) -> &'a mut W {
        self.variant(SYSRST_A::NO_SYSTEM_RESET_DETE)
    }
    #[doc = "System reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn system_reset_detecte(self) -> &'a mut W {
        self.variant(SYSRST_A::SYSTEM_RESET_DETECTE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - POR reset status"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External reset status"]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of the Watchdog reset"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset"]
    #[inline(always)]
    pub fn bod(&self) -> BOD_R {
        BOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of the software system reset"]
    #[inline(always)]
    pub fn sysrst(&self) -> SYSRST_R {
        SYSRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - POR reset status"]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W {
        POR_W { w: self }
    }
    #[doc = "Bit 1 - External reset status"]
    #[inline(always)]
    pub fn extrst(&mut self) -> EXTRST_W {
        EXTRST_W { w: self }
    }
    #[doc = "Bit 2 - Status of the Watchdog reset"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W {
        WDT_W { w: self }
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset"]
    #[inline(always)]
    pub fn bod(&mut self) -> BOD_W {
        BOD_W { w: self }
    }
    #[doc = "Bit 4 - Status of the software system reset"]
    #[inline(always)]
    pub fn sysrst(&mut self) -> SYSRST_W {
        SYSRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System reset status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysrststat](index.html) module"]
pub struct SYSRSTSTAT_SPEC;
impl crate::RegisterSpec for SYSRSTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysrststat::R](R) reader structure"]
impl crate::Readable for SYSRSTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysrststat::W](W) writer structure"]
impl crate::Writable for SYSRSTSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSRSTSTAT to value 0x03"]
impl crate::Resettable for SYSRSTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
