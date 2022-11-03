fn main() {
    create_block(3);
}

fn create_block(arg: u32) {
    if arg < 1 {
        return
    }
    for n in 1..=arg {
        let block = "#";
        println!("{}", block.repeat(n as usize));
    }
}
