use eframe::{egui, CreationContext, App, Theme, epaint::{Color32, Vec2, Rect} };

#[derive(Debug)]
enum CellType {
    Sand,
    Ground,
}

#[derive(Debug)]
struct Cell {
    x: i32,
    y: i32,
    t: CellType
}

impl Cell {
    const fn color(&self) -> Color32 {
        match self.t {
            CellType::Sand => Color32::from_rgb(170, 165, 255),
            CellType::Ground => Color32::from_rgb(170, 165, 255),
        }
    }
}

struct Grid {
    cells: Vec<Cell>
}

fn get_rocks() -> Vec<Cell> {
    let mut cells = std::collections::HashSet::new();
    let input_path = std::env::args().nth(1).unwrap_or(
        "input.txt".into()
    );
    let input = std::fs::read_to_string(input_path).expect("Can't read file");

    for structure in input.split('\n') {
        let mut prev = None;
        for rock in structure.split(" -> ") {
            let (x, y) = {
                let mut xy = rock.split(',');
                let mut get = || xy.next()
                    .expect("can't get next position")
                    .parse()
                    .expect("can't parse position");
                (get(), get())
            };

            if let Some((px, py)) = prev {
                use std::cmp::{min, max};
                let (from_x, to_x) = (min(px, x), max(px, x));
                let (from_y, to_y) = (min(py, y), max(py, y));

                for rock_x in from_x..=to_x {
                    for rock_y in from_y..=to_y {
                        cells.insert((rock_x, rock_y));
                    }
                }
            }

            prev = Some((x, y));
        }
    }

    cells
        .into_iter()
        .map(|(x, y)| Cell { x, y, t: CellType::Ground })
        .collect()
}


struct SandSimulation {
    grid: Grid,
    scale: Vec2,
}

impl SandSimulation {
    fn new(_: &CreationContext) -> Self {
        Self {
            grid: Grid{ cells: get_rocks() },
            scale: Vec2::new(10.0, 10.0),
        }
    }
}

impl App for SandSimulation {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let scale = self.scale;
            let half = Vec2::new(2.0, 2.0);

            let mut min_x = f32::MAX;
            let mut max_x = f32::MIN;
            let mut min_y = f32::MAX;
            let mut max_y = f32::MIN;

            for cell in &self.grid.cells {
                min_x = (cell.x as f32).min(min_x);
                max_x = (cell.x as f32).max(max_x);
                min_y = (cell.y as f32).min(min_y);
                max_y = (cell.y as f32).max(max_y);
            }

            let min_cell = Vec2::new(min_x, min_y) * scale;
            let max_cell = Vec2::new(max_x, max_y) * scale;
            let half_grid = (max_cell - min_cell) / half;
            let half_width = scale / half;

            let center = ui.clip_rect().center().to_vec2();
            for cell in &self.grid.cells {
                let point: Vec2 = (cell.x as f32, cell.y as f32).into();

                let aligned = point * scale - min_cell - half_grid + center;

                let rect = Rect {
                    min: (aligned - half_width).to_pos2(),
                    max: (aligned + half_width).to_pos2(),
                };

                ui.painter().rect_filled(rect, 0.0, cell.color());
            }
        });
    }
}


fn main() {
    let options = eframe::NativeOptions {
        default_theme: Theme::Light,
        ..Default::default()
    };

    eframe::run_native(
        "Sand simulation",
        options,
        Box::new(|cc| Box::new(SandSimulation::new(cc)))
    );
}