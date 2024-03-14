use bevy::ecs::system::Resource;

use crate::inner::SharedGenerators;
use crate::UIImpactFeedbackStyle;

/// bevy resource: offers interaction with ios impact feedback generators from bevy systems.
///
/// on platforms other than ios this becomes a no-op
#[derive(Resource, Clone, Debug, Default)]
pub struct ImpactResource {
    inner: SharedGenerators,
}

impl ImpactResource {
    /// Prepares the Taptic engine.
    ///
    /// refer to apple documentation for more details:
    /// see <https://developer.apple.com/documentation/uikit/uifeedbackgenerator/2369818-prepare?language=objc>
    ///
    /// on platforms other than ios this becomes a no-op.
    pub fn prepare(&mut self) {
        self.inner.prepare();
    }

    /// Triggers an impact with the given [style].
    ///
    /// refer to apple documentation for more details:
    /// see <https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator/2374286-impactoccurred?language=objc>
    ///
    /// on platforms other than ios this becomes a no-op.
    pub fn impact(&mut self, style: UIImpactFeedbackStyle) {
        self.inner.impact(style);
    }

    /// Triggers an impact with the given `style` and `intensity`
    ///
    /// refer to apple documentation for more details:
    /// see <https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator/2374286-impactoccurredwithintensity?language=objc>
    ///
    /// on platforms other than ios this becomes a no-op.
    pub fn impact_with_intensity(&mut self, style: UIImpactFeedbackStyle, intensity: f64) {
        self.inner.impact_with_intensity(style, intensity);
    }
}
