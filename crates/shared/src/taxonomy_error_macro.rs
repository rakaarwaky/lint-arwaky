// PURPOSE: define_error! / define_wrapper! macros — eliminate duplicate error struct boilerplate
//
// Generates: struct, new() constructor, Display impl, Error impl (via thiserror).

/// Generates a standard error struct with `message`, `error_code`, `cause` fields,
/// plus `new()`, `Display`, and `Error` (via thiserror) implementations.
///
/// # Usage — with extra fields
/// ```ignore
/// define_error! {
///     pub struct MetricsError {
///         pub path: FilePath,
///     }
///     display("Metrics Error", path: " for {}")
/// }
/// ```
///
/// # Usage — without extra fields
/// ```ignore
/// define_error! {
///     pub struct PluginError
///     display("Plugin Error")
/// }
/// ```
#[macro_export]
macro_rules! define_error {
    // With extra fields and a target in display
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $(pub $field:ident : $ftype:ty),* $(,)?
        }
        display ( $prefix:expr , $target_field:ident : $target_fmt:expr )
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, thiserror::Error)]
        pub struct $name {
            $(pub $field: $ftype,)*
            pub message: $crate::common::taxonomy_common_error::ErrorMessage,
            pub error_code: $crate::common::taxonomy_error_vo::ErrorCode,
            pub cause: $crate::common::taxonomy_common_error::Cause,
        }

        impl $name {
            pub fn new(message: $crate::common::taxonomy_common_error::ErrorMessage) -> Self {
                Self {
                    $($field: <$ftype>::default(),)*
                    message,
                    error_code: $crate::common::taxonomy_error_vo::ErrorCode::default(),
                    cause: $crate::common::taxonomy_common_error::Cause::default(),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let target_str = self.$target_field.to_string();
                let target = if target_str.is_empty() {
                    String::new()
                } else {
                    format!($target_fmt, target_str)
                };
                let code_str = self.error_code.to_string();
                let code = if code_str.is_empty() {
                    String::new()
                } else {
                    format!(" [{}]", code_str)
                };
                write!(f, "{}{}{}: {}", $prefix, target, code, self.message)
            }
        }
    };

    // Without extra fields (no target)
    (
        $(#[$meta:meta])*
        pub struct $name:ident
        display ( $prefix:expr )
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, thiserror::Error)]
        pub struct $name {
            pub message: $crate::common::taxonomy_common_error::ErrorMessage,
            pub error_code: $crate::common::taxonomy_error_vo::ErrorCode,
            pub cause: $crate::common::taxonomy_common_error::Cause,
        }

        impl $name {
            pub fn new(message: $crate::common::taxonomy_common_error::ErrorMessage) -> Self {
                Self {
                    message,
                    error_code: $crate::common::taxonomy_error_vo::ErrorCode::default(),
                    cause: $crate::common::taxonomy_common_error::Cause::default(),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let code_str = self.error_code.to_string();
                let code = if code_str.is_empty() {
                    String::new()
                } else {
                    format!(" [{}]", code_str)
                };
                write!(f, "{}{}: {}", $prefix, code, self.message)
            }
        }
    };
}

/// Generates a simple wrapper error struct that delegates to a base error type.
///
/// # Usage
/// ```ignore
/// define_wrapper! {
///     pub struct WatchSubscriptionError {
///         pub base: WatchServiceError,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_wrapper {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            pub base: $base:ty $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, thiserror::Error)]
        pub struct $name {
            #[serde(flatten)]
            pub base: $base,
        }

        impl $name {
            pub fn new(message: $crate::common::taxonomy_common_error::ErrorMessage) -> Self {
                Self {
                    base: <$base>::new(message),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.base)
            }
        }
    };
}
