use std::ops::Add;

pub trait Example {
    const CONST_NO_DEFAULT: i32;
    const CONST_WITH_DEFAULT: i8 = 2;
    type TypeNoDefault;

    fn method_with_default()-> i8{ 1 }
    fn method_without_default<T: Example>(&self) -> i8;
}

pub struct eg_struct{
    for_typeNoDefault: i32,
}

impl Example for eg_struct {
    type TypeNoDefault= i8;
    const CONST_NO_DEFAULT: i32 = 1;
    const CONST_WITH_DEFAULT: i8 = 2;
    
    fn method_with_default()-> i8{
        1
    }

    fn method_without_default<T: Example>(&self) -> i8{
        T::CONST_WITH_DEFAULT
    }
}