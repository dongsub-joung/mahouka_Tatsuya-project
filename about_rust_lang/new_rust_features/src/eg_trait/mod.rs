
pub trait Example {
    // "located in Stack or CPU registers as inline" Google Gemini notice  
    const CONST_NO_DEFAULT: i32;
    const CONST_WITH_DEFAULT: i8 = 2;
    type TypeNoDefault;
    
    fn method_with_default()-> i8{ 1 }
    fn method_without_default<T: Example>(&self) -> i8;
}

pub struct EgStruct{
    for_type_no_default: i32,
}

impl Example for EgStruct {
    type TypeNoDefault= i8;
    const CONST_NO_DEFAULT: i32 = 1;
    // const CONST_WITH_DEFAULT: i8 = 2;  not required/optional


    fn method_with_default()-> i8{
        1
    }

    fn method_without_default<T: Example>(&self) -> i8{
        T::CONST_WITH_DEFAULT
    }
}
