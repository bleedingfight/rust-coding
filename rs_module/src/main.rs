use crate::garden::vegetables;
// 引入garden模块
pub mod garden;
fn main() {
    let a = vegetables::Asparagus{
        price:10.0f32,
        weight:1.0f32,
    };
    println!("A = {:#?}",a);
}
