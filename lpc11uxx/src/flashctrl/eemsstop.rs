#[doc = "Reader of register EEMSSTOP"]
pub type R = crate::R<u32, super::EEMSSTOP>;
#[doc = "Writer for register EEMSSTOP"]
pub type W = crate::W<u32, super::EEMSSTOP>;
#[doc = "Register EEMSSTOP `reset()`'s with value 0"]
impl crate::ResetValue for super::EEMSSTOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STOPA`"]
pub type STOPA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STOPA`"]
pub struct STOPA_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `DEVSEL`"]
pub type DEVSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEVSEL`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `STRTBIST`"]
pub type STRTBIST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STRTBIST`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
}
