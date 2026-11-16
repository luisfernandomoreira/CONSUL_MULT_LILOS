#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "Field `ADDM` reader - Defines which bits of ADDRESS are compared with an incoming address byte"]
pub type AddmR = crate::FieldReader;
#[doc = "Field `ADDM` writer - Defines which bits of ADDRESS are compared with an incoming address byte"]
pub type AddmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 9:15 - Defines which bits of ADDRESS are compared with an incoming address byte"]
    #[inline(always)]
    pub fn addm(&self) -> AddmR {
        AddmR::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 9:15 - Defines which bits of ADDRESS are compared with an incoming address byte"]
    #[inline(always)]
    pub fn addm(&mut self) -> AddmW<'_, Ctl2Spec> {
        AddmW::new(self, 9)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL2 to value 0xfe00"]
impl crate::Resettable for Ctl2Spec {
    const RESET_VALUE: u32 = 0xfe00;
}
