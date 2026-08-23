#[macro_export]
macro_rules! command {
    (
        const $map_name:ident;
        $( prelude => { $($prelude:tt)* } $(,)? )?
        $(
            $prefix:literal:
                $( $syntax1:ident )? $( [ $( $syntax:expr ),* ] )? $(:)? $(,)?
                $( chain_filter_map |$chaind:ident, $chainv:ident, $chain:ident| => $chaine:tt )?
                $( filter_map |$descriptor:ident, $filter_var:ident| => $filter_expr:tt )?
                $( |$var:ident| => $expr:tt )?
                $(,)?
        )*
    ) => {
        const PRELUDE: &str = { "" $(; stringify!($($prelude)*) )? };
        pub const $map_name: &[&$crate::Command] = &[
            $(
                &$crate::Command {
                    prefix: $prefix,
                    prelude: PRELUDE,
                    syntax: & $( $syntax1 )? $( Map::All(&[ $($syntax),* ]) )? ,
                    generate: {
                        #[allow(unused)]
                        |_: &str, x: ::proc_macro2::TokenStream| { x }

                        $(;
                            |_: &str, $var: ::proc_macro2::TokenStream| {
                                ::quote::quote! { $expr }
                            }
                        )?

                        $(; |$descriptor, $filter_var| { $filter_expr } )?
                    },
                    generate_chain: {
                        #[allow(unused)]
                        None::<$crate::CommandGenChain>

                        $(; Some(|$chaind, $chainv, $chain| { $chaine }) )?
                    },
                },
            )*
        ];
    };
}

#[macro_export]
macro_rules! group {
    (
        const $group_name:ident;
        $( ~ repeat: $repeat_type:ident, )?
        $( ~ init => { $($init_expr:tt)* } $(,)? )?
        $( ~ fini => { $($fini_expr:tt)* } $(,)? )?
        $(
            $(
                $(
                    @ $mod_path:literal &$mod_map:ident
                )?
                $(
                    &$map:ident
                )?
                $(
                    $lit:literal
                )?
                $(
                    ( $lit1:literal $(| $lits:literal)* )
                )?
                $(
                    $ident:ident : $ty:ty
                        $( as $( $ident_flags:literal )* )?
                )?
            ),+
                // $(,)?
                => { $($expr:tt)* }
                // $(,)?
        )*
    ) => {
        #[allow(unused)]
        pub const $group_name: $crate::Map =
        $crate::Map::All(&[
            $( $crate::Map::Code(stringify!($($init_expr)*)), )?
            $crate::Map::Any(&[
                $(
                    $crate::Map::All(&[
                        $(
                            $( { #[path = $mod_path] mod inner; inner::$mod_map }, )?
                            $( $map, )?
                            $( $crate::Map::Lit($lit), )?
                            $(
                                $crate::Map::Any(&[
                                    $crate::Map::Lit($lit1),
                                    $( $crate::Map::Lit($lits), )*
                                ]),
                            )?
                            $(
                                $crate::Map::Val($crate::Val {
                                    name: stringify!($ident),
                                    ty: stringify!($ty),
                                    flags: &[$( $( $ident_flags , )* )?],
                                }),
                            )?
                        )*
                        $crate::Map::Code(stringify!($($expr)*)),
                    ]),
                )*
            ])
            $(.$repeat_type())?
            ,
            $( $crate::Map::Code(stringify!($($fini_expr)*)), )?
        ]);
    };
}
