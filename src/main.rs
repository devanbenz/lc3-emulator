use hardware::{Memory, instructions::decode_operation};

mod hardware;

fn main() {
    decode_operation(0b1111_0000_0000_0000);
}
