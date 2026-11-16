#[doc = "Register `DCRP_SADDR0` reader"]
pub type R = crate::R<DcrpSaddr0Spec>;
#[doc = "Register `DCRP_SADDR0` writer"]
pub type W = crate::W<DcrpSaddr0Spec>;
#[doc = "Field `DCRP0_SADDR` reader - DCRP area 0 start address"]
pub type Dcrp0SaddrR = crate::FieldReader;
#[doc = "Field `DCRP0_SADDR` writer - DCRP area 0 start address"]
pub type Dcrp0SaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - DCRP area 0 start address"]
    #[inline(always)]
    pub fn dcrp0_saddr(&self) -> Dcrp0SaddrR {
        Dcrp0SaddrR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DCRP area 0 start address"]
    #[inline(always)]
    pub fn dcrp0_saddr(&mut self) -> Dcrp0SaddrW<'_, DcrpSaddr0Spec> {
        Dcrp0SaddrW::new(self, 0)
    }
}
#[doc = "DCRP start address register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrp_saddr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrp_saddr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrpSaddr0Spec;
impl crate::RegisterSpec for DcrpSaddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrp_saddr0::R`](R) reader structure"]
impl crate::Readable for DcrpSaddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcrp_saddr0::W`](W) writer structure"]
impl crate::Writable for DcrpSaddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCRP_SADDR0 to value 0"]
impl crate::Resettable for DcrpSaddr0Spec {}
