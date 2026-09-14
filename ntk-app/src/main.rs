mod app;

fn main() {
    env_logger::init();

    app::Application::run();
}
