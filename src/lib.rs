// #![feature(trace_macros)]
//
// trace_macros!(true);

// #[macro_export]
// macro_rules! transform_struct {
//     (pub struct $base_struct:ident $new_stuct:ident {
//         $( $simple_field:ident: $simple_ty:ty ),*
//         $([ ($trans_field:ident, $trans_func:ident) : ($trans_base_ty:ty, $trans_new_ty:ty) ]),*
//         $(,)?
//     }) => {
//         pub struct $base_struct {
//             $(
//                 $simple_field: $simple_ty
//             ),*,
//             $(
//                 $trans_field : $trans_base_ty
//             ),*
//         }
//
//         // pub struct $new_struct {
//         //     $(
//         //         $field: $trans_new_ty,
//         //     ),*
//         // }
//     }
// }

#[macro_export]
macro_rules! transform_struct {
    // Case when simple (non-transformed) members are present. Separate cases are needed to handle
    // commas at the end of simple (non-transformed) members
    (pub struct $base_struct:ident $new_struct:ident {
        $( $simple_field:ident: $simple_ty:ty ),+ $(,)?
        $(> {
            $($trans_field:ident : $trans_base_ty:ty => ($trans_func:ident -> $trans_new_ty:ty) ),+
            $(,)?
        })?
    }) => {
        pub struct $base_struct {
            $(
                $simple_field: $simple_ty
            ),+,
            $(
                $(
                    $trans_field : $trans_base_ty
                ),+
            )?
        }

        pub struct $new_struct {
            $(
                $simple_field: $simple_ty
            ),+,
            $(
                $(
                    $trans_field : $trans_new_ty
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
    (pub struct $base_struct:ident $new_struct:ident {
        $(> {
            $($trans_field:ident : $trans_base_ty:ty => ($trans_func:ident -> $trans_new_ty:ty) ),+
            $(,)?
        })?
    }) => {
        pub struct $base_struct {
            $(
                $(
                    $trans_field : $trans_base_ty
                ),+
            )?
        }

        pub struct $new_struct {
            $(
                $(
                    $trans_field : $trans_new_ty
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
