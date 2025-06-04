use iced::widget::canvas::{self, LineCap, LineJoin};
use iced::{
    Color, Point, Rectangle, Renderer, Theme,
    alignment::{Horizontal as HorizontalAlignment, Vertical as VerticalAlignment},
};
use iced::{Element, mouse};

use crate::run_application::run_ui::{State, SudokuCanvasMessage};

#[derive(Debug, Clone)]
pub(super) struct Sudoku {
    objects: Vec<SudokuObject>,
    //mouse_interactables: Vec<MouseInteractable>
}

impl Sudoku {
    fn bounds(&self) -> Bounds {
        self.objects.iter().fold(
            Bounds {
                x_min: 0.0,
                x_max: 0.0,
                y_min: 0.0,
                y_max: 0.0,
            },
            |mut bounds, current| {
                bounds.include_bounds(current.bounds());
                bounds
            },
        )
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Bounds {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

impl Bounds {
    fn include_point(&mut self, point: Point<f32>, size: f32) {
        self.include_bounds(Bounds {
            x_min: point.x - size / 2.0,
            x_max: point.x + size / 2.0,
            y_min: point.y - size / 2.0,
            y_max: point.y + size / 2.0,
        });
    }

    fn include_bounds(&mut self, bounds: Bounds) {
        if self.x_max < bounds.x_max {
            self.x_max = bounds.x_max;
        }
        if self.y_max < bounds.y_max {
            self.y_max = bounds.y_max;
        }
        if self.x_min > bounds.x_min {
            self.x_min = bounds.x_min;
        }
        if self.y_min > bounds.y_min {
            self.y_min = bounds.y_min;
        }
    }
}

#[derive(Debug, Clone)]
struct MouseInteractable {}

#[derive(Debug, Clone)]
enum SudokuObject {
    Line {
        points: Vec<Point>,
        size: f32,
        line_cap: LineCap,
        line_join: LineJoin,
        color: Color,
    },
    Polygon {
        points: Vec<Point>,
        border_size: f32,
        border_color: Color,
        fill_color: Option<Color>,
    },
    Circle {
        center: Point,
        radius: f32,
        border_size: f32,
        border_color: Color,
        fill_color: Option<Color>,
    },
    Image {
        center: Point,
        width: f32,
        height: f32,
        value: canvas::Image,
    },
    Text {
        position: Point,
        centered: bool,
        size: f32,
        color: Color,
        value: String,
    },
}

impl SudokuObject {
    fn bounds(&self) -> Bounds {
        match self {
            SudokuObject::Line {
                points,
                size,
                color: _,
                line_cap: _,
                line_join: _,
            } => {
                let mut bounds = Bounds::default();
                for point in points {
                    bounds.include_point(*point, *size);
                }
                bounds
            }
            SudokuObject::Polygon {
                points,
                border_size,
                border_color: _,
                fill_color: _,
            } => {
                let mut bounds = Bounds::default();
                for point in points {
                    bounds.include_point(*point, *border_size);
                }
                bounds
            }
            SudokuObject::Circle {
                center,
                radius,
                border_size,
                border_color: _,
                fill_color: _,
            } => {
                let mut bounds = Bounds::default();
                bounds.include_point(*center, *border_size + *radius * 2.0);
                bounds
            }
            SudokuObject::Image {
                center,
                width,
                height,
                value: _,
            } => Bounds {
                x_min: center.x - *width / 2.0,
                x_max: center.x + *width / 2.0,
                y_min: center.y - *height / 2.0,
                y_max: center.y + *height / 2.0,
            },
            #[allow(clippy::cast_precision_loss)]
            SudokuObject::Text {
                position,
                size,
                value,
                centered: _,
                color: _,
            } => Bounds {
                x_min: position.x - *size * value.len() as f32 / 2.0,
                x_max: position.x + *size * value.len() as f32 / 2.0,
                y_min: position.y - *size / 2.0,
                y_max: position.y + *size / 2.0,
            },
        }
    }
}

impl canvas::Program<SudokuCanvasMessage> for Sudoku {
    fn update(
        &self,
        _state: &mut Self::State,
        _event: canvas::Event,
        _bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<SudokuCanvasMessage>) {
        (canvas::event::Status::Ignored, None)
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }

    type State = ();

    #[allow(clippy::too_many_lines)]
    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let (size_factor, x_offset, y_offset) = {
            let sudoku_bounds = self.bounds();
            let x_div = bounds.width / (sudoku_bounds.x_max - sudoku_bounds.x_min);
            let y_div = bounds.height / (sudoku_bounds.y_max - sudoku_bounds.y_min);
            if x_div < y_div {
                (
                    x_div,
                    -sudoku_bounds.x_min * x_div,
                    (bounds.height - (sudoku_bounds.y_max - sudoku_bounds.y_min) * x_div) / 2.0
                        - sudoku_bounds.y_min * y_div,
                )
            } else {
                (
                    y_div,
                    (bounds.width - (sudoku_bounds.x_max - sudoku_bounds.x_min) * y_div) / 2.0
                        - sudoku_bounds.x_min * x_div,
                    -sudoku_bounds.y_min * y_div,
                )
            }
        };

        let transform_point = |point: Point<f32>| {
            Point::new(
                x_offset + size_factor * point.x,
                y_offset + size_factor * point.y,
            )
        };

        self.objects.iter().for_each(|object| match object {
            SudokuObject::Line {
                points,
                size,
                color,
                line_cap,
                line_join,
            } => {
                let points: Vec<Point> =
                    points.iter().map(|point| transform_point(*point)).collect();
                let size = size * size_factor;
                if let Some((first, rest)) = points.split_first() {
                    let mut line = canvas::path::Builder::new();
                    line.move_to(*first);
                    for point in rest {
                        line.line_to(*point);
                    }
                    let line = line.build();
                    frame.stroke(
                        &line,
                        canvas::stroke::Stroke::default()
                            .with_color(*color)
                            .with_width(size)
                            .with_line_cap(*line_cap)
                            .with_line_join(*line_join),
                    );
                }
            }
            SudokuObject::Polygon {
                points,
                border_size,
                border_color,
                fill_color,
            } => {
                let points: Vec<Point> =
                    points.iter().map(|point| transform_point(*point)).collect();
                let border_size = border_size * size_factor;
                if let Some((first, rest)) = points.split_first() {
                    let mut polygon = canvas::path::Builder::new();
                    polygon.move_to(*first);
                    for point in rest {
                        polygon.line_to(*point);
                    }
                    polygon.close();
                    let polygon = polygon.build();
                    if let Some(color) = fill_color {
                        frame.fill(&polygon, *color);
                    }
                    frame.stroke(
                        &polygon,
                        canvas::stroke::Stroke::default()
                            .with_color(*border_color)
                            .with_width(border_size),
                    );
                }
            }
            SudokuObject::Circle {
                center,
                radius,
                border_size,
                border_color,
                fill_color,
            } => {
                let center: Point = transform_point(*center);
                let radius = radius * size_factor;
                let border_size = border_size * size_factor;

                let circle = canvas::Path::circle(center, radius);
                if let Some(color) = fill_color {
                    frame.fill(&circle, *color);
                }
                frame.stroke(
                    &circle,
                    canvas::stroke::Stroke::default()
                        .with_color(*border_color)
                        .with_width(border_size),
                );
            }
            SudokuObject::Image {
                center: _,
                width: _,
                height: _,
                value: _,
            } => todo!(),
            SudokuObject::Text {
                size,
                value,
                position,
                centered,
                color,
            } => {
                let position: Point = transform_point(*position);
                let size = size * size_factor;

                let text = canvas::Text {
                    content: value.clone(),
                    position,
                    color: *color,
                    size: iced::Pixels(size),
                    line_height: iced::widget::text::LineHeight::Relative(0.1),
                    font: iced::Font::DEFAULT, //TODO: Consider using MONOSPACE instead
                    horizontal_alignment: if *centered {
                        HorizontalAlignment::Center
                    } else {
                        HorizontalAlignment::Left
                    },
                    vertical_alignment: if *centered {
                        VerticalAlignment::Center
                    } else {
                        VerticalAlignment::Top
                    },
                    shaping: iced::widget::text::Shaping::Basic,
                };

                frame.fill_text(text);
            }
        });
        vec![frame.into_geometry()]
    }
}

pub(super) fn view(_state: &State) -> Element<'_, SudokuCanvasMessage> {
    let sudoku = example_sudoku();
    canvas::Canvas::new(sudoku)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .into()
}

fn example_sudoku() -> Sudoku {
    Sudoku {
        objects: {
            let mut objects: Vec<SudokuObject> = vec![];
            objects.push(SudokuObject::Line {
                points: vec![
                    Point::new(0.5, 2.5),
                    Point::new(1.5, 1.5),
                    Point::new(2.5, 1.5),
                    Point::new(2.5, 0.5),
                    Point::new(3.5, 1.5),
                ],
                size: 0.4,
                color: Color::from_rgb8(0, 255, 0),
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
            });
            for i in 0..9 {
                objects.append(
                    &mut (0..9)
                        .map(
                            #[allow(clippy::cast_precision_loss)]
                            |ii: i32| {
                                let x = (ii % 3 + (i % 3) * 3) as f32;
                                let y = (ii / 3 + (i / 3) * 3) as f32;
                                SudokuObject::Polygon {
                                    points: vec![
                                        Point::new(x + 1.0, y + 1.0),
                                        Point::new(x, y + 1.0),
                                        Point::new(x, y),
                                        Point::new(x + 1.0, y),
                                    ],
                                    border_size: 0.05,
                                    border_color: Color::BLACK,
                                    fill_color: if (i * 3 + (i / 3) + ii) % 9 + 1 == 9 {
                                        Some(Color::from_rgb8(200, 200, 200))
                                    } else {
                                        None
                                    },
                                }
                            },
                        )
                        .collect(),
                );
            }
            objects.append(
                &mut (0..9)
                    .map(
                        #[allow(clippy::cast_precision_loss)]
                        |i| {
                            let x = (i % 3) as f32 * 3.0;
                            let y = (i / 3) as f32 * 3.0;
                            SudokuObject::Polygon {
                                points: vec![
                                    Point::new(x + 3.0, y + 3.0),
                                    Point::new(x, y + 3.0),
                                    Point::new(x, y),
                                    Point::new(x + 3.0, y),
                                ],
                                border_size: 0.1,
                                border_color: Color::BLACK,
                                fill_color: None,
                            }
                        },
                    )
                    .collect(),
            );
            for i in 0..9 {
                objects.append(
                    &mut (0..9)
                        .map(
                            #[allow(clippy::cast_precision_loss)]
                            |ii: i32| {
                                let x = (ii % 3 + (i % 3) * 3) as f32;
                                let y = (ii / 3 + (i / 3) * 3) as f32;
                                SudokuObject::Text {
                                    position: Point {
                                        x: x + 0.5,
                                        y: y + 0.5,
                                    },
                                    centered: true,
                                    size: 0.8,
                                    color: Color::from_rgb8(0, 0, 200),
                                    value: format!("{}", (i * 3 + (i / 3) + ii) % 9 + 1),
                                }
                            },
                        )
                        .collect(),
                );
            }
            objects
        },
    }
}
