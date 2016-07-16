macro_rules! itemize(($($chunk:item)*) => ($($chunk)*));

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
    ($($argument:tt)+) => (raise!(format!($($argument)+)));
);

macro_rules! read_array(
    (@common $tape:ident, $count:expr) => ({
        let mut array: [u8; $count] = ::std::mem::uninitialized();
        if try!(::std::io::Read::read($tape, &mut array)) != $count {
            return raise!("failed to read as much as needed");
        }
        array
    });
    ($tape:ident, $count:expr, i8) => (unsafe {
        ::std::mem::transmute(read_array!(@common $tape, $count))
    });
    ($tape:ident, $count:expr, u8) => (unsafe {
        read_array!(@common $tape, $count)
    });
);

macro_rules! read_bytes(
    ($tape:ident, $count:expr) => (unsafe {
        let count = $count as usize;
        let mut values = Vec::with_capacity(count);
        values.set_len(count);
        if try!(::std::io::Read::read($tape, &mut values)) != count {
            return raise!("failed to read as much as needed");
        }
        values
    });
);

macro_rules! read_field(
    ($structure:ident, $tape:ident, $table:ident,
     [$kind:ty] |$pipe:ident, $chair:ident| $body:block) => ({
        #[inline(always)]
        #[allow(unused_variables)]
        fn read<T: ::Tape>($pipe: &mut T, $chair: &$structure) -> ::Result<$kind> $body
        try!(read($tape, &$table))
    });
    ($structure:ident, $tape:ident, $table:expr, [$kind:ty]) => (read_value!($tape));
);

macro_rules! read_value(
    ($tape:ident) => (try!(::Value::read($tape)));
    ($tape:ident, $kind:ty) => (try!(<$kind as ::Value>::read($tape)));
);

macro_rules! read_walue(
    ($tape:ident, $parameter:expr) => (try!(::Walue::read($tape, $parameter)));
    ($tape:ident, $parameter:expr, $kind:ty) => ({
        try!(<$kind as ::Walue<_>>::read($tape, $parameter))
    });
);

macro_rules! table {
    ($(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)+
    }) => (
        table! { @define $(#[$attribute])* pub $structure { $($field ($($kind)+),)+ } }
        table! { @implement pub $structure { $($field ($($kind)+) $(|$($argument),+| $body)*,)+ } }
    );
    (@define $(#[$attribute:meta])* pub $structure:ident {
        $($field:ident ($kind:ty),)+
    }) => (itemize! {
        $(#[$attribute])*
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $structure { $(pub $field: $kind,)+ }
    });
    (@implement pub $structure:ident {
        $($field:ident ($($kind:tt)+) $(|$($argument:ident),+| $body:block)*,)+
    }) => (
        impl ::Value for $structure {
            fn read<T: ::Tape>(tape: &mut T) -> ::Result<Self> {
                let mut table = $structure::default();
                $(
                    table.$field = read_field!($structure, tape, table, [$($kind)+]
                                               $(|$($argument),+| $body)*);
                )+
                Ok(table)
            }
        }
    );
}
