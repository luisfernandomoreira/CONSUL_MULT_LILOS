#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `ECCMEIF` reader - SRAM two bits non-correction event flag"]
pub type EccmeifR = crate::BitReader;
#[doc = "Field `ECCMEIF` writer - SRAM two bits non-correction event flag"]
pub type EccmeifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCSEIF` reader - SRAM single bit correction event flag"]
pub type EccseifR = crate::BitReader;
#[doc = "Field `ECCSEIF` writer - SRAM single bit correction event flag"]
pub type EccseifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SRAM two bits non-correction event flag"]
    #[inline(always)]
    pub fn eccmeif(&self) -> EccmeifR {
        EccmeifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM single bit correction event flag"]
    #[inline(always)]
    pub fn eccseif(&self) -> EccseifR {
        EccseifR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM two bits non-correction event flag"]
    #[inline(always)]
    pub fn eccmeif(&mut self) -> EccmeifW<'_, StatSpec> {
        EccmeifW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM single bit correction event flag"]
    #[inline(always)]
    pub fn eccseif(&mut self) -> EccseifW<'_, StatSpec> {
        EccseifW::new(self, 1)
    }
}
#[doc = "System status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
