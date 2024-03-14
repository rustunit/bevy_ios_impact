use ios_impact::{UIImpactFeedbackGenerator, UIImpactFeedbackStyle};
use objc2::rc::Id;

#[derive(Clone, Debug)]
pub(crate) struct ImpactGenerators {
    light: Id<UIImpactFeedbackGenerator>,
    medium: Id<UIImpactFeedbackGenerator>,
    heavy: Id<UIImpactFeedbackGenerator>,
    soft: Id<UIImpactFeedbackGenerator>,
    rigid: Id<UIImpactFeedbackGenerator>,
}

impl Default for ImpactGenerators {
    fn default() -> Self {
        Self {
            light: UIImpactFeedbackGenerator::initWithStyle(
                UIImpactFeedbackStyle::UIImpactFeedbackStyleLight,
            )
            .unwrap(),
            medium: UIImpactFeedbackGenerator::initWithStyle(
                UIImpactFeedbackStyle::UIImpactFeedbackStyleMedium,
            )
            .unwrap(),
            heavy: UIImpactFeedbackGenerator::initWithStyle(
                UIImpactFeedbackStyle::UIImpactFeedbackStyleHeavy,
            )
            .unwrap(),
            soft: UIImpactFeedbackGenerator::initWithStyle(
                UIImpactFeedbackStyle::UIImpactFeedbackStyleSoft,
            )
            .unwrap(),
            rigid: UIImpactFeedbackGenerator::initWithStyle(
                UIImpactFeedbackStyle::UIImpactFeedbackStyleRigid,
            )
            .unwrap(),
        }
    }
}

impl ImpactGenerators {
    pub fn prepare(&self) {
        self.light.prepare();
    }

    pub fn impact(&self, style: UIImpactFeedbackStyle) {
        self.get_generator(style).impactOccurred();
    }

    pub fn impact_with_intensity(&self, style: UIImpactFeedbackStyle, intensity: f64) {
        self.get_generator(style)
            .impactOccurredWithIntensity(intensity);
    }

    fn get_generator(&self, style: UIImpactFeedbackStyle) -> &Id<UIImpactFeedbackGenerator> {
        match style {
            UIImpactFeedbackStyle::UIImpactFeedbackStyleLight => &self.light,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleMedium => &self.medium,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleHeavy => &self.heavy,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleSoft => &self.soft,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleRigid => &self.rigid,
        }
    }
}
