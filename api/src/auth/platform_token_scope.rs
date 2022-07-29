pub use Read as ReadScope;
pub use Write as WriteScope;

pub trait PlatformTokenScope {
    fn as_enum() -> common_platform::PlatformTokenScope;
    fn as_str() -> &'static str;
}

macro_rules! scope {
    ( $( $name:ident ),+ $(,)? ) => {
        $(
            paste::paste! {
                pub struct $name;
                impl PlatformTokenScope for $name {
                    fn as_enum() -> common_platform::PlatformTokenScope {
                        common_platform::PlatformTokenScope::$name
                    }

                    fn as_str() -> &'static str {
                        stringify!([< $name:snake >])
                    }
                }
            }
        )+
    };
}

scope! {
    Read,
    Write,
}
