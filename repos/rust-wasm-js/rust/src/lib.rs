pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_potatoes(amount: u32) -> u32 {
    amount * 42


}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn calculates_potatoes() {
//         let mut result = calculate_potatoes(10);
//         assert_eq!(result, 420);

//         result = 500;
//         assert_eq!(result, 500);
//     }
// }
