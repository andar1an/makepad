// a bunch o buttons to select the world
use makepad_render::*;

#[derive(Clone)]
pub struct Viewport3D {
    pub pass: Pass,
    pub clear_color: Color,
    pub color_texture: Texture,
    pub depth_texture: Texture,
    pub view_2d: View,
    pub view_3d: View,
    pub measured_size: Vec2,
    pub camera_center: Vec3,
    pub camera_pos: Vec3,
    pub camera_rot: Vec3,
    pub camera_start: Option<(Vec3, Vec3)>,
    pub blit: Blit
}

impl Viewport3D {
    pub fn new(cx: &mut Cx) -> Self {
        Self {
            pass: Pass::default(),
            camera_center: Vec3 {x: 0.0, y: 0.0, z: 1.1 + 1.5},
            camera_pos: Vec3 {x: 0.0, y: 0.0, z: -1.1},
            camera_rot: Vec3 {x: 0.0, y: 0.0, z: 0.0},
            camera_start: None,
            clear_color: Color::parse_hex_str("040").unwrap(),
            color_texture: Texture::new(cx),
            depth_texture: Texture::new(cx),
            view_3d: View::new(cx),
            view_2d: View::new(cx),
            measured_size: Vec2::all(1.0),
            blit: Blit::new(cx)
        }
    }
    
    pub fn style(cx: &mut Cx) {
        live_body!(cx, r#"
            self::pos: vec3(0., 0.0, -1.1);
        "#);
    }
    
     
    pub fn handle_viewport_2d(&mut self, cx: &mut Cx, event: &mut Event) {
        match event.hits(cx, self.view_2d.get_view_area(cx), HitOpt::default()) {
            Event::FingerHover(_fe) => {
                cx.set_hover_mouse_cursor(MouseCursor::Move);
            },
            Event::FingerDown(_fe) => { 
                
                cx.set_down_mouse_cursor(MouseCursor::Move);
                self.camera_start = Some((self.camera_pos, self.camera_rot));
            }, 
            Event::FingerUp(_fe) => {
            } 
            Event::FingerScroll(fe) => {
                self.camera_pos.z += fe.scroll.y / 300.0;
                self.camera_center.z = -self.camera_pos.z + 1.5;
                self.pass_set_matrix_mode(cx);
            }
            Event::FingerMove(fe) => {
                if let Some((_pos, rot)) = self.camera_start {
                    self.camera_rot = Vec3 {
                        x: rot.x + (fe.abs.y - fe.abs_start.y),
                        y: rot.y + (fe.abs.x - fe.abs_start.x),
                        z: rot.z
                    };
                    self.pass_set_matrix_mode(cx)
                }
            },
            _ => ()
        }
    }
    
    pub fn pass_set_matrix_mode(&mut self, cx: &mut Cx) {
        self.pass.set_matrix_mode(cx, PassMatrixMode::Projection {
            fov_y: 40.0,
            near: 1.0,
            far: 1000.0,
            cam: Mat4::rotate_txyz_s_ry_rx_txyz(
                Vec3 {
                    x: self.camera_pos.x + self.camera_center.x,
                    y: self.camera_pos.y + self.camera_center.y,
                    z: self.camera_pos.z + self.camera_center.z
                },
                1.0,
                self.camera_rot.y,
                self.camera_rot.x,
                self.camera_center.neg(),
            )
        });
    }
    
    pub fn begin_viewport_3d(&mut self, cx: &mut Cx) -> ViewRedraw {
        if !self.view_3d.view_will_redraw(cx) {
            return Err(())
        }
        
        self.pass.begin_pass(cx);
        self.pass.set_size(cx, self.measured_size);
        self.pass_set_matrix_mode(cx);
        self.pass.add_color_texture(cx, self.color_texture, ClearColor::ClearWith(self.clear_color));
        self.pass.set_depth_texture(cx, self.depth_texture, ClearDepth::ClearWith(1.0));
        
        let _ = self.view_3d.begin_view(cx, Layout::default());
        
        Ok(())
    }
    
    pub fn end_viewport_3d(&mut self, cx: &mut Cx) {
        self.view_3d.end_view(cx);
        self.pass.end_pass(cx);
    }
    
    pub fn draw_viewport_2d(&mut self, cx: &mut Cx) {
        if self.view_2d.begin_view(cx, Layout::default()).is_err() {
            return
        };
        self.view_3d.redraw_view_area(cx);
        // blit the texture to a view rect
        let inst = self.blit.begin_blit_fill(cx, self.color_texture);
        self.blit.end_blit_fill(cx, inst);
        self.measured_size = Vec2 {x: cx.get_width_total(), y: cx.get_height_total()};
        
        self.view_2d.end_view(cx);
    }
}
