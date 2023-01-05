use std::{collections::HashSet};

use eframe::{egui::{self, Ui}, App, Theme, epaint::{Color32, Vec2, Rect}};
use crate::simulation::{Grid, Cell, Sides, Simulation};

const ROCK_COLOR: Color32 = Color32::from_rgb(170, 165, 255);
const SAND_COLOR: Color32 = Color32::from_rgb(219, 165, 7);

pub struct SandSimulation {
    grid: Grid,
    offset: Vec2,
    is_playing: bool,
    grid_size: usize,
    scale: f32,
    fall_step: i32,
    simulation: Simulation,
}

impl SandSimulation {
    pub fn new(cells: HashSet<Cell>, hole: Cell) -> Self {
        let grid = Grid{ cells };
        let grid_size = grid.cells.len();
        let simulation = grid.new_simulation(hole);

        let half = Vec2::new(2.0, 2.0);

        let Sides { left, top, right, bottom } = grid.sides();

        let min_cell = Vec2::new(left as f32, top as f32);
        let max_cell = Vec2::new(right as f32, bottom as f32);
        let half_grid = (max_cell - min_cell) / half;
        let offset = min_cell + half_grid;


        Self {
            grid,
            offset,
            grid_size,
            is_playing: false,
            fall_step: 1,
            scale: 10.0,
            simulation,
        }
    }
}

impl SandSimulation {

    fn paint_cells<'a, T: Iterator<Item = &'a Cell>>(&self, ui: &mut Ui, color: Color32, cells: T) {
        let scale = Vec2::new(self.scale, self.scale);
        let offset = self.offset;
        let half_width = scale / Vec2::new(2.0, 2.0);
        let center = ui.clip_rect().center().to_vec2();

        for cell in cells {
            let point: Vec2 = (cell.x as f32, cell.y as f32).into();

            let aligned = point * scale - offset * scale + center;

            let rect = Rect {
                min: (aligned - half_width).to_pos2(),
                max: (aligned + half_width).to_pos2(),
            };
            ui.painter().rect_filled(rect, 0.0, color);
        }
    }
}

impl App for SandSimulation {

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!(
                "Fallen sand: {}",
                self.simulation.grid.cells.len() - self.grid_size,
            ));

            ui.horizontal(|ui| {
                ui.add(egui::widgets::Slider::new(&mut self.fall_step, 1..=10000));
                if ui.button("Play").clicked() {
                    self.is_playing = !self.is_playing;
                }

                if ui.button("Fall").clicked() || self.is_playing {
                    for _ in 0..self.fall_step {
                        self.simulation.fall();
                    }
                    ctx.request_repaint();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Scale");
                ui.add(egui::widgets::Slider::new(&mut self.scale, 1.0..=20.0));
            });

            if ui.button("Start part 1").clicked() {
                self.simulation = self.grid.new_simulation(self.simulation.hole);
                self.grid_size = self.simulation.grid.cells.len();
            }

            if ui.button("Start part 2").clicked() {
                self.simulation = self.grid.new_simulation(self.simulation.hole);

                let Sides { mut left, mut right, mut bottom, .. } = self.simulation.grid.sides();
                let side_offset = 10000;
                left = left - side_offset;
                right = right + side_offset;
                bottom = bottom + 2;

                for x in left..=right {
                    self.simulation.grid.add(Cell { x, y: bottom });
                }

                self.grid_size = self.simulation.grid.cells.len();
            }

            if let Some(falling) = self.simulation.falling {
                self.paint_cells(ui, SAND_COLOR, [falling].iter());
            }

            self.paint_cells(ui, ROCK_COLOR, self.simulation.grid.cells.iter());
        });
    }
}

pub fn main(cells: HashSet<Cell>, hole: Cell) {
    let options = eframe::NativeOptions {
        default_theme: Theme::Light,
        ..Default::default()
    };

    let sim = SandSimulation::new(cells, hole);
    eframe::run_native(
        "Sand simulation",
        options,
        Box::new(|_| Box::new(sim))
    );
}
