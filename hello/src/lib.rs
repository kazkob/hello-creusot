#![cfg_attr(not(feature="contracts"), feature(proc_macro_hygiene, stmt_expr_attributes))]
use creusot_contracts::*;

#[ensures(result==42u32)]
pub fn get_the_answer() -> u32 {
    42
}

