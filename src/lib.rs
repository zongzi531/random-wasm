extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use rand;

#[wasm_bindgen]
pub fn random() -> f64 {
    rand::random()
}

#[cfg(test)]
mod tests {
    use crate::random;

    #[test]
    fn it_works() {
        let res1 = random();
        let res2 = random();
        assert_ne!(res1, res2);
    }
}
