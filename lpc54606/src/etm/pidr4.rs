#[doc = "Reader of register PIDR4"]
pub type R = crate::R<u32, super::PIDR4>;
#[doc = "Reader of field `JEP106`"]
pub type JEP106_R = crate::R<u8, u8>;
#[doc = "Reader of field `c4KB`"]
pub type C4KB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code."]
    #[inline(always)]
    pub fn jep106(&self) -> JEP106_R {
        JEP106_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB Count"]
    #[inline(always)]
    pub fn c4kb(&self) -> C4KB_R {
        C4KB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
