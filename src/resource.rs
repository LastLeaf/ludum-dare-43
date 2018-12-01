use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use glayout::canvas::CanvasConfig;
use glayout::canvas::element::{ImageLoader, ImageLoaderStatus};

pub struct ResourceLoader {
    cfg: Rc<CanvasConfig>,
    images: HashMap<&'static str, Rc<RefCell<ImageLoader>>>,
    wait_to: Instant,
}

impl ResourceLoader {
    pub fn new(cfg: Rc<CanvasConfig>) -> Self {
        Self {
            cfg,
            images: HashMap::new(),
            wait_to: Instant::now(),
        }
    }
    pub fn load_image(&mut self, name: &'static str, url: &'static str) {
        let loader = Rc::new(RefCell::new(ImageLoader::new_with_canvas_config(self.cfg.clone())));
        ImageLoader::load(loader.clone(), url);
        self.images.insert(name, loader);
    }
    pub fn wait(&mut self, dur: Duration) {
        self.wait_to += dur;
    }
    pub fn ended<F>(self, f: F) where F: 'static + Fn(Resource) {
        frame!(move |_| {
            if self.wait_to > Instant::now() {
                return true;
            }
            for loader in self.images.iter() {
                if loader.1.borrow().status() != ImageLoaderStatus::Loaded {
                    return true;
                }
            }
            unsafe { super::hide_loading() };
            f(Resource {
                images: self.images.clone(),
            });
            return false;
        });
    }
}

#[derive(Clone)]
pub struct Resource {
    images: HashMap<&'static str, Rc<RefCell<ImageLoader>>>
}

impl Resource {
    pub fn image(&self, name: &'static str) -> Rc<RefCell<ImageLoader>> {
        self.images[name].clone()
    }
}
