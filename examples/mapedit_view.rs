use std::num::Wrapping;
use euclid::{Point2D, Rect};
use calx_resource::Resource;
use scancode::Scancode;
use world::{Location, Portal, World};
use display;
use vitral;

enum PaintMode {
    Terrain(u8, u8),
    Portal,
}

/// Top-level application state for gameplay.
pub struct View {
    pub world: World,
    mode: PaintMode,
    /// Camera and second camera (for portaling)
    camera: (Location, Location),
    /// Do the two cameras move together?
    camera_lock: bool,
}

impl View {
    pub fn new(world: World) -> View {
        View {
            world: world,
            mode: PaintMode::Terrain(7, 2),
            camera: (Location::new(0, 0, 0), Location::new(0, 8, 0)),
            camera_lock: false,
        }
    }

    pub fn draw(&mut self, context: &mut display::Context, screen_area: &Rect<f32>) {
        // TODO: Camera logic
        let camera_loc = self.camera.0;

        let center = screen_area.origin + screen_area.size / 2.0;

        // Chart area, center in origin, inflated by tile width in every direction to get the cells
        // partially on screen included.
        let bounds = screen_area.translate(&-(center + screen_area.origin))
                                .inflate(display::PIXEL_UNIT * 2.0, display::PIXEL_UNIT * 2.0);

        context.ui.set_clip_rect(Some(*screen_area));

        let chart = display::screen_fov(&self.world, camera_loc, bounds);

        let mut sprites = Vec::new();

        let cursor_pos = display::view_to_chart(context.ui.mouse_pos() - center);

        for (&chart_pos, origins) in &chart {
            assert!(!origins.is_empty());

            let loc = origins[0] + chart_pos;

            let screen_pos = display::chart_to_view(chart_pos) + center;

            // TODO: Set up dynamic lighting, shade sprites based on angle and local light.
            display::draw_terrain_sprites(&self.world, loc, |layer, _angle, brush, frame_idx| {
                sprites.push(display::Sprite {
                    layer: layer,
                    offset: [screen_pos.x as i32, screen_pos.y as i32],
                    brush: brush.clone(),
                    frame_idx: frame_idx,
                })
            });

            for &origin in origins {
                if self.world.portal(origin + chart_pos).is_some() {
                    let screen_pos = screen_pos - Point2D::new(display::PIXEL_UNIT, display::PIXEL_UNIT);
                    sprites.push(display::Sprite {
                        layer: display::Layer::Decal,
                        offset: [screen_pos.x as i32, screen_pos.y as i32],
                        brush: Resource::new("portal".to_string()).unwrap(),
                        frame_idx: 0,
                    });
                    break;
                }
            }
        }

        if let Some(origins) = chart.get(&cursor_pos) {
            let screen_pos = display::chart_to_view(cursor_pos) + center -
                             Point2D::new(display::PIXEL_UNIT, display::PIXEL_UNIT);
            // Always portal in root coordinates.
            // TODO: It's currently not obvious from UI what the root coordinates are.
            let portal_loc = origins[origins.len() - 1] + cursor_pos;
            let loc = origins[0] + cursor_pos;

            sprites.push(display::Sprite {
                layer: display::Layer::Decal,
                offset: [screen_pos.x as i32, screen_pos.y as i32],
                brush: Resource::new("cursor".to_string()).unwrap(),
                frame_idx: 0,
            });
            sprites.push(display::Sprite {
                layer: display::Layer::Effect,
                offset: [screen_pos.x as i32, screen_pos.y as i32],
                brush: Resource::new("cursor_top".to_string()).unwrap(),
                frame_idx: 0,
            });

            match self.mode {
                PaintMode::Terrain(draw, erase) => {
                    if context.ui.is_mouse_pressed(vitral::MouseButton::Left) {
                        self.world.terrain.set(loc, draw);
                    }

                    if context.ui.is_mouse_pressed(vitral::MouseButton::Right) {
                        self.world.terrain.set(loc, erase);
                    }
                }

                PaintMode::Portal => {
                    let (a, b) = self.camera;
                    if a != b && context.ui.is_mouse_pressed(vitral::MouseButton::Left) {
                        self.world.set_portal(portal_loc, Portal::new(a, b));
                    }
                    if context.ui.is_mouse_pressed(vitral::MouseButton::Right) {
                        self.world.remove_portal(portal_loc);
                    }
                }
            }

        }

        sprites.sort();

        for i in &sprites {
            i.draw(&mut context.ui)
        }

        if context.ui.button("draw void") {
            self.mode = PaintMode::Terrain(0, 2);
        }

        if context.ui.button("draw gate") {
            self.mode = PaintMode::Terrain(1, 2);
        }

        if context.ui.button("draw wall") {
            self.mode = PaintMode::Terrain(6, 2);
        }

        if context.ui.button("draw rock") {
            self.mode = PaintMode::Terrain(7, 2);
        }

        if context.ui.button("PORTALS!") {
            self.mode = PaintMode::Portal;
        }

        for (y, origin) in chart.get(&cursor_pos).unwrap_or(&Vec::new()).iter().enumerate() {
            let font = context.ui.default_font();
            let loc = *origin + cursor_pos;
            context.ui.draw_text(&*font,
                                 Point2D::new(400.0, y as f32 * 20.0 + 20.0),
                                 [1.0, 1.0, 1.0, 1.0],
                                 &format!("{:?}", loc));
        }

        context.ui.set_clip_rect(None);

        if let Some(scancode) = context.backend.poll_key().and_then(|k| Scancode::new(k.scancode)) {
            use scancode::Scancode::*;
            match scancode {
                Q => self.move_camera(Point2D::new(-1, 0), 0),
                W => self.move_camera(Point2D::new(-1, -1), 0),
                E => self.move_camera(Point2D::new(0, -1), 0),
                A => self.move_camera(Point2D::new(0, 1), 0),
                S => self.move_camera(Point2D::new(1, 1), 0),
                D => self.move_camera(Point2D::new(1, 0), 0),
                Tab => self.switch_camera(),
                RightBracket => self.move_camera(Point2D::new(0, 0), 1),
                LeftBracket => self.move_camera(Point2D::new(0, 0), -1),
                _ => {}
            }
        }
    }

    fn move_camera(&mut self, delta: Point2D<i32>, dz: i8) {
        let second_delta = if self.camera_lock { delta } else { Point2D::new(0, 0) };

        let (a, b) = self.camera;
        self.camera = (a + delta, b + second_delta);

        let z0 = Wrapping(self.camera.0.z) + Wrapping(dz);
        let z1 = Wrapping(self.camera.1.z) + Wrapping(if self.camera_lock { dz } else { 0 });

        self.camera.0.z = z0.0;
        self.camera.1.z = z1.0;
    }

    fn switch_camera(&mut self) {
        let (a, b) = self.camera;
        self.camera = (b, a);
    }
}