use euclid::{Point2D, Rect};
use calx_resource::Resource;
use world::{Location, World};
use sprite::Sprite;
use backend;
use view;
use render;

/// Top-level application state for gameplay.
pub struct GameView {
    pub world: World,
    terrain_brush: u8,
}

impl GameView {
    pub fn new(world: World) -> GameView {
        GameView {
            world: world,
            terrain_brush: 7,
        }
    }

    pub fn draw(&mut self, context: &mut backend::Context, screen_area: &Rect<f32>) {
        // TODO: Camera logic
        let camera_loc = Location::new(0, 0);

        let center = screen_area.origin + screen_area.size / 2.0;

        // Chart area, center in origin, inflated by tile width in every direction to get the cells
        // partially on screen included.
        let bounds = screen_area.translate(&-(center + screen_area.origin))
                                .inflate(view::PIXEL_UNIT * 2.0, view::PIXEL_UNIT * 2.0);

        context.set_clip_rect(Some(*screen_area));

        let chart = view::screen_fov(&self.world, camera_loc, bounds);

        let mut sprites = Vec::new();

        let cursor_pos = view::view_to_chart(context.mouse_pos() - center);

        for (&chart_pos, origins) in chart.iter() {
            assert!(!origins.is_empty());

            let loc = origins[0] + chart_pos;

            let screen_pos = view::chart_to_view(chart_pos) + center;

            // TODO: Set up dynamic lighting, shade sprites based on angle and local light.
            render::draw_terrain_sprites(&self.world, loc, |layer, _angle, brush, frame_idx| {
                sprites.push(Sprite {
                    layer: layer,
                    offset: [screen_pos.x as i32, screen_pos.y as i32],
                    brush: brush.clone(),
                    frame_idx: frame_idx,
                })
            });
        }

        if let Some(origins) = chart.get(&cursor_pos) {
            let screen_pos = view::chart_to_view(cursor_pos) + center -
                             Point2D::new(view::PIXEL_UNIT, view::PIXEL_UNIT);
            let loc = origins[0] + cursor_pos;

            sprites.push(Sprite {
                layer: render::Layer::Decal,
                offset: [screen_pos.x as i32, screen_pos.y as i32],
                brush: Resource::new("cursor".to_string()).unwrap(),
                frame_idx: 0,
            });
            sprites.push(Sprite {
                layer: render::Layer::Effect,
                offset: [screen_pos.x as i32, screen_pos.y as i32],
                brush: Resource::new("cursor_top".to_string()).unwrap(),
                frame_idx: 0,
            });

            if context.is_mouse_pressed() {
                self.world.terrain.set(loc, self.terrain_brush);
            }
        }

        sprites.sort();

        for i in sprites.iter() {
            i.draw(context)
        }

        if context.button("draw void") {
            self.terrain_brush = 0;
        }

        if context.button("draw gate") {
            self.terrain_brush = 1;
        }

        if context.button("draw ground") {
            self.terrain_brush = 2;
        }

        if context.button("draw wall") {
            self.terrain_brush = 6;
        }

        if context.button("draw rock") {
            self.terrain_brush = 7;
        }

        context.set_clip_rect(None);
    }
}