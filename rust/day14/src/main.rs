use iced::widget::Scrollable;
use iced::widget::canvas::{Canvas, Cursor, Frame, Geometry, Path, Program};
use iced::{Color, Sandbox, Length, Settings, Element, Theme, Rectangle, Size, Point, Vector};

pub fn main() -> iced::Result {
    Grid::run(Settings::default())
}

fn rock_color() -> Color {
    Color::from_rgb8(170, 165, 255)
    // let peach: Color = Color::from_rgb8(255, 170, 165);
}

#[derive(Debug)]
struct Cell {
    x: i32,
    y: i32,
    color: Color,
}

struct Grid {
    cell_size: f32,
    cell_padding: f32,
    cells: Vec<Cell>
}

impl Program<Message> for Grid {
    type State = ();

    fn draw(&self, _state: &(), _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let mut frame = Frame::new(bounds.size());

        for cell in &self.cells {
            let point = Point {x: cell.x as f32 * self.cell_size, y: cell.y as f32 * self.cell_size };
            let padded_point = point - Vector { x: self.cell_padding / 2.0, y: self.cell_padding / 2.0 };
            let rect = Path::rectangle(
                padded_point,
                Size { width: self.cell_size - self.cell_padding, height: self.cell_size - self.cell_padding});
            frame.fill(&rect, cell.color);
        }

        vec![frame.into_geometry()]
    }
}


#[derive(Debug)]
enum Message {}

impl Sandbox for Grid {
    type Message = Message;

    fn new() -> Self {
        dbg!(get_rocks());
        Grid {
            cell_size: 10.0,
            cell_padding: 0.1,
            cells: get_rocks()
        }
    }

    fn title(&self) -> String {
        String::from("Sand sim")
    }

    fn update(&mut self, _message: Message) {

    }

    fn view(&self) -> Element<Message> {
        Scrollable::new(
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
        ).height(Length::Fill).into()
    }
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
            let (x, y): (i32, i32)  = {
                let mut xy = rock.split(',');
                let mut get = || xy.next()
                    .expect("can't get next position")
                    .parse()
                    .expect("can't parse position");
                (get(), get())
            };
            // dbg!(x, y);
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
        .map(|(x, y)| Cell { x, y, color: rock_color() })
        .collect()
}