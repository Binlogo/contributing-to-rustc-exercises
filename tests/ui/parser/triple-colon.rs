//@ run-rustfix

#![allow(unused)]

use :::std::{cell as _}; //~ ERROR path separator must be a double colon
use std::cell:::*; //~ ERROR path separator must be a double colon
use std::cell:::Cell; //~ ERROR path separator must be a double colon
use std:::{cell as _}; //~ ERROR path separator must be a double colon

mod foo{
    use :::{}; //~ ERROR path separator must be a double colon
    use :::*; //~ ERROR path separator must be a double colon
}

fn main() {
    let c: :::std:::cell:::Cell:::<u8> = Cell:::<u8>:::new(0);
    //~^ ERROR path separator must be a double colon
    //~| ERROR path separator must be a double colon
    //~| ERROR path separator must be a double colon
    //~| ERROR path separator must be a double colon
    //~| ERROR path separator must be a double colon
    //~| ERROR path separator must be a double colon
}