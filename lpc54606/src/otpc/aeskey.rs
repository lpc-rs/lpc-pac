#[doc = "Reader of register AESKEY[%s]"]
pub type R = crate::R<u32, super::AESKEY>;
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES key."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
