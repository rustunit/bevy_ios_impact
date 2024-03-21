use bevy::ecs::system::Resource;

use crate::inner::SharedGenerators;
use crate::FeedbackStyle;

/// bevy resource: offers interaction with ios impact feedback generators from bevy systems.
///
/// on platforms other than ios this becomes a no-op
#[derive(Resource, Clone, Debug, Default)]
pub struct ImpactResource {
    inner: SharedGenerators,
    disabled: bool,
}

impl ImpactResource {
    /// allows disabling impacts without checking in user-code whether or not to call the `impact` APIs
    pub fn disable(&mut self) {
        self.disabled = true;
    }

    /// enable impacts
    pub fn enable(&mut self) {
        self.disabled = true;
    }

    /// allows fetching enabled state
    pub fn enabled(&mut self) -> bool {
        self.disabled
    }

    /// toggles enabled state and returns state it toggled to
    pub fn toggle(&mut self) -> bool {
        self.disabled = !self.disabled;
        self.disabled
    }

    /// Prepares the Taptic engine.
    /// According to apple docs the engine might be asleep if not prepared, which could leads to delays.
    /// If not used via eg. [impact] the engine will go back to sleep to save battery.
    ///
    /// refer to apple documentation for more details:
    /// see <https://developer.apple.com/documentation/uikit/uifeedbackgenerator/2369818-prepare?language=objc>
    ///
    /// on platforms other than ios this becomes a no-op.
    pub fn prepare(&mut self) {
        if !self.disabled {
            self.inner.prepare();
        }
    }

    /// Triggers an impact with the given `style`.
    ///
    /// refer to apple documentation for more details:
    /// see <https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator/2374286-impactoccurred?language=objc>
    ///
    /// on platforms other than ios this becomes a no-op.
    pub fn impact(&mut self, style: FeedbackStyle) {
        if !self.disabled {
            self.inner.impact(style);
        }
    }

    /// Triggers an impact with the given `style` and `intensity`
    ///
    /// refer to apple documentation for more details:
    /// see <https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator/2374286-impactoccurredwithintensity?language=objc>
    ///
    /// on platforms other than ios this becomes a no-op.
    pub fn impact_with_intensity(&mut self, style: FeedbackStyle, intensity: f64) {
        if !self.disabled {
            self.inner.impact_with_intensity(style, intensity);
        }
    }
}
