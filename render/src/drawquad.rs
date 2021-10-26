// ok lets implement these things
live_body!{
    
    use crate::shader_std::*;
    use crate::geometrygen::GeometryQuad2D;
    
    DrawQuad: DrawShader2D {
        //debug: true;
        rust_type: {{DrawQuad}};
        geometry: GeometryQuad2D {};
        
        varying pos: vec2;
        
        fn scroll(self) -> vec2 {
            return self.draw_scroll.xy;
        }
        
        fn vertex(self) -> vec4 {
            let scr = self.scroll();
            
            let clipped: vec2 = clamp(
                self.geom_pos * self.rect_size + self.rect_pos - scr,
                self.draw_clip.xy,
                self.draw_clip.zw
            );
            self.pos = (clipped + scr - self.rect_pos) / self.rect_size;
            // only pass the clipped position forward
            return self.camera_projection * (self.camera_view * (self.view_transform * vec4(
                clipped.x,
                clipped.y,
                self.draw_depth + self.draw_zbias,
                1.
            )));
        }
        
        fn pixel(self) -> vec4 {
            return #f0f;
        }
    }
}

use crate::cx::*;

//#[derive(Debug)]
#[repr(C)]
pub struct DrawQuad {
    //#[local()]
    pub area: Area,
    
    //#[live()]
    pub geometry: GeometryQuad2D,
    
    //#[local()]
    pub draw_shader: Option<DrawShader>,
    
    //#[local()]
    pub draw_call_vars: DrawCallVars,
    
    //#[live(Vec2::all(0.0))]
    pub rect_pos: Vec2,
    //#[live(Vec2::all(0.0))]
    pub rect_size: Vec2,
    //#[live(1.0)]
    pub draw_depth: f32,
    //#[pad_f32()] 
}

impl DrawQuad {
    pub fn live_update_value(&mut self, cx: &mut Cx, id: Id, ptr: LivePtr) {
        match id {
            id!(geometry) => self.geometry.live_update(cx, ptr),
            id!(rect_pos) => self.rect_pos.live_update(cx, ptr),
            id!(rect_size) => self.rect_size.live_update(cx, ptr),
            id!(draw_depth) => self.draw_depth.live_update(cx, ptr),
            _ => self.live_update_value_unknown(cx, id, ptr)
        }
    }
}

impl LiveUpdateHooks for DrawQuad {
    fn live_update_value_unknown(&mut self, cx: &mut Cx, id: Id, ptr: LivePtr) {
        cx.update_draw_call_var(
            self.draw_shader.as_ref().unwrap().draw_shader_ptr,
            ptr,
            id,
            &mut self.draw_call_vars
        );
    }
    
    fn before_live_update(&mut self, cx: &mut Cx, live_ptr: LivePtr) {
        self.draw_shader = cx.get_draw_shader_from_ptr(
            DrawShaderPtr(live_ptr),
            &self.geometry
        );
    }
    
    fn after_live_update(&mut self, cx: &mut Cx, _live_ptr: LivePtr) {
        cx.init_draw_call_vars(
            self.draw_shader,
            &mut self.draw_call_vars
        );
    }
}

// how could we compile this away
impl LiveNew for DrawQuad {
    fn live_new(cx: &mut Cx) -> Self {
        Self {
            area: Area::Empty,
            
            draw_shader: None,
            geometry: LiveNew::live_new(cx),
            
            draw_call_vars: DrawCallVars::default(),
            rect_pos: Vec2::all(0.0),
            rect_size: Vec2::all(0.0),
            draw_depth: 1.0,
        }
    }
    
    fn live_type() -> LiveType {
        LiveType(std::any::TypeId::of::<DrawQuad>())
    }
    
    fn live_register(cx: &mut Cx) {
        cx.register_live_body(live_body());
        struct Factory();
        impl LiveFactory for Factory {
            fn live_new(&self, cx: &mut Cx) -> Box<dyn LiveUpdate> {
                Box::new(DrawQuad ::live_new(cx))
            }
            
            fn live_fields(&self, fields: &mut Vec<LiveField>) {
                fields.push(LiveField {id: Id::from_str("geometry").unwrap(), live_type: GeometryQuad2D::live_type()});
                fields.push(LiveField {id: Id::from_str("rect_pos").unwrap(), live_type: Vec2::live_type()});
                fields.push(LiveField {id: Id::from_str("rect_size").unwrap(), live_type: Vec2::live_type()});
                fields.push(LiveField {id: Id::from_str("draw_depth").unwrap(), live_type: f32::live_type()});
                // can i somehow someway autogenerate this
                fields.push(LiveField {id: Id(0), live_type: f32::live_type()});
            }
            
            fn live_type(&self) -> LiveType {
                DrawQuad::live_type()
            }
        }
        cx.register_factory(DrawQuad::live_type(), Box::new(Factory()));
    }
}

impl LiveUpdate for DrawQuad {
    fn live_update(&mut self, cx: &mut Cx, live_ptr: LivePtr) {
        self.before_live_update(cx, live_ptr);
        // how do we verify this?
        if let Some(mut iter) = cx.shader_registry.live_registry.live_class_iterator(live_ptr) {
            while let Some((id, live_ptr)) = iter.next(&cx.shader_registry.live_registry) {
                if id == id!(rust_type) && !cx.verify_type_signature(live_ptr, Self::live_type()) {
                    // give off an error/warning somehow!
                    return;
                }
                self.live_update_value(cx, id, live_ptr)
            }
        }
        self.after_live_update(cx, live_ptr);
    }
    
    fn _live_type(&self) -> LiveType {
        Self::live_type()
    }
}


impl DrawQuad {
    
    pub fn begin_quad(&mut self, cx: &mut Cx, layout: Layout) {
        if let Some(draw_shader) = self.draw_shader {
            let new_area = cx.add_aligned_instance(draw_shader, self.draw_call_vars.instance_slice(), &self.draw_call_vars);
            self.area = cx.update_area_refs(self.area, new_area);
        }
        cx.begin_turtle(layout, self.area);
    }
    
    pub fn end_quad(&mut self, cx: &mut Cx) {
        let rect = cx.end_turtle(self.area);
        self.area.set_rect(cx, &rect);
    }
    
    pub fn draw_quad_walk(&mut self, cx: &mut Cx, walk: Walk) {
        let rect = cx.walk_turtle(walk);
        self.rect_pos = rect.pos;
        self.rect_size = rect.size;
        self.draw_quad(cx);
    }
    
    pub fn draw_quad_abs(&mut self, cx: &mut Cx, rect: Rect) {
        self.rect_pos = rect.pos;
        self.rect_size = rect.size;
        self.draw_quad(cx);
    }
    
    pub fn draw_quad_rel(&mut self, cx: &mut Cx, rect: Rect) {
        let rect = rect.translate(cx.get_turtle_origin());
        self.rect_pos = rect.pos;
        self.rect_size = rect.size;
        self.draw_quad(cx);
    }
    
    pub fn draw_quad(&mut self, cx: &mut Cx) {
        if let Some(draw_shader) = self.draw_shader {
            let new_area = cx.add_aligned_instance(draw_shader, self.draw_call_vars.instance_slice(), &self.draw_call_vars);
            self.area = cx.update_area_refs(self.area, new_area);
        }
    }
    
    
}

