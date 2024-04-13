// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.



use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";
    // ?操作的设计是提前进行return结果，所以只有return值为Option和Result才能用
    // let cost = total_cost(pretend_user_input)?;
    let cost: i32 = 0;
    if let Ok(ref cost) = total_cost(pretend_user_input){};

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
