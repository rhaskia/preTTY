use dioxus::prelude::*;
use tokio::time::Duration;

pub async fn wait_for_next_render() {
    let mut finished = use_signal(|| false);

    use_after_render(move || finished.set(true));

    while !finished() {
        tokio::time::sleep(Duration::from_secs_f32(1.0)).await;
    }
}
