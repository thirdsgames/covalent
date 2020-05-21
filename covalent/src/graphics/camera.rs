use std::cell::Cell;
use cgmath::{vec3, Point3, Matrix4, Transform, InnerSpace, SquareMatrix};

/// A camera is the lens through which your scene can be viewed. This tells covalent how to map the scene in 3D space
/// onto your screen, a 2D window. The two major types of camera are perspective and orthographic.
pub trait Camera3D {
    fn get_projection_matrix(&self) -> Matrix4<f32>;
    fn get_view_matrix(&self) -> Matrix4<f32>;
    fn get_combined_matrix(&self) -> Matrix4<f32>;

    /// Allows you to cast down to a perspective camera, if this is indeed that kind of camera.
    fn as_perspective_camera(&mut self) -> Option<&mut PerspectiveCamera> {
        None
    }
}

/// A perspective camera is used in a 3D setting. It emulates how our eyes or cameras work, making near things appear
/// large and far things appear small.
pub struct PerspectiveCamera {
    pos: Point3<f32>,
    proj: Cell<Matrix4<f32>>,
    view: Cell<Matrix4<f32>>,
    combined: Cell<Matrix4<f32>>,
    /// If the camera is "dirty", it needs to recalculate its matrices before next time they are used.
    dirty: Cell<bool>
}

impl PerspectiveCamera {
    /// Constructs a new perspective camera from the arguments supplied.
    pub fn new(pos: Point3<f32>) -> PerspectiveCamera {
        PerspectiveCamera {
            pos,
            proj: Cell::new(Matrix4::one()),
            view: Cell::new(Matrix4::one()),
            combined: Cell::new(Matrix4::one()),
            dirty: Cell::new(true)
        }
    }

    /// Updates the matrices contained within the camera. Call if you need to retrieve a value from
    /// this camera, but the state is dirty.
    fn update_matrices(&self) {
        self.proj.set({
            let mut p = cgmath::perspective(cgmath::Deg(60.0), 1.0, 0.01, 100.0);
            p.z.z *= -1.0;
            //p.w.z *= -1.0;
            p.z.w *= -1.0;
            p
        });
        //println!("Proj: {:?}", self.proj.get());
        //self.view.set(cgmath::Matrix4::look_at_dir(self.pos, vec3(0.0, 1.0, -1.0).normalize(), vec3(0.0, 1.0, 0.0).normalize()));
        let negpos = self.pos;
        self.view.set(cgmath::Matrix4::from_translation(cgmath::Vector3::new(negpos.x, negpos.y, negpos.z)));
        //println!("View: {:?}", self.view.get());
        self.combined.set(self.proj.get() * self.view.get());
        //println!("Combined: {:?}", self.combined.get());
        //println!("Example: {:?}", self.combined.get().transform_point(cgmath::Point3::new(0.0, 0.0, 0.0)));
        //println!("Example: {:?}", self.combined.get().transform_point(cgmath::Point3::new(1.0, 0.0, 0.0)));
        //println!("Example: {:?}", self.combined.get().transform_point(cgmath::Point3::new(0.0, 1.0, 0.0)));
        self.dirty.set(false);
    }

    pub fn set_pos(&mut self, pos: Point3<f32>) {
        self.pos = pos;
        self.dirty.set(true);
    }
}

impl Camera3D for PerspectiveCamera {
    fn get_projection_matrix(&self) -> Matrix4<f32> {
        if self.dirty.get() {
            self.update_matrices();
        }
        self.proj.get()
    }
    fn get_view_matrix(&self) -> Matrix4<f32> {
        if self.dirty.get() {
            self.update_matrices();
        }
        self.view.get()
    }
    fn get_combined_matrix(&self) -> Matrix4<f32> {
        if self.dirty.get() {
            self.update_matrices();
        }
        self.combined.get()
    }

    fn as_perspective_camera(&mut self) -> Option<&mut PerspectiveCamera> {
        Some(self)
    }
}