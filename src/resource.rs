use crate::FeedbackStyle;
use crate::inner::ImpactGenerators;

/// bevy **non-send** resource: offers interaction with ios impact feedback generators
/// from bevy systems. Take it as `NonSendMut<ImpactResource>`.
///
/// It cannot be a plain [`Resource`](bevy_ecs::resource::Resource): UIKit's feedback
/// generators are main-thread-only, and `NonSend` is what pins the systems touching
/// them to that thread.
///
/// on platforms other than ios this becomes a no-op
#[derive(Clone, Debug)]
pub struct ImpactResource {
    inner: Option<ImpactGenerators>,
}

impl Default for ImpactResource {
    fn default() -> Self {
        Self::new(true)
    }
}

impl ImpactResource {
    /// initialize based on config variable.
    /// ideally for loading up based on saved settings value
    pub fn new(enabled: bool) -> Self {
        Self {
            inner: if enabled {
                Some(Default::default())
            } else {
                None
            },
        }
    }
}

impl ImpactResource {
    /// allows disabling impacts without checking in user-code whether or not to call the `impact` APIs
    pub fn disable(&mut self) {
        self.inner = None;
    }

    /// enable impacts
    pub fn enable(&mut self) {
        if self.inner.is_none() {
            self.inner = Some(Default::default());
        }
    }

    /// allows fetching enabled state
    pub fn enabled(&self) -> bool {
        self.inner.is_some()
    }

    /// toggles enabled state and returns state it toggled to
    pub fn toggle(&mut self) -> bool {
        if self.inner.is_none() {
            self.enabled();
            true
        } else {
            self.disable();
            false
        }
    }

    /// Prepares the Taptic engine.
    /// According to apple docs the engine might be asleep if not prepared, which could leads to delays.
    /// If not used via eg. [Self::impact] the engine will go back to sleep to save battery.
    ///
    /// refer to apple documentation for more details:
    /// see <https://developer.apple.com/documentation/uikit/uifeedbackgenerator/2369818-prepare?language=objc>
    ///
    /// on platforms other than ios this becomes a no-op.
    pub fn prepare(&mut self) {
        if let Some(inner) = &self.inner {
            inner.prepare();
        }
    }

    /// Triggers an impact with the given `style`.
    ///
    /// refer to apple documentation for more details:
    /// see <https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator/2374286-impactoccurred?language=objc>
    ///
    /// on platforms other than ios this becomes a no-op.
    pub fn impact(&mut self, style: FeedbackStyle) {
        if let Some(inner) = &self.inner {
            inner.impact(style);
        }
    }

    /// Triggers an impact with the given `style` and `intensity`
    ///
    /// refer to apple documentation for more details:
    /// see <https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator/2374286-impactoccurredwithintensity?language=objc>
    ///
    /// on platforms other than ios this becomes a no-op.
    pub fn impact_with_intensity(&mut self, style: FeedbackStyle, intensity: f64) {
        if let Some(inner) = &self.inner {
            inner.impact_with_intensity(style, intensity);
        }
    }
}
