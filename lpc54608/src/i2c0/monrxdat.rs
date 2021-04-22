#[doc = "Register `MONRXDAT` reader"]
pub struct R(crate::R<MONRXDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONRXDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MONRXDAT_SPEC>> for R {
    fn from(reader: crate::R<MONRXDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MONRXDAT` reader - Monitor function Receiver Data. This reflects every data byte that passes on the I2C pins."]
pub struct MONRXDAT_R(crate::FieldReader<u8, u8>);
impl MONRXDAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MONRXDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONRXDAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Monitor Received Start.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONSTART_A {
    #[doc = "0: No start detected. The Monitor function has not detected a Start event on the I2C bus."]
    NO_START_DETECTED = 0,
    #[doc = "1: Start detected. The Monitor function has detected a Start event on the I2C bus."]
    START_DETECTED = 1,
}
impl From<MONSTART_A> for bool {
    #[inline(always)]
    fn from(variant: MONSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONSTART` reader - Monitor Received Start."]
pub struct MONSTART_R(crate::FieldReader<bool, MONSTART_A>);
impl MONSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        MONSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONSTART_A {
        match self.bits {
            false => MONSTART_A::NO_START_DETECTED,
            true => MONSTART_A::START_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_START_DETECTED`"]
    #[inline(always)]
    pub fn is_no_start_detected(&self) -> bool {
        **self == MONSTART_A::NO_START_DETECTED
    }
    #[doc = "Checks if the value of the field is `START_DETECTED`"]
    #[inline(always)]
    pub fn is_start_detected(&self) -> bool {
        **self == MONSTART_A::START_DETECTED
    }
}
impl core::ops::Deref for MONSTART_R {
    type Target = crate::FieldReader<bool, MONSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Monitor Received Repeated Start.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRESTART_A {
    #[doc = "0: No repeated start detected. The Monitor function has not detected a Repeated Start event on the I2C bus."]
    NOT_DETECTED = 0,
    #[doc = "1: Repeated start detected. The Monitor function has detected a Repeated Start event on the I2C bus."]
    DETECTED = 1,
}
impl From<MONRESTART_A> for bool {
    #[inline(always)]
    fn from(variant: MONRESTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONRESTART` reader - Monitor Received Repeated Start."]
pub struct MONRESTART_R(crate::FieldReader<bool, MONRESTART_A>);
impl MONRESTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        MONRESTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONRESTART_A {
        match self.bits {
            false => MONRESTART_A::NOT_DETECTED,
            true => MONRESTART_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == MONRESTART_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == MONRESTART_A::DETECTED
    }
}
impl core::ops::Deref for MONRESTART_R {
    type Target = crate::FieldReader<bool, MONRESTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Monitor Received NACK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONNACK_A {
    #[doc = "0: Acknowledged. The data currently being provided by the Monitor function was acknowledged by at least one master or slave receiver."]
    ACKNOWLEDGED = 0,
    #[doc = "1: Not acknowledged. The data currently being provided by the Monitor function was not acknowledged by any receiver."]
    NOT_ACKNOWLEDGED = 1,
}
impl From<MONNACK_A> for bool {
    #[inline(always)]
    fn from(variant: MONNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONNACK` reader - Monitor Received NACK."]
pub struct MONNACK_R(crate::FieldReader<bool, MONNACK_A>);
impl MONNACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MONNACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONNACK_A {
        match self.bits {
            false => MONNACK_A::ACKNOWLEDGED,
            true => MONNACK_A::NOT_ACKNOWLEDGED,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOWLEDGED`"]
    #[inline(always)]
    pub fn is_acknowledged(&self) -> bool {
        **self == MONNACK_A::ACKNOWLEDGED
    }
    #[doc = "Checks if the value of the field is `NOT_ACKNOWLEDGED`"]
    #[inline(always)]
    pub fn is_not_acknowledged(&self) -> bool {
        **self == MONNACK_A::NOT_ACKNOWLEDGED
    }
}
impl core::ops::Deref for MONNACK_R {
    type Target = crate::FieldReader<bool, MONNACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Monitor function Receiver Data. This reflects every data byte that passes on the I2C pins."]
    #[inline(always)]
    pub fn monrxdat(&self) -> MONRXDAT_R {
        MONRXDAT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Monitor Received Start."]
    #[inline(always)]
    pub fn monstart(&self) -> MONSTART_R {
        MONSTART_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Monitor Received Repeated Start."]
    #[inline(always)]
    pub fn monrestart(&self) -> MONRESTART_R {
        MONRESTART_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Monitor Received NACK."]
    #[inline(always)]
    pub fn monnack(&self) -> MONNACK_R {
        MONNACK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "Monitor receiver data register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monrxdat](index.html) module"]
pub struct MONRXDAT_SPEC;
impl crate::RegisterSpec for MONRXDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monrxdat::R](R) reader structure"]
impl crate::Readable for MONRXDAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MONRXDAT to value 0"]
impl crate::Resettable for MONRXDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
