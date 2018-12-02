use std::rc::Rc;
use std::cell::RefCell;
use glayout::{canvas};
use glayout::canvas::element::{Event, Element, Empty, Text, Image};
use glayout::canvas::element::style::{PositionType, DisplayType};

pub struct Cover {}

fn start_level(context: &Rc<RefCell<canvas::CanvasContext>>, resource: &super::resource::Resource) {
    let ctx = context.clone();
    ctx.borrow_mut().root().remove(0);
    super::level::Level::show(context.clone(), resource.clone(), 0, super::level::RedSkills {
        ice: false,
        bite: false,
        fire: false,
    });
}

impl Cover {
    pub fn show(context: Rc<RefCell<canvas::CanvasContext>>, resource: super::resource::Resource, head: &'static str) {
        let mut ctx = context.borrow_mut();
        let cfg = ctx.canvas_config();
        let mut root = ctx.root();

        let ctx_clone_1 = context.clone();
        let resource_clone_1 = resource.clone();

        let cover = element!(&cfg, Empty {
            Image {
                position: PositionType::Absolute;
                left: 420.;
                top: 300.;
                width: 120.;
                height: 120.;
                set_loader(resource.image(head));
            };
            Empty {
                position: PositionType::Absolute;
                left: 450.;
                top: 450.;
                color: (0.7, 0.5, 0.5, 1.);
                Text {
                    display: DisplayType::Block;
                    font_size: 30.;
                    line_height: 50.;
                    set_text("Little Red");
                };
            };
            Empty {
                position: PositionType::Absolute;
                left: 450.;
                top: 500.;
                width: 80.;
                height: 30.;
                color: (0.2, 0.2, 0.2, 1.);
                background_color: (0.7, 0.7, 0.7, 1.);
                Text {
                    display: DisplayType::Block;
                    font_size: 20.;
                    line_height: 30.;
                    set_text(" Play > ");
                };
                @ "touchend" => move |_: &Event| {
                    start_level(&ctx_clone_1, &resource_clone_1);
                };
            };
            Empty {
                position: PositionType::Absolute;
                left: 450.;
                top: 650.;
                width: 100.;
                height: 30.;
                color: (0.5, 0.5, 0.5, 1.);
                Text {
                    display: DisplayType::Block;
                    font_size: 16.;
                    line_height: 30.;
                    set_text("Created by LastLeaf with ♥ for Ludum Dare 43");
                };
            };
        });
        root.append(cover);
    }
}
