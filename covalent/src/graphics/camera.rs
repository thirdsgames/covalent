use std::sync::{Arc, RwLock};
use cgmath::{Vector3, Point3, Matrix4, Transform, InnerSpace, SquareMatrix, Vector2};
use std::sync::atomic::{AtomicBool, Ordering};

/// A camera is the lens through which your scene can be viewed. This tells covalent how to map the
/// scene in 2D/3D space onto your screen, a 2D window. The two major types of camera are
/// perspective and orthographic.
pub trait Camera {
    fn get_projection_matrix(&self) -> Matrix4<f32>;
    fn get_view_matrix(&self) -> Matrix4<f32>;
    fn get_combined_matrix(&self) -> Matrix4<f32>;

    /// Sends the camera's matrices to the graphics backend for rendering.
    /// Run this function when you've edited the camera's variables e.g. position, view angle,
    /// aspect ratio etc.
    fn update_matrices(&self, matrices: Arc<RwLock<CameraMatrices>>) {
        let c = self.get_combined_matrix();
        let mut matrices = matrices.write().expect("Could not write to matrices variable");

        matrices.combined = c;
        matrices.inverse = c.invert().unwrap_or(Matrix4::identity())
    }
}

/// A representation of the camera's matrices that can be sent to the graphics backend to be
/// used for rendering.
pub struct CameraMatrices {
    pub combined: Matrix4<f32>,
    pub inverse: Matrix4<f32>,
}

impl Default for CameraMatrices {
    fn default() -> Self {
        Self {
            combined: Matrix4::identity(),
            inverse: Matrix4::identity(),
        }
    }
}

/// A perspective camera is used in a 3D setting. It emulates how our eyes or cameras work, making near things appear
/// large and far things appear small.
pub struct PerspectiveCamera {
    pos: Point3<f32>,
    dir: Vector3<f32>,
    up: Vector3<f32>,

    screen_resolution: Vector2<f32>,

    proj: RwLock<Matrix4<f32>>,
    view: RwLock<Matrix4<f32>>,
    combined: RwLock<Matrix4<f32>>,

    /// If the camera is "dirty", it needs to recalculate its matrices before next time they are used.
    dirty: AtomicBool
}

impl PerspectiveCamera {
    /// Constructs a new perspective camera from the arguments supplied.
    pub fn new(pos: Point3<f32>, dir: Vector3<f32>, up: Vector3<f32>) -> PerspectiveCamera {
        PerspectiveCamera {
            pos,
            dir,
            up,

            // Supply a dummy screen resolution to provide a 1:1 aspect ratio.
            screen_resolution: cgmath::vec2(800.0, 800.0),

            proj: RwLock::new(Matrix4::one()),
            view: RwLock::new(Matrix4::one()),
            combined: RwLock::new(Matrix4::one()),

            dirty: AtomicBool::new(true)
        }
    }

    /// Updates the matrices contained within the camera. Call if you need to retrieve a value from
    /// this camera, but the state is dirty.
    fn update_matrices(&self) {
        *self.proj.write().unwrap() = cgmath::perspective(cgmath::Deg(60.0), self.get_aspect_ratio(), 0.01, 100.0);
        *self.view.write().unwrap() = cgmath::Matrix4::look_at_dir(self.pos, self.dir, self.up);
        *self.combined.write().unwrap() = *self.proj.read().unwrap() * *self.view.read().unwrap();
        self.dirty.store(false, Ordering::SeqCst);
    }

    /// Sets the position that the camera is looking from.
    pub fn set_pos(&mut self, pos: Point3<f32>) {
        self.pos = pos;
        self.dirty.store(true, Ordering::SeqCst);
    }

    /// Retrieves the position that the camera is looking from.
    pub fn get_pos(&self) -> Point3<f32> {
        self.pos
    }

    /// Sets the direction that the camera is looking towards.
    /// This will be normalised automatically.
    pub fn set_dir(&mut self, dir: Vector3<f32>) {
        self.dir = dir.normalize();
        self.dirty.store(true, Ordering::SeqCst);
    }

    /// Retrieves the (normalised) direction that the camera is looking towards.
    pub fn get_dir(&self) -> Vector3<f32> {
        self.dir
    }

    /// Sets the direction pointing upwards from the camera.
    /// This is normally something like `vec3(0, 0, 1)`.
    /// This will be normalised automatically.
    pub fn set_up(&mut self, up: Vector3<f32>) {
        self.up = up.normalize();
        self.dirty.store(true, Ordering::SeqCst);
    }

    /// Retrieves the (normalised) direction pointing upwards from the camera. This is not
    /// necessarily perpendicular to the `dir` direction.
    pub fn get_up(&self) -> Vector3<f32> {
        self.up
    }

    /// Retrieves the (normalised) direction pointing to the right from the camera.
    pub fn get_right(&self) -> Vector3<f32> {
        self.dir.cross(self.up)
    }

    /// Sets the resolution of the screen. The camera will deduce the aspect ratio for the given
    /// screen resolution.
    pub fn set_screen_resolution(&mut self, screen_resolution: Vector2<f32>) {
        self.screen_resolution = screen_resolution;
        self.dirty.store(true, Ordering::SeqCst);
    }

    /// Retrieves the screen resolution supplied to this camera.
    pub fn get_screen_resolution(&self) -> Vector2<f32> {
        self.screen_resolution
    }

    /// Returns the aspect ratio supplied to this camera. This is calculated from the screen
    /// resolution: `width / height`.
    pub fn get_aspect_ratio(&self) -> f32 {
        self.screen_resolution.x / self.screen_resolution.y
    }
}

impl Camera for PerspectiveCamera {
    fn get_projection_matrix(&self) -> Matrix4<f32> {
        if self.dirty.load(Ordering::SeqCst) {
            self.update_matrices();
        }
        *self.proj.read().unwrap()
    }
    fn get_view_matrix(&self) -> Matrix4<f32> {
        if self.dirty.load(Ordering::SeqCst) {
            self.update_matrices();
        }
        *self.view.read().unwrap()
    }
    fn get_combined_matrix(&self) -> Matrix4<f32> {
        if self.dirty.load(Ordering::SeqCst) {
            self.update_matrices();
        }
        *self.combined.read().unwrap()
    }
}