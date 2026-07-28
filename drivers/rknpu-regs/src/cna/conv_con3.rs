#[doc = "Register `CONV_CON3` reader"]
pub type R = crate::R<ConvCon3Spec>;
#[doc = "Register `CONV_CON3` writer"]
pub type W = crate::W<ConvCon3Spec>;
#[doc = "Field `CONV_X_STRIDE` reader - 卷积 X 步长"]
pub type ConvXStrideR = crate::FieldReader;
#[doc = "Field `CONV_X_STRIDE` writer - 卷积 X 步长"]
pub type ConvXStrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CONV_Y_STRIDE` reader - 卷积 Y 步长"]
pub type ConvYStrideR = crate::FieldReader;
#[doc = "Field `CONV_Y_STRIDE` writer - 卷积 Y 步长"]
pub type ConvYStrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DECONV_X_STRIDE` reader - 反卷积 X 步长"]
pub type DeconvXStrideR = crate::FieldReader;
#[doc = "Field `DECONV_X_STRIDE` writer - 反卷积 X 步长"]
pub type DeconvXStrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DECONV_Y_STRIDE` reader - 反卷积 Y 步长"]
pub type DeconvYStrideR = crate::FieldReader;
#[doc = "Field `DECONV_Y_STRIDE` writer - 反卷积 Y 步长"]
pub type DeconvYStrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATROUS_X_DILATION` reader - 空洞卷积 X 方向膨胀值（行方向两像素间插入的 pad 数）。>0 时启用空洞卷积"]
pub type AtrousXDilationR = crate::FieldReader;
#[doc = "Field `ATROUS_X_DILATION` writer - 空洞卷积 X 方向膨胀值（行方向两像素间插入的 pad 数）。>0 时启用空洞卷积"]
pub type AtrousXDilationW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATROUS_Y_DILATION` reader - 空洞卷积 Y 方向膨胀值（列方向两像素间插入的 pad 数）"]
pub type AtrousYDilationR = crate::FieldReader;
#[doc = "Field `ATROUS_Y_DILATION` writer - 空洞卷积 Y 方向膨胀值（列方向两像素间插入的 pad 数）"]
pub type AtrousYDilationW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NN_MODE` reader - 多核协作模式。0：32×32（单核）；1：64×32；2：96×32；4：32×64；5：32×96。单核模式保持 0"]
pub type NnModeR = crate::FieldReader;
#[doc = "Field `NN_MODE` writer - 多核协作模式。0：32×32（单核）；1：64×32；2：96×32；4：32×64；5：32×96。单核模式保持 0"]
pub type NnModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 卷积 X 步长"]
    #[inline(always)]
    pub fn conv_x_stride(&self) -> ConvXStrideR {
        ConvXStrideR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 卷积 Y 步长"]
    #[inline(always)]
    pub fn conv_y_stride(&self) -> ConvYStrideR {
        ConvYStrideR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 反卷积 X 步长"]
    #[inline(always)]
    pub fn deconv_x_stride(&self) -> DeconvXStrideR {
        DeconvXStrideR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - 反卷积 Y 步长"]
    #[inline(always)]
    pub fn deconv_y_stride(&self) -> DeconvYStrideR {
        DeconvYStrideR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 16:20 - 空洞卷积 X 方向膨胀值（行方向两像素间插入的 pad 数）。>0 时启用空洞卷积"]
    #[inline(always)]
    pub fn atrous_x_dilation(&self) -> AtrousXDilationR {
        AtrousXDilationR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - 空洞卷积 Y 方向膨胀值（列方向两像素间插入的 pad 数）"]
    #[inline(always)]
    pub fn atrous_y_dilation(&self) -> AtrousYDilationR {
        AtrousYDilationR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 28:30 - 多核协作模式。0：32×32（单核）；1：64×32；2：96×32；4：32×64；5：32×96。单核模式保持 0"]
    #[inline(always)]
    pub fn nn_mode(&self) -> NnModeR {
        NnModeR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 卷积 X 步长"]
    #[inline(always)]
    pub fn conv_x_stride(&mut self) -> ConvXStrideW<'_, ConvCon3Spec> {
        ConvXStrideW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 卷积 Y 步长"]
    #[inline(always)]
    pub fn conv_y_stride(&mut self) -> ConvYStrideW<'_, ConvCon3Spec> {
        ConvYStrideW::new(self, 3)
    }
    #[doc = "Bits 8:10 - 反卷积 X 步长"]
    #[inline(always)]
    pub fn deconv_x_stride(&mut self) -> DeconvXStrideW<'_, ConvCon3Spec> {
        DeconvXStrideW::new(self, 8)
    }
    #[doc = "Bits 11:13 - 反卷积 Y 步长"]
    #[inline(always)]
    pub fn deconv_y_stride(&mut self) -> DeconvYStrideW<'_, ConvCon3Spec> {
        DeconvYStrideW::new(self, 11)
    }
    #[doc = "Bits 16:20 - 空洞卷积 X 方向膨胀值（行方向两像素间插入的 pad 数）。>0 时启用空洞卷积"]
    #[inline(always)]
    pub fn atrous_x_dilation(&mut self) -> AtrousXDilationW<'_, ConvCon3Spec> {
        AtrousXDilationW::new(self, 16)
    }
    #[doc = "Bits 21:25 - 空洞卷积 Y 方向膨胀值（列方向两像素间插入的 pad 数）"]
    #[inline(always)]
    pub fn atrous_y_dilation(&mut self) -> AtrousYDilationW<'_, ConvCon3Spec> {
        AtrousYDilationW::new(self, 21)
    }
    #[doc = "Bits 28:30 - 多核协作模式。0：32×32（单核）；1：64×32；2：96×32；4：32×64；5：32×96。单核模式保持 0"]
    #[inline(always)]
    pub fn nn_mode(&mut self) -> NnModeW<'_, ConvCon3Spec> {
        NnModeW::new(self, 28)
    }
}
#[doc = "conv_con3\n\nYou can [`read`](crate::Reg::read) this register and get [`conv_con3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conv_con3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConvCon3Spec;
impl crate::RegisterSpec for ConvCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conv_con3::R`](R) reader structure"]
impl crate::Readable for ConvCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`conv_con3::W`](W) writer structure"]
impl crate::Writable for ConvCon3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONV_CON3 to value 0"]
impl crate::Resettable for ConvCon3Spec {}
