#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::f64::consts::PI;
use std::iter::Scan;
use std::ops::Add;

use big_number::BigNumber;
use big_number::BigVec2;
use cooldown::*;
use macroquad::color::Color;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
use macroquad::text::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};
mod algebra_parser;
mod big_number;
mod cooldown;
mod derivative_solver;
type NumberDependency = f64;
static mut SETTINGS_POSITION: Vec2 = vec2(0.0, 0.0);
type CanvasDimensions<'a> = &'a mut NumberDependency;

// Helper Functions
fn area_of_circle(radius: NumberDependency) -> NumberDependency {
    return (PI as NumberDependency) * radius.powf(2.0);
}
fn radius_from_area_of_circle(area: NumberDependency) -> NumberDependency {
    return (area / PI).sqrt();
}
fn negate_vector(vector: Vec2) -> Vec2 {
    vec2(-vector.x, -vector.y)
}
//

struct AppState<'a, 'b> {
    // Settings
    settings_position: &'b mut Vec2,
    resolution_slider_value: &'a mut f32,
    //
    old_screen_width: &'a mut f32,
    old_screen_height: &'a mut f32,
    current_fps: i32,
}
impl<'a, 'b> AppState<'a, 'b> {
    fn update_fps(&mut self) {
        self.current_fps = get_fps();
    }
}
trait ShapeScale {
    fn draw_figure(&self, color: Color);
}
#[derive(Copy, Clone)]
struct Circle {
    radius: NumberDependency,
    x_pos: NumberDependency,
    y_pos: NumberDependency,

    original_width: NumberDependency,
    original_height: NumberDependency,
}
impl ShapeScale for Circle {
    fn draw_figure(&self, color: Color) {
        let screen_width = screen_width() as NumberDependency;
        let screen_height = screen_height() as NumberDependency;
        let old_area = self.original_width * self.original_height;
        let new_area = screen_width * screen_height;
        // println!("{}", radius_from_area_of_circle(((area_of_circle(self.radius) / old_area) * new_area)));
        draw_circle(
            ((self.x_pos / self.original_width) * screen_width) as f32,
            ((self.y_pos / self.original_height) * screen_height) as f32,
            radius_from_area_of_circle(((area_of_circle(self.radius) / old_area) * new_area))
                as f32,
            color,
        );
    }
}
fn create_circle(
    circle_radius: NumberDependency,
    circle_x_pos: NumberDependency,
    circle_y_pos: NumberDependency,
) -> Circle {
    Circle {
        radius: circle_radius,
        x_pos: circle_x_pos,
        y_pos: circle_y_pos,
        original_width: screen_width() as NumberDependency,
        original_height: screen_height() as NumberDependency,
    }
}
struct CircleCache {
    cache: Vec<Circle>,
}
impl CircleCache {
    fn push(&mut self, new_circle: Circle) -> bool {
        let index = self.cache.iter().position(|&item| {
            (item.radius == new_circle.radius)
                && (item.x_pos == new_circle.x_pos)
                && (item.y_pos == new_circle.y_pos)
        });
        let mut flag = false;
        match index {
            Some(_t) => {
                // println!("Already inserted circle into cache!");
            }
            None => {
                flag = true;
                self.cache.push(new_circle.clone());
            }
        }
        flag
    }
    fn execute_drawings(&self) {
        for circle in self.cache.iter() {
            circle.draw_figure(BLACK);
        }
    }
}
fn create_ui(global_state: &mut AppState) {
    let (font_size, font_scale, font_aspect) = camera_font_scale(70.0);
    let params = TextParams {
        font_size,
        font_scale,
        font_scale_aspect: font_aspect,
        color: BLACK,
        ..Default::default()
    };
    draw_text_ex(
        global_state.current_fps.to_string().as_str(),
        screen_width() / 12.0,
        screen_height() / 12.0,
        params,
    );
    widgets::Window::new(hash!(), *global_state.settings_position, vec2(320.0, 400.0))
        .label("Settings")
        .movable(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Resolution Slider:");
            ui.slider(
                hash!(),
                "(0 .. 1)",
                0.0f32..1.0f32,
                global_state.resolution_slider_value,
            );
        });
}
fn update_resolution(global_state: &mut AppState) {
    let is_width_too_small = (1920.0 * *global_state.resolution_slider_value) < 500.0;
    let is_height_too_small = (1080.0 * *global_state.resolution_slider_value) < 500.0;
    if ((screen_height() > screen_width()) && is_width_too_small) {
        println!("WIDTH TOO SMALL");
        request_new_screen_size(500.0, (1920.0 / 1080.0) * 500.0);
        return;
    } else if ((screen_width() > screen_height()) && is_height_too_small) {
        println!("HEIGHT TOO SMALL");
        request_new_screen_size((1920.0 / 1080.0) * 500.0, 500.0);
        return;
    }
    request_new_screen_size(
        1920.0 * *global_state.resolution_slider_value,
        1080.0 * *global_state.resolution_slider_value,
    );
}
struct Camera {
    position: BigVec2,
    number_distance: f32,
}
impl Camera {
    fn new() -> Self {
        Camera {
            position: BigVec2 {
                x: BigNumber::new_d(0.0),
                y: BigNumber::new_d(0.0),
            },
            number_distance: screen_width() / 10.0,
        }
    }
}
fn update_grid(camera: &Camera) {
    let origin_offset = camera.position.clone();
    // let tl_corner = origin_offset + BigVec2::new((camera.number_distance * 5.0), BigNumber::new());
}
#[macroquad::main("GRAPHING_CALCULATOR")]
async fn main() {
    let mut resolution_slider_value = 1.0f32;
    let mut old_screen_width = screen_width();
    let mut old_screen_height = screen_height();
    let mut settings_position = vec2(400.0, 200.0);
    let mut GlobalState = AppState {
        resolution_slider_value: &mut resolution_slider_value,
        old_screen_width: &mut old_screen_width,
        old_screen_height: &mut old_screen_height,
        settings_position: &mut settings_position,
        current_fps: get_fps(),
    };
    let mut circle_cache = CircleCache { cache: Vec::new() };
    request_new_screen_size(1920.0, 1080.0);
    let mut is_first_iteration = true;
    let mut cooldown_storage = HashMap::new();
    let resolution_cooldown = cooldown::job::add(&mut cooldown_storage, "resolution", 2);
    let fps_cooldown = cooldown::job::add(&mut cooldown_storage, "fps", 1);
    let camera = Camera::new();
    loop {
        // Code that must run at the beginning of the frame
        if (is_first_iteration) {
            is_first_iteration = false;
            while ((1920.0 != screen_width()) && (1080.0 != screen_height())) {
                next_frame().await;
            }
            continue;
        }
        cooldown::job::update(&mut cooldown_storage);
        clear_background(WHITE);
        if (cooldown::job::is_on(&cooldown_storage, "resolution")) {
            update_resolution(&mut GlobalState);
        }
        if cooldown::job::is_on(&cooldown_storage, "fps") {
            GlobalState.current_fps = get_fps();
        }
        create_ui(&mut GlobalState);
        // Body Code
        let circle_radius = 150.0;
        let circle_x_pos = 200.0;
        let circle_y_pos = 200.0;
        let circle = create_circle(
            NumberDependency::from(circle_radius),
            NumberDependency::from(circle_x_pos),
            NumberDependency::from(circle_y_pos),
        );
        circle_cache.push(circle);
        update_grid(&camera);
        println!("{}, lol", (BigNumber::new_d(5.0) * 400000.0).get_value());
        // Code that must run at the end of the frame
        cooldown::job::update_next(&mut cooldown_storage);
        next_frame().await;
    }
}
