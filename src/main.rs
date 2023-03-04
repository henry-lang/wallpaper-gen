mod camera;
mod config;
mod pipeline;
mod renderer;

use tinyrand::{Rand, StdRand};

use config::Config;
use pipeline::Instance;
use renderer::Renderer;

fn random_f32(rand: &mut impl Rand, lim: f32) -> f32 {
    let b = 32;
    let f = std::f32::MANTISSA_DIGITS - 1;
    (f32::from_bits((1 << (b - 2)) - (1 << f) + (rand.next_u32() >> (b - f))) - 1.0) * lim
}

fn generate_instances(rand: &mut impl Rand, config: &Config) -> Vec<Instance> {
    (0..config.count)
        .map(|_| Instance {
            color: config.colors[rand.next_lim_usize(config.colors.len())],
            transform: glam::Mat4::from_scale_rotation_translation(
                glam::Vec3::new(config.square_size[0], config.square_size[1], 0.0),
                glam::Quat::from_rotation_z(random_f32(rand, std::f32::consts::TAU) / 2.0),
                glam::Vec3::new(
                    random_f32(rand, config.size.0 as f32),
                    random_f32(rand, config.size.1 as f32),
                    0.0,
                ),
            )
            .to_cols_array_2d(),
        })
        .collect()
}

async fn run() {
    let config = Config {
        size: (1920, 1080),
        colors: vec![
            [64.0, 60.0, 60.0f32].map(|c| (c / 255.0).powf(2.2)),
            [64.0, 60.0, 60.0f32].map(|c| (c / 255.0).powf(2.2)),
            [64.0, 60.0, 60.0f32].map(|c| (c / 255.0).powf(2.2)),
            [124.0, 133.0, 113.0f32].map(|c| (c / 255.0).powf(2.2)),
            [219.0, 186.0, 139.0f32].map(|c| (c / 255.0).powf(2.2)),
        ],
        count: 1000,
        square_size: [200.0, 200.0],
    };

    env_logger::init();
    let mut renderer = Renderer::new((3840, 2160)).await;
    let mut rand = StdRand::default();

    renderer
        .render(&generate_instances(&mut rand, &config))
        .await
        .unwrap();
}

fn main() {
    pollster::block_on(run());
}
