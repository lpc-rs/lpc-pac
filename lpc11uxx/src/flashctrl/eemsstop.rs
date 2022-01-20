#[doc = "Register `EEMSSTOP` reader"]
pub struct R(crate::R<EEMSSTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEMSSTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEMSSTOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEMSSTOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEMSSTOP` writer"]
pub struct W(crate::W<EEMSSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEMSSTOP_SPEC>;
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
impl From<crate::W<EEMSSTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEMSSTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOPA` reader - BIST stop address: Bit 0 is fixed zero since only even addresses are allowed."]
pub struct STOPA_R(crate::FieldReader<u16, u16>);
impl STOPA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        STOPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOPA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPA` writer - BIST stop address: Bit 0 is fixed zero since only even addresses are allowed."]
pub struct STOPA_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `DEVSEL` reader - BIST device select bit 0: the BIST signature is generated over the total memory space. Singe pages are interleaved over the EEPROM devices when multiple devices are used, the signature is generated over memory of multiple devices. 1: the BIST signature is generated only over a memory range located on a single EEPROM device. Therefore the internal address generation is done such that the address' CS bits are kept stable to select only the same device. The address' MSB and LSB bits are used to step through the memory range specified by the start and stop address fields. Note: if this bit is set the start and stop address fields must be programmed such that they both address the same EEPROM device. Therefore the address' CS bits in both the start and stop address must be the same."]
pub struct DEVSEL_R(crate::FieldReader<bool, bool>);
impl DEVSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEVSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVSEL` writer - BIST device select bit 0: the BIST signature is generated over the total memory space. Singe pages are interleaved over the EEPROM devices when multiple devices are used, the signature is generated over memory of multiple devices. 1: the BIST signature is generated only over a memory range located on a single EEPROM device. Therefore the internal address generation is done such that the address' CS bits are kept stable to select only the same device. The address' MSB and LSB bits are used to step through the memory range specified by the start and stop address fields. Note: if this bit is set the start and stop address fields must be programmed such that they both address the same EEPROM device. Therefore the address' CS bits in both the start and stop address must be the same."]
pub struct DEVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `STRTBIST` reader - BIST start bit Setting this bit will start the BIST. This bit is self-clearing."]
pub struct STRTBIST_R(crate::FieldReader<bool, bool>);
impl STRTBIST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STRTBIST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRTBIST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRTBIST` writer - BIST start bit Setting this bit will start the BIST. This bit is self-clearing."]
pub struct STRTBIST_W<'a> {
    w: &'a mut W,
}
impl<'a> STRTBIST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - BIST stop address: Bit 0 is fixed zero since only even addresses are allowed."]
    #[inline(always)]
    pub fn stopa(&self) -> STOPA_R {
        STOPA_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - BIST device select bit 0: the BIST signature is generated over the total memory space. Singe pages are interleaved over the EEPROM devices when multiple devices are used, the signature is generated over memory of multiple devices. 1: the BIST signature is generated only over a memory range located on a single EEPROM device. Therefore the internal address generation is done such that the address' CS bits are kept stable to select only the same device. The address' MSB and LSB bits are used to step through the memory range specified by the start and stop address fields. Note: if this bit is set the start and stop address fields must be programmed such that they both address the same EEPROM device. Therefore the address' CS bits in both the start and stop address must be the same."]
    #[inline(always)]
    pub fn devsel(&self) -> DEVSEL_R {
        DEVSEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - BIST start bit Setting this bit will start the BIST. This bit is self-clearing."]
    #[inline(always)]
    pub fn strtbist(&self) -> STRTBIST_R {
        STRTBIST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - BIST stop address: Bit 0 is fixed zero since only even addresses are allowed."]
    #[inline(always)]
    pub fn stopa(&mut self) -> STOPA_W {
        STOPA_W { w: self }
    }
    #[doc = "Bit 30 - BIST device select bit 0: the BIST signature is generated over the total memory space. Singe pages are interleaved over the EEPROM devices when multiple devices are used, the signature is generated over memory of multiple devices. 1: the BIST signature is generated only over a memory range located on a single EEPROM device. Therefore the internal address generation is done such that the address' CS bits are kept stable to select only the same device. The address' MSB and LSB bits are used to step through the memory range specified by the start and stop address fields. Note: if this bit is set the start and stop address fields must be programmed such that they both address the same EEPROM device. Therefore the address' CS bits in both the start and stop address must be the same."]
    #[inline(always)]
    pub fn devsel(&mut self) -> DEVSEL_W {
        DEVSEL_W { w: self }
    }
    #[doc = "Bit 31 - BIST start bit Setting this bit will start the BIST. This bit is self-clearing."]
    #[inline(always)]
    pub fn strtbist(&mut self) -> STRTBIST_W {
        STRTBIST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM BIST stop address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eemsstop](index.html) module"]
pub struct EEMSSTOP_SPEC;
impl crate::RegisterSpec for EEMSSTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eemsstop::R](R) reader structure"]
impl crate::Readable for EEMSSTOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eemsstop::W](W) writer structure"]
impl crate::Writable for EEMSSTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEMSSTOP to value 0"]
impl crate::Resettable for EEMSSTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
