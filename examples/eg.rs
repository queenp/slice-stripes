extern crate slice_stripes;

use slice_stripes::Striped;

fn main() {
    let x = [1,2,3,
             4,5,6,
             7,8,9];

    println!("{:?}",x.stripes(3).next().unwrap().collect::<Vec<u8>>());
}
