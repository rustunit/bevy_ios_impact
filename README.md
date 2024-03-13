# bevy_ios_impact

Exposes [UIImpactFeedbackGenerator](https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator?language=objc) API via Bevy Resource.

On non-ios platforms this has no effect and will compile.

Currently build for `bevy="0.13"`.

This is a pre-release and only available via git dependency for now: 
`bevy_ios_impact = { version = "0.1.0", git = "https://github.com/rustunit/bevy_ios_impact.git" }`

Example:
```rust
app.init_resource::<bevy_ios_impact::ImpactFeedbackGeneratorResource>();

fn my_system(ios_impact: Res<bevy_ios_impact::ImpactFeedbackGeneratorResource>) {
  ios_impact.impact(bevy_ios_impact::UIImpactFeedbackStyle::UIImpactFeedbackStyleHeavy);
}
```
