mod camera;
mod pipeline;
mod renderer;

use wgpu::SurfaceError;

use renderer::Renderer;

async fn run() {
    env_logger::init();

    let mut state = Renderer::new((1920, 1080)).await;
    state.render().await;
}

fn main() {
    pollster::block_on(run());
}
