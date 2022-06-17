macro_rules! ext_impl {
    {$(#[$m:meta] $fn:item)*} => {
        macro_rules! exts {
           () => { $(#[$m]$fn)* };
        }

        pub(super) use exts;
    };
}

macro_rules! add_exts {
    ($($name:ident)*) => {
        $(
            $name::exts!();
        )*

    };
}
