use hanoi::Towers;
fn main() {
    let mut t = Towers::new(5);
    t.print();
    t.solve();
    t.print();
}
