# bevy_ios_impact

Exposes [UIImpactFeedbackGenerator](https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator?language=objc) API via Bevy Resource.

On non-ios platforms this has no effect and will compile.

Currently build for `bevy="0.13"`.

usage:
```
bevy_ios_impact = { version = "0.1.0" }
```

Example:
```rust
app.add_plugins(bevy_ios_impact::ImpactPlugin);

fn my_system(mut impacts: ResMut<bevy_ios_impact::ImpactResource>) {
  // optional: haptic engine might be asleep if not prepared.
  // in practice i never felt a delay, but see apple docs on this:
  // https://developer.apple.com/documentation/uikit/uifeedbackgenerator?language=objc
  impacts.prepare();

  // triggere the impact with different impact strengths (or 'style')
  impacts.impact(bevy_ios_impact::UIImpactFeedbackStyle::UIImpactFeedbackStyleHeavy);
}
```

# License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

## Your contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
