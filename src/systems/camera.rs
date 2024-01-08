use amethyst::core::math::Vector3;
use amethyst::core::num::{clamp_max, clamp_min};
use amethyst::core::Transform;
use amethyst::ecs::{Join, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::Camera;
use amethyst::shred::Read;

pub struct CameraSystem {
    current_scale: f32,
}

impl Default for CameraSystem {
    fn default() -> Self {
        CameraSystem { current_scale: 1.0 }
    }
}

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        WriteStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (cameras, mut transforms, input): Self::SystemData) {
        for (_, transform) in (&cameras, &mut transforms).join() {
            let speed = 5.0;
            let scale_speed = 0.1;

            if let Some(movement) = input.axis_value("horizontal") {
                transform.translation_mut().x += speed * movement as f32;
            }

            if let Some(movement) = input.axis_value("vertical") {
                transform.translation_mut().y += speed * movement as f32;
            }

            if let Some(scale) = input.axis_value("scale") {
                self.current_scale *= 1.0 + scale * scale_speed;
                self.current_scale = clamp_max(self.current_scale, 5.0);
                self.current_scale = clamp_min(self.current_scale, 0.25);
                let scale = Vector3::new(1.0, 1.0, 1.0);
                transform.set_scale(scale * self.current_scale);
            }
        }
    }
}
