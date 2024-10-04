#![doc = include_str!("../README.md")]
#![no_std]

/// See [crate docs](`crate`).
#[macro_export]
macro_rules! alias {
    (
        $(#[$attr:meta])*
        $vis:vis
        trait
        $alias:ident
        $(< $(
            $tparam:ident
            $( : ( $( $tparam_bound:tt )+ ) )?
        ),+ $(,)? >)?
        : $( $bounds:tt )+
    ) => {
        $(#[$attr])*
        $vis trait $alias $( < $(
            $tparam
            $( : $($tparam_bound)+)?
        ),+ > )?
        : $( $bounds )+
        { }

        impl<__Self, $( $(
            $tparam
            $( : $($tparam_bound)+)?
        ),+ )?>
        $alias $( < $( $tparam ),+ > )?
        for __Self
        where __Self : $( $bounds )+
        { }
    }
}