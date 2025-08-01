use core::time;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};


/*
Control Flow
a statement that tells the program to excute based on a certain condition.


1. if/else
2. match
3. loop
4. while
5. For
*/

fn main() {
    // let mut number:i32 = 3;

    // if number  >= 5 {
    //     number = 5 + number;
    // } else {
    //   number -= 1;
    // }

    // println!("number: {}", number);

    // let tracfic = "black";

    // trafic_light(tracfic);

    // let mut count = 0u32;

    // loop {

    //     count += 1;
    //     // count = count + 1;
    //     if count == 3 {
    //      println!("count 3 as you  can see. Count: {}" , count);
    //      continue;
    //     }
    //     else if count == 5 {
    //         println!("count has ended. Count: {}", count);
    //         break;
    //     }

    //     println!("looping {}", count);

    // }

    // println!("---- 🕹️ Counter Stopper ----");
    // println!("Press ENTER when the countdown hits 0 to win!");

    //  let mut count = 10;
    // enable_raw_mode().unwrap();

    // while count >= 0 {
    //     println!("Count: {}", count);
    //     let start = Instant::now();
    //     let mut pressed = false;

    //     // Wait for 1 second OR until Enter is pressed
    //     while start.elapsed() < Duration::from_secs(1) {
    //         if event::poll(Duration::from_millis(100)).unwrap() {
    //             if let Event::Key(event) = event::read().unwrap() {
    //                 if event.code == KeyCode::Enter {
    //                     pressed = true;
    //                     break;
    //                 }
    //             }
    //         }
    //     }

    //     // Handle game logic based on current count
    //     if count == 0 {
    //         if pressed {
    //             println!("🎉 You pressed Enter right on time");
    //         } else {
    //             println!("You didn't press Enter in time ⌛");
    //         }
    //         break;
    //     } else if pressed {
    //         println!("Count down is still running ⏱️");
    //         break;
    //     }

    //     count -= 1;
    // }
    
    // disable_raw_mode().unwrap();
    // println!("---- 🕹️ Counter Stopper. GAME OVER----");

    let mut students = ["Miracle", "Vector", "Favour"];

    // for name in students.iter(){
    //     match name {
    //         &"Wilson" => println!("{name} Miracled found"),
    //         _ => println!("not found")
    //     }
    // }

    for name in students.into_iter() {
        match name {
            "Miracle" => println!("{name} Miracled found"),
            _ => println!("not found")
        }

        println!("students: {:?}", students);

    }
}

//Assignment: use a loop inside another loop and break out of the inner loop when a condition you have set have been meet. the break out of the outer loop when another contion has been meet.
//"outer" andf "inner" make sure to use this.

// fn trafic_light (color:&str) {
//     // if color == "green" {
//     //     println!("go");
//     // } else if color == "yellow" {
//     //     println!("slow down");
//     // }
//     // else if color == "red" {
//     //     println!("stop");
//     // }

//     // else {
//     //     println!("in valid color");
//     // }

//     match color {
//         "green"=> println!("go"),
//         "yellow" => println!("slow down"),
//         "red" => println!("stop"),
//         _ => println!("invalid color")
//     }

// }

/*
> - grater than
< - less than
>= - grater than or equal to
<= - less than or equal to
== - equal to
!= - not equal to

*/
