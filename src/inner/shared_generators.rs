use super::impact_generators::ImpactGenerators;
use ios_impact::UIImpactFeedbackStyle;

use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct SharedGenerators {
    internal: Arc<Mutex<ImpactGenerators>>,
}

unsafe impl Send for SharedGenerators {}
unsafe impl Sync for SharedGenerators {}

impl Default for SharedGenerators {
    fn default() -> Self {
        Self {
            internal: Arc::new(Mutex::new(ImpactGenerators::default())),
        }
    }
}

impl SharedGenerators {
    pub fn prepare(&self) {
        if let Ok(internal) = self.internal.lock() {
            internal.prepare();
        }
    }

    pub fn impact(&self, style: UIImpactFeedbackStyle) {
        if let Ok(internal) = self.internal.lock() {
            internal.impact(style);
        }
    }

    pub fn impact_with_intensity(&self, style: UIImpactFeedbackStyle, intensity: f64) {
        if let Ok(internal) = self.internal.lock() {
            internal.impact_with_intensity(style, intensity);
        }
    }
}
