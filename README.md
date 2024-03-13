# bevy_ios_impact

Exposes [UIImpactFeedbackGenerator](https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator?language=objc) API via Bevy Resource.

Example:
```rust
app.init_resource::<bevy_ios_impact::ImpactFeedbackGeneratorResource>();

fn my_system(ios_impact: Res<bevy_ios_impact::ImpactFeedbackGeneratorResource>) {
  ios_impact.impact(bevy_ios_impact::UIImpactFeedbackStyle::UIImpactFeedbackStyleHeavy);
}
```
