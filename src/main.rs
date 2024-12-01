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

fn calculate_total_price(item_count: i32, price_per_item: i32) -> i32 {
    // TODO: Implement the calculation for the total price using item_count and price_per_item
    
    item_count * price_per_item
}

fn main() {
    let pies_count = 3;
    let price_per_pie = 15;
    // TODO: Call your function here to calculate the total price for pies, given the quantity and price per pie.
    let pies_price = calculate_total_price (pies_count,price_per_pie);
    println!("Total price for pies: ${}", pies_price);
}


