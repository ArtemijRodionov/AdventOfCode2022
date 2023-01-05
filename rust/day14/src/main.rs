pub mod ui;
pub mod simulation;

fn main() {
    let cells = crate::simulation::get_rocks();
    crate::ui::main(cells, crate::simulation::Cell { x: 500, y: 0 });
}
