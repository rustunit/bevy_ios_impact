use icrate::Foundation::{CGFloat, NSInteger, NSObject};
use objc2::{
    extern_class, extern_methods, msg_send_id, mutability, rc::Id, ClassType, Encode, Encoding,
};

extern_class!(
    ///see `https://developer.apple.com/documentation/uikit/uifeedbackgenerator?language=objc`
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFeedbackGenerator;

    unsafe impl ClassType for UIFeedbackGenerator {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

extern_methods!(
    unsafe impl UIFeedbackGenerator {
        #[method(prepare)]
        pub fn prepare(&self);
    }
);

extern_class!(
    ///see `https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator`
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIImpactFeedbackGenerator;

    unsafe impl ClassType for UIImpactFeedbackGenerator {
        type Super = UIFeedbackGenerator;
        type Mutability = mutability::InteriorMutable;
    }
);

///see `https://developer.apple.com/documentation/uikit/uiimpactfeedbackstyle?language=objc`
#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(isize)]
pub enum UIImpactFeedbackStyle {
    UIImpactFeedbackStyleLight,
    UIImpactFeedbackStyleMedium,
    UIImpactFeedbackStyleHeavy,
    UIImpactFeedbackStyleSoft,
    UIImpactFeedbackStyleRigid,
}

unsafe impl Encode for UIImpactFeedbackStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

extern_methods!(
    unsafe impl UIImpactFeedbackGenerator {
        ///see `https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator/2374286-initwithstyle?language=objc`
        #[allow(non_snake_case)]
        pub fn initWithStyle(style: UIImpactFeedbackStyle) -> Option<Id<Self>> {
            let this = UIImpactFeedbackGenerator::alloc();
            unsafe {
                msg_send_id![
                    this,
                    initWithStyle: style,
                ]
            }
        }

        ///see `https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator/2374287-impactoccurred?language=objc`
        #[allow(non_snake_case)]
        #[method(impactOccurred)]
        pub fn impactOccurred(&self);

        ///see `https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator/3183920-impactoccurredwithintensity?language=objc`
        #[allow(non_snake_case)]
        #[method(impactOccurredWithIntensity:)]
        pub fn impactOccurredWithIntensity(&self, intensity: CGFloat);
    }
);
