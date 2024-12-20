/// Facilitates the creation of error structures with a custom display message
///
/// # Example
/// ```
/// use rs42::error_struct_custom_display;
///
/// error_struct_custom_display!(
///     ErrorStructName,
///     "Error msg",
/// );
///
/// error_struct_custom_display!(
///     OtherErrorStructName {
///         data: u32,
///     },
///     "Error msg {}", data
/// );
/// ```
#[macro_export]
macro_rules! error_struct_custom_display {
    ($struct_name:ident $( { $( $field_name:ident : $field_type:ty ),* $(,)? } )?,
     $format_message:expr $(, $( $format_var:ident ), * $(,)? )?) => {
        pub struct $struct_name {
            $( $( pub $field_name : $field_type ),*, )?
        }

        impl $struct_name {
            #[allow(dead_code)]
            pub fn new($( $( $field_name : impl Into<$field_type> ),* )?) -> Self {
                $struct_name {
                    $( $( $field_name: $field_name.into() ),* )?
                }
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $format_message $(, $( self.$format_var ), * )?)
            }
        }

        impl std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{self}")
            }
        }

        impl std::error::Error for $struct_name {}
    };
}

/// Facilitates the creation of error structures with debug display
///
/// # Example
/// ```
/// use rs42::error_struct;
///
/// error_struct!(ErrorStructName);
///
/// error_struct!(
///     OtherErrorStructName {
///         data: u32,
///     },
/// );
/// ```
#[macro_export]
macro_rules! error_struct {
    ($struct_name:ident $( { $( $field_name:ident : $field_type:ty ),* $(,)? } )?$(,)?) => {
        #[derive(Debug)]
        pub struct $struct_name {
            $( $( pub $field_name : $field_type ),*, )?
        }

        impl $struct_name {
            #[allow(dead_code)]
            pub fn new($( $( $field_name : impl Into<$field_type> ),* )?) -> Self {
                $struct_name {
                    $( $( $field_name: $field_name.into() ),* )?
                }
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{self:?}")
            }
        }

        impl std::error::Error for $struct_name {}
    };
}
