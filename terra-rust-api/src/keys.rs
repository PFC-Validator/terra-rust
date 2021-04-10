mod address;

use crate::errors::Result;
trait Key {
    fn new() -> Self;
    fn sign(payload: &Vec<u8>) -> Result<Vec<u8>>;
}
