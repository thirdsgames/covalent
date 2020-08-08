use crate::scene::*;
use std::sync::{RwLock, Arc, Weak};
use cgmath::{vec3, Vector3, Quaternion, Matrix4, Transform};
use crate::graphics::Renderable;
use crate::input::ElementState;

/// The node is the root of anything that is in the scene.
/// Nodes have a list of `Behaviour`s, which represent the functionality of the node.
pub struct Node {
    /// Refers to this node.
    self_ref: Weak<RwLock<Self>>,
    /// A reference to the scene that contains this node.
    scene: Weak<RwLock<Scene>>,
    /// The position of the node.
    pos: Vector3<f32>,
    /// The rotation of the node.
    rot: Quaternion<f32>,
    /// The scale of the node (which can be different for each axis).
    scl: Vector3<f32>,
    /// The matrix that represents the transformation of this node.
    xform: Matrix4<f32>,

    /// The ordered list of components this node contains.
    pub components: Vec<Arc<RwLock<dyn Component>>>,

    /// A reference to the renderable that we are going to try to render with this instance, if we want to actually render something.
    pub renderable: Option<Arc<Renderable>>,
}

impl Node {
    /// This is an internal function. Do not call this yourself!
    /// Alternative: `Scene::new_node`.
    /// 
    /// Creates a new node with default settings and no instances or renderable.
    /// Does not implement `Default`: we want to encapsulate every node in an `Arc<RwLock<>>`.
    pub(crate) fn default(scene: Arc<RwLock<Scene>>) -> Arc<RwLock<Self>> {
        let node = Arc::new(RwLock::new(Node {
            self_ref: Weak::new(),
            scene: Arc::downgrade(&scene),
            pos: vec3(0.0, 0.0, 0.0),
            rot: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scl: vec3(1.0, 1.0, 1.0),
            xform: Matrix4::one(),

            components: Vec::new(),

            renderable: None
        }));
        node.write().unwrap().self_ref = Arc::downgrade(&node);
        return node;
    }

    pub fn scene(&self) -> &Weak<RwLock<Scene>> {
        &self.scene
    }
}

/// Components listen for events to execute event-driven code.
pub trait Component: Send + Sync {}

// TICK DEBUG COMPONENT

pub struct TickDebugComponent {
    node: Weak<RwLock<Node>>,
    tick_num: i32
}
impl Component for TickDebugComponent {}

crate::lock_data! {
    TickDebugData
    component: write TickDebugComponent
}

impl TickDebugComponent {
    pub fn new(node: Arc<RwLock<Node>>) {
        let component = Arc::new(RwLock::new(TickDebugComponent {
            node: Arc::downgrade(&node),
            tick_num: 0
        }));

        let data = Arc::new(RwLock::new(TickDebugData {
            component: Arc::downgrade(&component),
        }));

        if let Some(scene) = node.read().unwrap().scene.upgrade() {
            TickDebugData::listen(&data, &scene.read().unwrap().events.tick, |_event, component| {
                component.tick_num += 1;
                //println!("Tick {} (delta: {})", component.tick_num, event.delta);
            });
        }

        node.write().unwrap().components.push(component);
    }
}

// CAMERA MOTION COMPONENT

pub struct CameraMotionComponent {
    cam: crate::graphics::PerspectiveCamera,
    camera_matrices: Arc<RwLock<crate::graphics::CameraMatrices>>,

    key_forward: bool,
    key_backward: bool,
    key_left: bool,
    key_right: bool,
    key_up: bool,
    key_down: bool,

    pitch: f32,
    yaw: f32,
}
impl Component for CameraMotionComponent {}

crate::lock_data! {
    CameraMotionData
    component: write CameraMotionComponent
}

impl CameraMotionComponent {
    pub fn new(node: Arc<RwLock<Node>>, mut cam: crate::graphics::PerspectiveCamera, camera_matrices: Arc<RwLock<crate::graphics::CameraMatrices>>) {
        use crate::graphics::Camera;

        cam.set_pos(crate::pt3(3.0, 3.0, 3.0));

        let component = Arc::new(RwLock::new(CameraMotionComponent {
            cam,
            camera_matrices,

            key_forward: false,
            key_backward: false,
            key_left: false,
            key_right: false,
            key_up: false,
            key_down: false,

            pitch: 0.0,
            yaw: 0.0,
        }));

        let data = Arc::new(RwLock::new(CameraMotionData {
            component: Arc::downgrade(&component),
        }));

        if let Some(scene) = node.read().unwrap().scene.upgrade() {
            CameraMotionData::listen(&data, &scene.read().unwrap().events.tick, |_event, component| {
                // Update motion according to the keys pressed.
                let mut offset_pos = crate::vec3(0.0, 0.0, 0.0);

                if component.key_forward {
                    offset_pos += component.cam.get_dir();
                }
                if component.key_backward {
                    offset_pos -= component.cam.get_dir();
                }
                if component.key_right {
                    offset_pos += component.cam.get_right();
                }
                if component.key_left {
                    offset_pos -= component.cam.get_right();
                }
                if component.key_up {
                    offset_pos += component.cam.get_up();
                }
                if component.key_down {
                    offset_pos -= component.cam.get_up();
                }

                component.cam.set_pos(component.cam.get_pos() + offset_pos * 0.001);
                let xy = component.pitch.cos();
                component.cam.set_dir(cgmath::vec3(-xy * component.yaw.cos(), xy * component.yaw.sin(), -component.pitch.sin()));

                component.cam.update_matrices(Arc::clone(&component.camera_matrices));
            });

            CameraMotionData::listen(&data, &scene.read().unwrap().events.key, |event, component| {
                match event.virtual_keycode {
                    Some(crate::input::VirtualKeyCode::W) => {
                        component.key_forward = match event.state {
                            ElementState::Pressed => { true },
                            ElementState::Released => { false },
                        }
                    }
                    Some(crate::input::VirtualKeyCode::A) => {
                        component.key_left = match event.state {
                            ElementState::Pressed => { true },
                            ElementState::Released => { false },
                        }
                    }
                    Some(crate::input::VirtualKeyCode::S) => {
                        component.key_backward = match event.state {
                            ElementState::Pressed => { true },
                            ElementState::Released => { false },
                        }
                    }
                    Some(crate::input::VirtualKeyCode::D) => {
                        component.key_right = match event.state {
                            ElementState::Pressed => { true },
                            ElementState::Released => { false },
                        }
                    }
                    Some(crate::input::VirtualKeyCode::E) => {
                        component.key_up = match event.state {
                            ElementState::Pressed => { true },
                            ElementState::Released => { false },
                        }
                    }
                    Some(crate::input::VirtualKeyCode::Q) => {
                        component.key_down = match event.state {
                            ElementState::Pressed => { true },
                            ElementState::Released => { false },
                        }
                    }
                    _ => {}
                }
            });

            CameraMotionData::listen(&data, &scene.read().unwrap().events.mouse_delta, |event, component| {
                component.pitch += event.delta.y as f32 * 0.001f32;
                component.yaw += event.delta.x as f32 * 0.001f32;
            });
        }

        node.write().unwrap().components.push(component);
    }
}
