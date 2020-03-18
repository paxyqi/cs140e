// FIXME: Make me compile! Diff budget: 1 line.

struct MyType(usize);
impl Clone for MyType{
    fn clone(&self) -> Self{
        Self(self.0)
        }
    }

impl Copy for MyType{}
// Do not modify this function.
pub fn main() {
    let x = MyType(1);
    let y = &x;
    let z = *y;
}
