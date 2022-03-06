mod oscilloscope {
    // use iced_native::layout::{self, Layout};
    // use iced_native::renderer;
    // use iced_native::{Color, Element, Length, Point, Rectangle, Size, Widget};
    use arc_swap::ArcSwap;
    use std::sync::Arc;
    use iced::{
        canvas::{self, Cache, Canvas, Cursor, Geometry, LineCap, Path, Stroke},
        executor, Color, Command, Container, Element, Length, Point, Rectangle,
        Settings, Subscription, Vector,
    };

    #[derive(Default)]
    pub struct Oscillo {
        buffer: Vec<i8>,
        color: i8, //dummy
        cache: Cache,
    }

    #[derive(Debug, Clone)]
    enum Message {
        Update(Arc<Vec<f64>>), //for now, only mono
    }

    impl Oscillo {
        // pub fn new() -> Self {
        //     Self{}
        // }
        pub fn update(&mut self, msg: Message) {
            match msg {
                Message::Update(v) => {}
            }
        }
    }

    impl canvas::Program<Message> for Oscillo {
        fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
            let ui = self.cache.draw(bounds.size(), |frame| {
                let mut points_iter = self
                    .buffer
                    .iter()
                    .enumerate()
                    .map(|(i, num)| {
                        Point::new(
                            i as f32 / bounds.width,
                            *num as f32 * bounds.height * 0.5 / i8::MAX as f32,
                        )
                    });

                let path = Path::new(|builder| {
                    builder.move_to(points_iter.next().unwrap());
                    points_iter.for_each(|p| builder.line_to(p));
                });

                let thin_stroke = Stroke {
                    width: 2.0,
                    color: Color::BLACK,
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                };
                frame.with_save(|frame| frame.stroke(&path, thin_stroke))
            });
            vec![ui]
        }
    }
}
