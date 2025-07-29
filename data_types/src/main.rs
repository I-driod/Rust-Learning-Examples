/*
We will continue with data types.

Data type tells the compiler what type of data is being used.

Categories:
Scalar Types: Single values, like numbers or characters.

Compound Types: Groups of values, like arrays or tuples. 


i8	8 bits	-128 to 127
u8	8 bits	0 to 255
i16	16 bits	-32,768 to 32,767
u16	16 bits	0 to 65,535
i32	32 bits	-2,147,483,648 to 2,147,483,647
u32	32 bits	0 to 4,294,967,295
i64	64 bits	-9 quintillion to 9 quintillion (approx.)
u64	64 bits	0 to 18 quintillion (approx.)
i128	128 bits	Really huge negative to positive numbers
u128	128 bits	0 to really huge positive numbers


String Slice: (&str) String ref, it's immutable (it can't chnage). 
let name: &str = "your name";


String: is growable, it's stored in a memory you contol, you can change it.
let mut last_name: String = String::from("your last name");


Brrowing lets you acess data without taking ownership. 

Immutable Brrowing (&): read only ability.
Mutable Brrowing (&mut): read and edit ability.


*/






///------///
/// -------------///

fn main() {
//crate an array of your fav sport 
//write a function that aceepts this array and print it.







    let nombres = ["Karate", "powerlifting", "Football"];
    print_names(&nombres);
    








// let miracle_car: String = String::from("Miracle Toyota Camary");

// let victor = miracle_car.clone();

// println!("victor driving: {}", victor);



// let mut get_rusty: String = String::from("Get Rusty");
// let victor: &String = &get_rusty;
// let miracle = &get_rusty;

// println!("victor: {}, Mircale: {}", victor, miracle);

// let victor_write: &mut String = &mut get_rusty;

// victor_write.push_str(" I'm getting to know RUST");


// println!("updated rusty: {}, original book: {}", victor_write, get_rusty);


// let fruits: [&str; 3] = ["Orange", "banana", "Apple"];
// let victor_fruits: &_ = &fruits[1..3];

// println!("victor's fruits: {:#?}", victor_fruits)




// victor.push_str("string");




// let name: &'static str = "Godwin";
// // name.pus
// let mut full_name: String = String::from("Godwin");
// full_name.push_str(" David");

// println!("name: {}", full_name);


    // let number: u32 = -18;

    // println!("my number {}", number)

// let name: = "Godwin David";



// let mut  last_name: String = String::from("David ");
// last_name.push_str("Oyewusi");



// print_name(last_name);


// println!("name: {}", last_name);



}



fn print_names(nombres: &[&str]) {
    println!("Listof name: {:?}", nombres);
    println!("sport 1: {}", nombres[0]);
    println!("Sport 2: {}", nombres[1]);
    println!("Sport 3: {}", nombres[2]);
}


// fn print_name (name: String){
//     println!("name: {}", name);
// }

