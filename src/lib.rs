#[macro_export]
macro_rules! transform_struct {
    // Case when simple (non-transformed) members are present. Separate cases are needed to handle
    // commas at the end of simple (non-transformed) members
    ($(#[$outer_meta1:meta])*
    $struct_vis1:vis struct $base_struct:ident
    $(#[$outer_meta2:meta])*
    $struct_vis2:vis struct $new_struct:ident {
        $(
            $(#[$inner_meta_1:meta])*
            $field_vis:vis $simple_field:ident: $simple_ty:ty
        ),+
        $(,)?
        $(> {
            $(
                $(#[$inner_meta_2:meta])*
                $trans_vis:vis $trans_field:ident : $trans_base_ty:ty => ($trans_func:ident -> $trans_new_ty:ty)
            ),+
            $(,)?
        })?
    }) => {
        $(#[$outer_meta1])*
        $struct_vis1 struct $base_struct {
            $(
                $(#[$inner_meta_1])*
                $field_vis $simple_field: $simple_ty
            ),+,
            $(
                $(
                    $(#[$inner_meta_2])*
                    $trans_vis $trans_field : $trans_base_ty
                ),+
            )?
        }

        $(#[$outer_meta2])*
        $struct_vis2 struct $new_struct {
            $(
                $(#[$inner_meta_1])*
                $field_vis $simple_field: $simple_ty
            ),+,
            $(
                $(
                    $(#[$inner_meta_2])*
                    $trans_vis $trans_field : $trans_new_ty
                ),+
            )?
        }

        #[automatically_derived]
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
    (
        $(#[$outer_meta1:meta])*
        $struct_vis1:vis struct $base_struct:ident
        $(#[$outer_meta2:meta])*
        $struct_vis2:vis struct $new_struct:ident {
            $(> {
                $(
                    $(#[$inner_meta:meta])*
                    $field_vis:vis $trans_field:ident : $trans_base_ty:ty => ($trans_func:ident -> $trans_new_ty:ty)
                ),+
                $(,)?
            })?
        }) => {
            $(#[$outer_meta1])*
            $struct_vis1 struct $base_struct {
                $(
                    $(
                        $(#[$inner_meta])*
                        $field_vis $trans_field : $trans_base_ty
                    ),+
                )?
            }

            $(#[$outer_meta2])*
            $struct_vis2 struct $new_struct {
                $(
                    $(
                        $(#[$inner_meta])*
                        $field_vis $trans_field : $trans_new_ty
                    ),+
                )?
            }

            #[automatically_derived]
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
