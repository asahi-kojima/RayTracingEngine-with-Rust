mod ray_tracing_engine;


fn main() {
    let mut camera: ray_tracing_engine::camera::Camera = ray_tracing_engine::camera::Camera{x : 1.0};
    camera.x = 123.0;


    println!("Hello, world!");
}



