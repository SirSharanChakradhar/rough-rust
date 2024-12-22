// fn bake_cookies(batch_count: i32) {
//     println!("Baking {} batches cookies!", batch_count);
// }

// fn sell_cookies(batch_count: i32, cookies_per_batch: i32) -> i32 {
//     let total = batch_count * cookies_per_batch;

//     println!("{}", total); // Error on this line
//     total
// }

// fn main() {
//     bake_cookies(3); // Assuming each batch has the same number of cookies
//     let total_cookies = sell_cookies(3, 10); // Selling 3 batches, 10 cookies each
//     println!("Total cookies sold: {}", total_cookies);
// }
//----------------------------------------------------------------------------------------------------------------------

//We have baked some delicious rustic pies, so now let's create a function to calculate the total price for these goods.
// Also, make sure to inform the chef of the total number of pies ready to leave the kitchen.

// fn calculate_total_price(item_count: i32, price_per_item: i32) -> i32 {
//     // TODO: Implement the calculation for the total price using item_count and price_per_item
    
//     item_count * price_per_item
// }

// fn main() {
//     let pies_count = 3;
//     let price_per_pie = 15;
//     // TODO: Call your function here to calculate the total price for pies, given the quantity and price per pie.
//     let pies_price = calculate_total_price (pies_count,price_per_pie);
//     println!("Total price for pies: ${}", pies_price);
// }
//----------------------------------------------------------------------------------------------------------------------

//You've practiced modifying and expanding code; now, let's create something from scratch! Your mission is to write a Rust program that calculates the area of a rectangle. Follow the TODO instructions to and use your knowledge of functions to complete this task.
// TODO: Define a function to calculate the area of a field.
// This function should accept 2 arguments for length and width and return the calculated area as a float.

// fn calculate_area (length:f64, breadth:f64) -> f64{
//  length * breadth
       
//   }
  
//   fn main() {
//       // TODO: Call your function with values for length and width and print the result
//    let rect_length: f64 = 5.0;
//    let rect_breadth: f64 = 6.0;
//    let rect_area= calculate_area(rect_length,rect_breadth);
//   println!("The area of the rectangle is: {} square units", rect_area); 
  
//   }

//----------------------------------------------------------------------------------------------------------------------
//Fantastic progress, Space Explorer! Let's dive a bit deeper into Rust's variable scope and shadowing concepts. Your mission is to enhance our small software versioning system by using shadowing to declare a new version number. Provide the missing line of code to shadow the version number and increment it.

// fn main() {
//     let version = 1; // `version` is in scope for the entire function
    
//     // TODO: Shadow the `version` variable by incrementing it and then print the new version in one line
//     let version = version + 1 ;
//     println!("The new version is: {}", version)

// }

//----------------------------------------------------------------------------------------------------------------------
//Modify the given code to apply the correct mutable behavior of the version variable, ensuring it accurately reflects the updated version after the block ends. Your goal is to fix the scope issue so that the incremented version value persists and is correctly reported in the final output.
// fn main() {
//     let mut version = 1;
//     {
//        version= version + 1; // Incrementing the version number.
//         println!("Updated to software version {}", version);
//     }
//     println!("Current software version {}", version); // Incorrect output due to the bug
// }
//----------------------------------------------------------------------------------------------------------------------

// fn main() {
//     let text = "Madduri Rohith ";
//     let repetitions = 9_999_999;

//     // Build the output once to avoid repetitive printing overhead
//     let output = text.repeat(repetitions);

//     // Print the entire string in one go
//     println!("{}", output);
// }
//----------------------------------------------------------------------------------------------------------------------

fn buy_apples(apple_count: &mut i32, apples_bought: i32) {
    *apple_count += apples_bought;
}

fn main() {
    let mut my_apple_count = 5; // Starting with 5 apples
    let shopping_cart = &mut my_apple_count;
    buy_apples(shopping_cart, 10); // Buying 10 more apples
    println!("I had {} apples; now I have {}!", 5, my_apple_count);
}