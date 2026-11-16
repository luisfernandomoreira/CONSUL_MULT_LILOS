#[doc = "Register `INSEL` reader"]
pub type R = crate::R<InselSpec>;
#[doc = "Register `INSEL` writer"]
pub type W = crate::W<InselSpec>;
#[doc = "Field `CI0_SEL` reader - Channel 0 input selection"]
pub type Ci0SelR = crate::FieldReader;
#[doc = "Field `CI0_SEL` writer - Channel 0 input selection"]
pub type Ci0SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Channel 0 input selection"]
    #[inline(always)]
    pub fn ci0_sel(&self) -> Ci0SelR {
        Ci0SelR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel 0 input selection"]
    #[inline(always)]
    pub fn ci0_sel(&mut self) -> Ci0SelW<'_, InselSpec> {
        Ci0SelW::new(self, 0)
    }
}
#[doc = "input selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`insel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`insel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InselSpec;
impl crate::RegisterSpec for InselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`insel::R`](R) reader structure"]
impl crate::Readable for InselSpec {}
#[doc = "`write(|w| ..)` method takes [`insel::W`](W) writer structure"]
impl crate::Writable for InselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INSEL to value 0"]
impl crate::Resettable for InselSpec {}
