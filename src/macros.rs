macro_rules! ext_impl {
    {$(#[$m:meta] $fn:item)*} => {
        macro_rules! exts {
           () => { $(#[$m]$fn)* };
        }

        pub(super) use exts;
    };
}

macro_rules! add_exts {
    ($($name:ident),*) => {
        $(
            $name::exts!();
        )*

    };
}

macro_rules! rollercoaster {
    ($($name:ident),*; $($feature:literal = $($feature_mod:ident),*);*) => {
        $(
            mod $name;
            use crate::$name::*;
        )*


        pub trait Rollercoaster: Iterator
        where
            Self: Sized,
        {
            add_exts!($($name),*);

            $(
                #[cfg(feature = $feature)]
                add_exts!($($feature_mod),*);
            )*
        }

        impl<T: Iterator> Rollercoaster for T {}
    };
}
