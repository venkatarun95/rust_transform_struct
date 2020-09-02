#[macro_export]
macro_rules! transform_struct {
    // Case when simple (non-transformed) members are present. Separate cases are needed to handle
    // commas at the end of simple (non-transformed) members
    ($(#[derive($($arg:tt),+)])?
    $struct_vis:vis struct $base_struct:ident $new_struct:ident {
        $( $field_vis:vis $simple_field:ident: $simple_ty:ty ),+ $(,)?
        $(> {
            $($trans_vis:vis $trans_field:ident : $trans_base_ty:ty => ($trans_func:ident -> $trans_new_ty:ty) ),+
            $(,)?
        })?
    }) => {
        $(#[derive($($arg),+)])?
        $struct_vis struct $base_struct {
            $(
                $field_vis $simple_field: $simple_ty
            ),+,
            $(
                $(
                    $trans_vis $trans_field : $trans_base_ty
                ),+
            )?
        }

        $(#[derive($($arg),+)])?
        $struct_vis struct $new_struct {
            $(
                $field_vis $simple_field: $simple_ty
            ),+,
            $(
                $(
                    $trans_vis $trans_field : $trans_new_ty
                ),+
            )?
        }

        impl From<$base_struct> for $new_struct {
            fn from(base: $base_struct) -> $new_struct {
                Self {
                    $(
                        $simple_field: base.$simple_field
                    ),+,
                    $(
                        $(
                            $trans_field : $trans_func(base.$trans_field)
                        ),+
                    )?
                }
            }
        }
    };
    // Case when only transformed members are present
    ($(#[derive($($arg:tt),+)])?
    $struct_vis:vis struct $base_struct:ident $new_struct:ident {
        $(> {
            $($field_vis:vis $trans_field:ident : $trans_base_ty:ty => ($trans_func:ident -> $trans_new_ty:ty) ),+
            $(,)?
        })?
    }) => {
        $(#[derive($($arg),+)])?
        $struct_vis struct $base_struct {
            $(
                $(
                    $field_vis $trans_field : $trans_base_ty
                ),+
            )?
        }

        $(#[derive($($arg),+)])?
        $struct_vis struct $new_struct {
            $(
                $(
                    $field_vis $trans_field : $trans_new_ty
                ),+
            )?
        }

        impl From<$base_struct> for $new_struct {
            fn from(base: $base_struct) -> $new_struct {
                Self {
                    $(
                        $(
                            $trans_field : $trans_func(base.$trans_field)
                        ),+
                    )?
                }
            }
        }
    };


}
