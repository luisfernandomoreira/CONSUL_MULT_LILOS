#[doc = "Register `DCRP_SADDR1` reader"]
pub type R = crate::R<DcrpSaddr1Spec>;
#[doc = "Register `DCRP_SADDR1` writer"]
pub type W = crate::W<DcrpSaddr1Spec>;
#[doc = "Field `DCRP1_SADDR` reader - DCRP area 1 start address offset"]
pub type Dcrp1SaddrR = crate::FieldReader;
#[doc = "Field `DCRP1_SADDR` writer - DCRP area 1 start address offset"]
pub type Dcrp1SaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - DCRP area 1 start address offset"]
    #[inline(always)]
    pub fn dcrp1_saddr(&self) -> Dcrp1SaddrR {
        Dcrp1SaddrR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DCRP area 1 start address offset"]
    #[inline(always)]
    pub fn dcrp1_saddr(&mut self) -> Dcrp1SaddrW<'_, DcrpSaddr1Spec> {
        Dcrp1SaddrW::new(self, 0)
    }
}
#[doc = "DCRP start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrp_saddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrp_saddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrpSaddr1Spec;
impl crate::RegisterSpec for DcrpSaddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcrp_saddr1::R`](R) reader structure"]
impl crate::Readable for DcrpSaddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dcrp_saddr1::W`](W) writer structure"]
impl crate::Writable for DcrpSaddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCRP_SADDR1 to value 0"]
impl crate::Resettable for DcrpSaddr1Spec {}
