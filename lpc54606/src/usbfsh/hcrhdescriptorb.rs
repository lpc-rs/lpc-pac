#[doc = "Reader of register HCRHDESCRIPTORB"]
pub type R = crate::R<u32, super::HCRHDESCRIPTORB>;
#[doc = "Writer for register HCRHDESCRIPTORB"]
pub type W = crate::W<u32, super::HCRHDESCRIPTORB>;
#[doc = "Register HCRHDESCRIPTORB `reset()`'s with value 0"]
impl crate::ResetValue for super::HCRHDESCRIPTORB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DR`"]
pub type DR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DR`"]
pub struct DR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PPCM`"]
pub type PPCM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PPCM`"]
pub struct PPCM_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DeviceRemovable Each bit is dedicated to a port of the Root Hub."]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set."]
    #[inline(always)]
    pub fn ppcm(&self) -> PPCM_R {
        PPCM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DeviceRemovable Each bit is dedicated to a port of the Root Hub."]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W {
        DR_W { w: self }
    }
    #[doc = "Bits 16:31 - PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set."]
    #[inline(always)]
    pub fn ppcm(&mut self) -> PPCM_W {
        PPCM_W { w: self }
    }
}
