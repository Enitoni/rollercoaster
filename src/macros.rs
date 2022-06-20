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

        /// Adds useful extension iterators for any [Iterator].
        ///
        /// Please consult the documentation for more information.
        ///
        /// # Usage
        /// ```
        /// use rollercoaster::Rollercoaster;
        ///
        /// // Now we can use the extra
        /// // iterator extensions on any iterator.
        /// let result: Vec<_> = [1, 2, 2, 3, 4, 5]
        ///     .into_iter()
        ///     .unique()
        ///     .group_by(|x| *x > 3)
        ///     .map(|g| g.items)
        ///     .collect();
        ///
        /// assert_eq!(result, vec![
        ///     vec![1, 2, 3],
        ///     vec![4, 5]
        /// ])
        /// ```
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
