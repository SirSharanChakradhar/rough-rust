fn bake_cookies(batch_count: i32) {
    println!("Baking {} batches cookies!", batch_count);
}

fn sell_cookies(batch_count: i32, cookies_per_batch: i32) -> i32 {
    let total = batch_count * cookies_per_batch;

    println!("{}", total); // Error on this line
    total
}

fn main() {
    bake_cookies(3); // Assuming each batch has the same number of cookies
    let total_cookies = sell_cookies(3, 10); // Selling 3 batches, 10 cookies each
    println!("Total cookies sold: {}", total_cookies);
}


