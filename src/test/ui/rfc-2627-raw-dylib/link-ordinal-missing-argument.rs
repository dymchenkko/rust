#![feature(raw_dylib)]
//~^ WARN the feature `raw_dylib` is incomplete

#[link(name = "foo")]
extern "C" {
    #[link_ordinal()]
    //~^ ERROR incorrect number of arguments to `#[link_ordinal]`
    fn foo();
    #[link_ordinal()]
    //~^ ERROR incorrect number of arguments to `#[link_ordinal]`
    static mut imported_variable: i32;
}

fn main() {}
