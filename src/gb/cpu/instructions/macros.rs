macro_rules! opcode {
    ($fn:ident ( $($arg:ident : $targ:ty),* ) { $($code:tt)* } $($inner_fn:ident ( $($inner_arg:ident : $inner_targ:ty),* ) $inner_code:block)* ) => {
        pub(super) fn $fn($($arg:$targ),* ) -> Instruction {
            $(fn $inner_fn($($inner_arg:$inner_targ),* ) -> Instruction $inner_code)*
            $($code)*
        }
    };
}
