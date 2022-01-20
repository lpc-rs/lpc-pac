#[doc = "Register `FIFO_STATUS` reader"]
pub struct R(crate::R<FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_STATUS` writer"]
pub struct W(crate::W<FIFO_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_STATUS_SPEC>;
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
impl From<crate::W<FIFO_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - Interrupt flag. Asserted when FIFO data reaches the level specified in the FIFOCTRL register. Writing a one to this bit clears the flag. Remark: note that the bus clock to the DMIC subsystem must be running in order for an interrupt to occur."]
pub struct INT_R(crate::FieldReader<bool, bool>);
impl INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT` writer - Interrupt flag. Asserted when FIFO data reaches the level specified in the FIFOCTRL register. Writing a one to this bit clears the flag. Remark: note that the bus clock to the DMIC subsystem must be running in order for an interrupt to occur."]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
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
#[doc = "Field `OVERRUN` reader - Overrun flag. Indicates that a FIFO overflow has occurred at some point. Writing a one to this bit clears the flag. This flag does not cause an interrupt."]
pub struct OVERRUN_R(crate::FieldReader<bool, bool>);
impl OVERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN` writer - Overrun flag. Indicates that a FIFO overflow has occurred at some point. Writing a one to this bit clears the flag. This flag does not cause an interrupt."]
pub struct OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN_W<'a> {
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
#[doc = "Field `UNDERRUN` reader - Underrun flag. Indicates that a FIFO underflow has occurred at some point. Writing a one to this bit clears the flag."]
pub struct UNDERRUN_R(crate::FieldReader<bool, bool>);
impl UNDERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERRUN` writer - Underrun flag. Indicates that a FIFO underflow has occurred at some point. Writing a one to this bit clears the flag."]
pub struct UNDERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERRUN_W<'a> {
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
    #[doc = "Bit 0 - Interrupt flag. Asserted when FIFO data reaches the level specified in the FIFOCTRL register. Writing a one to this bit clears the flag. Remark: note that the bus clock to the DMIC subsystem must be running in order for an interrupt to occur."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun flag. Indicates that a FIFO overflow has occurred at some point. Writing a one to this bit clears the flag. This flag does not cause an interrupt."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Underrun flag. Indicates that a FIFO underflow has occurred at some point. Writing a one to this bit clears the flag."]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag. Asserted when FIFO data reaches the level specified in the FIFOCTRL register. Writing a one to this bit clears the flag. Remark: note that the bus clock to the DMIC subsystem must be running in order for an interrupt to occur."]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bit 1 - Overrun flag. Indicates that a FIFO overflow has occurred at some point. Writing a one to this bit clears the flag. This flag does not cause an interrupt."]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W {
        OVERRUN_W { w: self }
    }
    #[doc = "Bit 2 - Underrun flag. Indicates that a FIFO underflow has occurred at some point. Writing a one to this bit clears the flag."]
    #[inline(always)]
    pub fn underrun(&mut self) -> UNDERRUN_W {
        UNDERRUN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_status](index.html) module"]
pub struct FIFO_STATUS_SPEC;
impl crate::RegisterSpec for FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_status::R](R) reader structure"]
impl crate::Readable for FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_status::W](W) writer structure"]
impl crate::Writable for FIFO_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_STATUS to value 0"]
impl crate::Resettable for FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
