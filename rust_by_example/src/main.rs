//we arw going to solve the following code 
#![allow(unused_variables)]
fn main() {

let width = 4i32;
let height = 5.3;
let area = calculate_area(width, height);

println!("Area: {}", area);



//calculate text lenght

let text1 = String::from("Victor");
let text2 = text1;

//todo: make calculate_lenght to print text1
    calculate_lenght(text1)

}



fn calculate_lenght(text: String){
    println!("Lenght of text is {}", text.len());
}

///todo: modify the scope of this function so it runs correctly . 
fn calculate_area (width: i32, height: f32) -> f32 {
    width  * height
}


