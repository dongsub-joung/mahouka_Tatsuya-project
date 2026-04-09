mod eg_trait;
mod eg_generictraits;

use crate::eg_trait::*;
use crate::eg_generictraits::*;


fn get_one<T:eg_trait::Example>()-> i32{
    T::CONST_NO_DEFAULT
}

fn main(){
    let status_one= get_one::<eg_struct>();
}
