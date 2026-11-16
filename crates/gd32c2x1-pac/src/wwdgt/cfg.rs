#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `WIN` reader - 7-bit window value"]
pub type WinR = crate::FieldReader;
#[doc = "Field `WIN` writer - 7-bit window value"]
pub type WinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PSC_0_1` reader - Prescaler"]
pub type Psc0_1R = crate::FieldReader;
#[doc = "Field `PSC_0_1` writer - Prescaler"]
pub type Psc0_1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EWIE` reader - Early wakeup interrupt"]
pub type EwieR = crate::BitReader;
#[doc = "Field `EWIE` writer - Early wakeup interrupt"]
pub type EwieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSC_2_3` reader - Prescaler"]
pub type Psc2_3R = crate::FieldReader;
#[doc = "Field `PSC_2_3` writer - Prescaler"]
pub type Psc2_3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Prescaler"]
    #[inline(always)]
    pub fn psc_0_1(&self) -> Psc0_1R {
        Psc0_1R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewie(&self) -> EwieR {
        EwieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Prescaler"]
    #[inline(always)]
    pub fn psc_2_3(&self) -> Psc2_3R {
        Psc2_3R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn win(&mut self) -> WinW<'_, CfgSpec> {
        WinW::new(self, 0)
    }
    #[doc = "Bits 7:8 - Prescaler"]
    #[inline(always)]
    pub fn psc_0_1(&mut self) -> Psc0_1W<'_, CfgSpec> {
        Psc0_1W::new(self, 7)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewie(&mut self) -> EwieW<'_, CfgSpec> {
        EwieW::new(self, 9)
    }
    #[doc = "Bits 16:17 - Prescaler"]
    #[inline(always)]
    pub fn psc_2_3(&mut self) -> Psc2_3W<'_, CfgSpec> {
        Psc2_3W::new(self, 16)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0x7f"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x7f;
}
