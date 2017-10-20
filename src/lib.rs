extern crate jni;

pub use jni::JNIEnv;
pub use jni::objects::JClass;
use jni::objects::JString;

pub trait JvmConvertible<'jni> {
    type JvmType;
    fn into_jvm_type<'a>(self, env: &'a JNIEnv<'jni>) -> Self::JvmType;
    fn from_jvm_type<'a>(env: &'a JNIEnv<'jni>, jvm_value: Self::JvmType) -> Self;
}

impl<'jni> JvmConvertible<'jni> for String {
    type JvmType = JString<'jni>;

    fn into_jvm_type<'a>(self, env: &'a JNIEnv<'jni>) -> Self::JvmType {
        env.new_string(self).expect("Couldn't create Java string")
    }

    fn from_jvm_type<'a>(env: &'a JNIEnv<'jni>, jvm_value: Self::JvmType) -> Self {
        env.get_string(jvm_value).expect("Couldn't get Java string").into()
    }
}

impl<'jni, 'b> JvmConvertible<'jni> for &'b str {
    type JvmType = JString<'jni>;

    fn into_jvm_type<'a>(self, env: &'a JNIEnv<'jni>) -> Self::JvmType {
        env.new_string(self).expect("Couldn't create Java string")
    }

    fn from_jvm_type<'a>(_env: &'a JNIEnv<'jni>, _jvm_value: Self::JvmType) -> Self {
        let (file, line) = (file!(), line!());
        unimplemented!("This is actually impossible to do safely, {}:{}", file, line)
    }
}

macro_rules! jvm_primitives {
    ($($typ:ty),*) => {
        $(
            impl<'jni> JvmConvertible<'jni> for $typ {
                type JvmType = $typ;

                fn into_jvm_type<'a>(self, _env: &'a JNIEnv<'jni>) -> Self::JvmType {
                    self
                }

                fn from_jvm_type<'a>(_env: &'a JNIEnv<'jni>, jvm_value: Self::JvmType) -> Self {
                    jvm_value
                }
            }
        )*
    };
}

// see jni::sys for available types
jvm_primitives!(i32, i64, i8, u8, u16, i16, f32, f64, ());