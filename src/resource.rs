use bevy::ecs::system::Resource;

use crate::{impact_generator::ImpactFeedbackGenerator, UIImpactFeedbackStyle};

/// convenience type ready to be used inside bevy.
///
/// creates an instance of all possible impact styles and prepares them to be available right on app start (or whenever the resource is initialized)
#[derive(Resource, Clone, Debug)]
pub struct ImpactFeedbackGeneratorResource {
    light: ImpactFeedbackGenerator,
    medium: ImpactFeedbackGenerator,
    heavy: ImpactFeedbackGenerator,
    soft: ImpactFeedbackGenerator,
    rigid: ImpactFeedbackGenerator,
}

impl Default for ImpactFeedbackGeneratorResource {
    fn default() -> Self {
        Self {
            light: ImpactFeedbackGenerator::new(UIImpactFeedbackStyle::UIImpactFeedbackStyleLight),
            medium: ImpactFeedbackGenerator::new(
                UIImpactFeedbackStyle::UIImpactFeedbackStyleMedium,
            ),
            heavy: ImpactFeedbackGenerator::new(UIImpactFeedbackStyle::UIImpactFeedbackStyleHeavy),
            soft: ImpactFeedbackGenerator::new(UIImpactFeedbackStyle::UIImpactFeedbackStyleSoft),
            rigid: ImpactFeedbackGenerator::new(UIImpactFeedbackStyle::UIImpactFeedbackStyleRigid),
        }
    }
}

impl ImpactFeedbackGeneratorResource {
    ///
    pub fn impact(&self, style: UIImpactFeedbackStyle) {
        let generator = match style {
            UIImpactFeedbackStyle::UIImpactFeedbackStyleLight => &self.light,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleMedium => &self.medium,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleHeavy => &self.heavy,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleSoft => &self.soft,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleRigid => &self.rigid,
        };
        generator.impact();
    }

    ///
    pub fn impact_with_intensity(&self, style: UIImpactFeedbackStyle, intensity: f64) {
        let generator = match style {
            UIImpactFeedbackStyle::UIImpactFeedbackStyleLight => &self.light,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleMedium => &self.medium,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleHeavy => &self.heavy,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleSoft => &self.soft,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleRigid => &self.rigid,
        };
        generator.impact_with_intensity(intensity);
    }
}
