mod foo;
mod util;
// mod foo1;
fn main() {
    let mut person = util::Point { x: 40.0, y: 30.0 };
    foo::foo::bar::foo_bar();
    foo::foo1::bar::foo_bar();
    util::Point::move_pos(&mut person)
}
