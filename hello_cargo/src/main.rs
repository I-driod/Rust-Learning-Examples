// Lesson-one : About cargo,main src file,cargo.toml and running first program refrence to hello-world folder
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/*
Cargo is Rut's helper tool that makes starting, building, and running project easy.It's like a project manager that sets up your workplace and 
handles task for you.


src/main.rs: this is where you ypur code.
Cargo.toml: this is where you put your dependencies and description of your project.
*/


//Lessson-two : Data-Types in rust I.
//^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

/*

Data Type

Numbers,
 - assigned integers and unsigned intergers 
 i8, u8,
 2^7 = 128, -2 ^7 - 1 = 127
 i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
 - assigned floating points
 f32, f64
 -  booleans
 true, false
 -  characters
 char
String,
Array,
Tuple,



*/
//condition statment 

/*

> is grater than
< is less than
>= is grater than or equal to
<= is less than or equal to
== is equal to
!= is not equal to
*/






fn main() {
    let number:i32 = 6;
 let result = is_grater(number);

 let my_char = '😀';

println!("Our result is {} {}", result, my_char)

}



//function that accept a number and check if that number is equal to five 
fn is_grater(num: i32)  -> bool{
   num != 5
}




