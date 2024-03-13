use bevy::ecs::system::Resource;

use crate::{impact_generator::ImpactFeedbackGenerator, UIImpactFeedbackStyle};

/// convenience type ready to be used inside bevy.
///
/// creates an instance of all possible impact styles and prepares them to be available right on app start (or whenever the resource is initialized)
#[derive(Resource, Clone, Debug)]
#[allow(dead_code)]
pub struct ImpactFeedbackGeneratorResource {
    light: ImpactFeedbackGenerator,
    medium: ImpactFeedbackGenerator,
    heavy: ImpactFeedbackGenerator,
    soft: ImpactFeedbackGenerator,
    rigid: ImpactFeedbackGenerator,
}

impl Default for ImpactFeedbackGeneratorResource {
    #[cfg(target_os = "ios")]
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

    #[cfg(not(target_os = "ios"))]
    fn default() -> Self {
        Self {
            light: ImpactFeedbackGenerator {},
            medium: ImpactFeedbackGenerator {},
            heavy: ImpactFeedbackGenerator {},
            soft: ImpactFeedbackGenerator {},
            rigid: ImpactFeedbackGenerator {},
        }
    }
}

impl ImpactFeedbackGeneratorResource {
    /// Prepares the Taptic engine
    #[cfg(target_os = "ios")]
    pub fn prepare(&mut self) {
        self.light.prepare();
    }

    ///
    #[cfg(target_os = "ios")]
    pub fn impact(&mut self, style: UIImpactFeedbackStyle) {
        self.get_generator(style).impact();
    }

    ///
    #[cfg(target_os = "ios")]
    pub fn impact_with_intensity(&mut self, style: UIImpactFeedbackStyle, intensity: f64) {
        self.get_generator(style).impact_with_intensity(intensity);
    }

    #[cfg(target_os = "ios")]
    fn get_generator(&mut self, style: UIImpactFeedbackStyle) -> &mut ImpactFeedbackGenerator {
        match style {
            UIImpactFeedbackStyle::UIImpactFeedbackStyleLight => &mut self.light,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleMedium => &mut self.medium,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleHeavy => &mut self.heavy,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleSoft => &mut self.soft,
            UIImpactFeedbackStyle::UIImpactFeedbackStyleRigid => &mut self.rigid,
        }
    }

    #[cfg(not(target_os = "ios"))]
    pub fn prepare(&mut self) {}

    #[cfg(not(target_os = "ios"))]
    pub fn impact(&mut self, _style: UIImpactFeedbackStyle) {}

    #[cfg(not(target_os = "ios"))]
    pub fn impact_with_intensity(&mut self, _style: UIImpactFeedbackStyle, _intensity: f64) {}
}
