use std::cell::Cell;
use cgmath::{Vector3, Point3, Matrix4, Transform, InnerSpace};

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
    dir: Vector3<f32>,
    up: Vector3<f32>,
    proj: Cell<Matrix4<f32>>,
    view: Cell<Matrix4<f32>>,
    combined: Cell<Matrix4<f32>>,
    /// If the camera is "dirty", it needs to recalculate its matrices before next time they are used.
    dirty: Cell<bool>
}

impl PerspectiveCamera {
    /// Constructs a new perspective camera from the arguments supplied.
    pub fn new(pos: Point3<f32>, dir: Vector3<f32>, up: Vector3<f32>) -> PerspectiveCamera {
        PerspectiveCamera {
            pos,
            dir,
            up,
            proj: Cell::new(Matrix4::one()),
            view: Cell::new(Matrix4::one()),
            combined: Cell::new(Matrix4::one()),
            dirty: Cell::new(true)
        }
    }

    /// Updates the matrices contained within the camera. Call if you need to retrieve a value from
    /// this camera, but the state is dirty.
    fn update_matrices(&self) {
        self.proj.set(cgmath::perspective(cgmath::Deg(60.0), 1.0, 0.01, 100.0));
        self.view.set(cgmath::Matrix4::look_at_dir(self.pos, self.dir, self.up));
        self.combined.set(self.proj.get() * self.view.get());
        self.dirty.set(false);
    }

    /// Sets the position that the camera is looking from.
    pub fn set_pos(&mut self, pos: Point3<f32>) {
        self.pos = pos;
        self.dirty.set(true);
    }

    /// Sets the direction that the camera is looking towards.
    /// This will be normalised automatically.
    pub fn set_dir(&mut self, dir: Vector3<f32>) {
        self.dir = dir.normalize();
        self.dirty.set(true);
    }

    /// Sets the direction pointing upwards from the camera.
    /// This is normally something like `vec3(0, 0, 1)`.
    /// This will be normalised automatically.
    pub fn set_up(&mut self, up: Vector3<f32>) {
        self.up = up.normalize();
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