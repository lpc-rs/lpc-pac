#[doc = "Register `MAC_ADDR_LOW` reader"]
pub struct R(crate::R<MAC_ADDR_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MAC_ADDR_LOW_SPEC>> for R {
    fn from(reader: crate::R<MAC_ADDR_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR_LOW` writer"]
pub struct W(crate::W<MAC_ADDR_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR_LOW_SPEC>;
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
impl core::convert::From<crate::W<MAC_ADDR_LOW_SPEC>> for W {
    fn from(writer: crate::W<MAC_ADDR_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A31_0` reader - MAC Address0 \\[31:0\\]
This field contains the lower 32 bits of the 6-byte first MAC address."]
pub struct A31_0_R(crate::FieldReader<u32, u32>);
impl A31_0_R {
    pub(crate) fn new(bits: u32) -> Self {
        A31_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A31_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A31_0` writer - MAC Address0 \\[31:0\\]
This field contains the lower 32 bits of the 6-byte first MAC address."]
pub struct A31_0_W<'a> {
    w: &'a mut W,
}
impl<'a> A31_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\]
This field contains the lower 32 bits of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a31_0(&self) -> A31_0_R {
        A31_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\]
This field contains the lower 32 bits of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a31_0(&mut self) -> A31_0_W {
        A31_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC address0 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr_low](index.html) module"]
pub struct MAC_ADDR_LOW_SPEC;
impl crate::RegisterSpec for MAC_ADDR_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr_low::R](R) reader structure"]
impl crate::Readable for MAC_ADDR_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr_low::W](W) writer structure"]
impl crate::Writable for MAC_ADDR_LOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDR_LOW to value 0xffff_ffff"]
impl crate::Resettable for MAC_ADDR_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
