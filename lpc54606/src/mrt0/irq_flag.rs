#[doc = "Register `IRQ_FLAG` reader"]
pub struct R(crate::R<IRQ_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IRQ_FLAG_SPEC>> for R {
    fn from(reader: crate::R<IRQ_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_FLAG` writer"]
pub struct W(crate::W<IRQ_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_FLAG_SPEC>;
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
impl core::convert::From<crate::W<IRQ_FLAG_SPEC>> for W {
    fn from(writer: crate::W<IRQ_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Monitors the interrupt flag of TIMER0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG0_A {
    #[doc = "0: No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT = 0,
    #[doc = "1: Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT = 1,
}
impl From<GFLAG0_A> for bool {
    #[inline(always)]
    fn from(variant: GFLAG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFLAG0` reader - Monitors the interrupt flag of TIMER0."]
pub struct GFLAG0_R(crate::FieldReader<bool, GFLAG0_A>);
impl GFLAG0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFLAG0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GFLAG0_A {
        match self.bits {
            false => GFLAG0_A::NO_PENDING_INTERRUPT,
            true => GFLAG0_A::PENDING_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        **self == GFLAG0_A::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_pending_interrupt(&self) -> bool {
        **self == GFLAG0_A::PENDING_INTERRUPT
    }
}
impl core::ops::Deref for GFLAG0_R {
    type Target = crate::FieldReader<bool, GFLAG0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFLAG0` writer - Monitors the interrupt flag of TIMER0."]
pub struct GFLAG0_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLAG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GFLAG0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline(always)]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(GFLAG0_A::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline(always)]
    pub fn pending_interrupt(self) -> &'a mut W {
        self.variant(GFLAG0_A::PENDING_INTERRUPT)
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
#[doc = "Field `GFLAG1` reader - Monitors the interrupt flag of TIMER1. See description of channel 0."]
pub struct GFLAG1_R(crate::FieldReader<bool, bool>);
impl GFLAG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFLAG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFLAG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFLAG1` writer - Monitors the interrupt flag of TIMER1. See description of channel 0."]
pub struct GFLAG1_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLAG1_W<'a> {
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
#[doc = "Field `GFLAG2` reader - Monitors the interrupt flag of TIMER2. See description of channel 0."]
pub struct GFLAG2_R(crate::FieldReader<bool, bool>);
impl GFLAG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFLAG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFLAG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFLAG2` writer - Monitors the interrupt flag of TIMER2. See description of channel 0."]
pub struct GFLAG2_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLAG2_W<'a> {
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
#[doc = "Field `GFLAG3` reader - Monitors the interrupt flag of TIMER3. See description of channel 0."]
pub struct GFLAG3_R(crate::FieldReader<bool, bool>);
impl GFLAG3_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFLAG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFLAG3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFLAG3` writer - Monitors the interrupt flag of TIMER3. See description of channel 0."]
pub struct GFLAG3_W<'a> {
    w: &'a mut W,
}
impl<'a> GFLAG3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Monitors the interrupt flag of TIMER0."]
    #[inline(always)]
    pub fn gflag0(&self) -> GFLAG0_R {
        GFLAG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[inline(always)]
    pub fn gflag1(&self) -> GFLAG1_R {
        GFLAG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[inline(always)]
    pub fn gflag2(&self) -> GFLAG2_R {
        GFLAG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[inline(always)]
    pub fn gflag3(&self) -> GFLAG3_R {
        GFLAG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Monitors the interrupt flag of TIMER0."]
    #[inline(always)]
    pub fn gflag0(&mut self) -> GFLAG0_W {
        GFLAG0_W { w: self }
    }
    #[doc = "Bit 1 - Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[inline(always)]
    pub fn gflag1(&mut self) -> GFLAG1_W {
        GFLAG1_W { w: self }
    }
    #[doc = "Bit 2 - Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[inline(always)]
    pub fn gflag2(&mut self) -> GFLAG2_W {
        GFLAG2_W { w: self }
    }
    #[doc = "Bit 3 - Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[inline(always)]
    pub fn gflag3(&mut self) -> GFLAG3_W {
        GFLAG3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_flag](index.html) module"]
pub struct IRQ_FLAG_SPEC;
impl crate::RegisterSpec for IRQ_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_flag::R](R) reader structure"]
impl crate::Readable for IRQ_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_flag::W](W) writer structure"]
impl crate::Writable for IRQ_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_FLAG to value 0"]
impl crate::Resettable for IRQ_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
