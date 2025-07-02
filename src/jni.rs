use crate::abstraction::{Api, Test};
use crate::click::Click;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static CLICK: Lazy<Mutex<Click>> = Lazy::new(|| Mutex::new(Click::default()));

#[no_mangle]
pub extern "system" fn Java_com_example_geetest_TripleValidator_registerTest(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let (gt, challenge) = CLICK
        .lock()
        .unwrap()
        .register_test("https://passport.bilibili.com/x/passport-login/captcha?source=main_web")
        .unwrap();

    let result_json = serde_json::json!({
        "gt": gt,
        "challenge": challenge
    })
    .to_string();

    let output = env
        .new_string(result_json)
        .expect("Couldn't create java string!");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_example_geetest_TripleValidator_simpleMatchRetry(
    mut env: JNIEnv,
    _class: JClass,
    gt: JString,
    challenge: JString,
) -> jstring {
    let gt: String = env
        .get_string(&gt)
        .expect("Couldn't get java string!")
        .into();
    let challenge: String = env
        .get_string(&challenge)
        .expect("Couldn't get java string!")
        .into();

    let result = CLICK
        .lock()
        .unwrap()
        .simple_match_retry(&gt, &challenge)
        .unwrap();

    let output = env
        .new_string(result)
        .expect("Couldn't create java string!");
    output.into_raw()
}
