pub use Administrator as AdministratorRole;
pub use Approver as ApproverRole;
pub use User as UserRole;

pub trait AccessRole {
    fn as_enum() -> common::AccessRole;
    fn as_str() -> &'static str;
}

macro_rules! role {
    ( $( $name:ident ),+ $(,)? ) => {
        $(
            paste::paste! {
                pub struct $name;
                impl AccessRole for $name {
                    fn as_enum() -> common::AccessRole {
                        common::AccessRole::$name
                    }

                    fn as_str() -> &'static str {
                        stringify!([< $name:snake >])
                    }
                }
            }
        )+
    };
}

role! {
    User,
    Approver,
    Administrator
}
