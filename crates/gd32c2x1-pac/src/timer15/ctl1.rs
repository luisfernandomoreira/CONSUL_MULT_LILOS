#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `CCSE` reader - Commutation control shadow enable"]
pub type CcseR = crate::BitReader;
#[doc = "Field `CCSE` writer - Commutation control shadow enable"]
pub type CcseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCUC` reader - Commutation control shadow register update control"]
pub type CcucR = crate::BitReader;
#[doc = "Field `CCUC` writer - Commutation control shadow register update control"]
pub type CcucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAS` reader - DMA request source selection"]
pub type DmasR = crate::BitReader;
#[doc = "Field `DMAS` writer - DMA request source selection"]
pub type DmasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO0` reader - Idle state of channel 0 output"]
pub type Iso0R = crate::BitReader;
#[doc = "Field `ISO0` writer - Idle state of channel 0 output"]
pub type Iso0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO0N` reader - Idle state of channel 0 complementary output"]
pub type Iso0nR = crate::BitReader;
#[doc = "Field `ISO0N` writer - Idle state of channel 0 complementary output"]
pub type Iso0nW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Commutation control shadow enable"]
    #[inline(always)]
    pub fn ccse(&self) -> CcseR {
        CcseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&self) -> CcucR {
        CcucR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&self) -> DmasR {
        DmasR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&self) -> Iso0R {
        Iso0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Idle state of channel 0 complementary output"]
    #[inline(always)]
    pub fn iso0n(&self) -> Iso0nR {
        Iso0nR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Commutation control shadow enable"]
    #[inline(always)]
    pub fn ccse(&mut self) -> CcseW<'_, Ctl1Spec> {
        CcseW::new(self, 0)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&mut self) -> CcucW<'_, Ctl1Spec> {
        CcucW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&mut self) -> DmasW<'_, Ctl1Spec> {
        DmasW::new(self, 3)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&mut self) -> Iso0W<'_, Ctl1Spec> {
        Iso0W::new(self, 8)
    }
    #[doc = "Bit 9 - Idle state of channel 0 complementary output"]
    #[inline(always)]
    pub fn iso0n(&mut self) -> Iso0nW<'_, Ctl1Spec> {
        Iso0nW::new(self, 9)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
