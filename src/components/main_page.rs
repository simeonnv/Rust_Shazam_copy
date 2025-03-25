use dioxus::prelude::*;
use web_sys::{AudioContext, MediaStream, MediaStreamConstraints, MediaStreamTrack};
use js_sys::Promise;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use gloo_timers::future::TimeoutFuture;

#[component]
pub fn Main_page() -> Element {
    let mut status = use_signal(|| "Not requested".to_string());
    let mut audio_data = use_signal(|| "No data yet".to_string());
    let mut stream = use_signal::<Option<MediaStream>>(|| None);
    let mut is_running = use_signal(|| false);

    let request_microphone = move |_| {
        spawn(async move {
            if stream().is_some() {
                return;
            }

            let window = web_sys::window().expect("no global window exists");
            let navigator = window.navigator();
            let media_devices = navigator.media_devices().expect("media devices not available");

            let constraints = MediaStreamConstraints::new();
            constraints.set_audio(&JsValue::from(true));

            let promise: Promise = media_devices
                .get_user_media_with_constraints(&constraints)
                .expect("getUserMedia failed");
            match JsFuture::from(promise).await {
                Ok(js_stream) => {
                    let media_stream = js_stream.dyn_into::<MediaStream>().unwrap();
                    status.set("Microphone access granted".to_string());
                    stream.set(Some(media_stream.clone()));
                    is_running.set(true);

                    let audio_context = AudioContext::new().unwrap();
                    let source = audio_context
                        .create_media_stream_source(&media_stream)
                        .unwrap();
                    let analyser = audio_context.create_analyser().unwrap();
                    source.connect_with_audio_node(&analyser).unwrap();
                    analyser.set_fft_size(2048);

                    spawn(async move {
                        let mut buffer = vec![0u8; analyser.frequency_bin_count() as usize];
                        while is_running() {
                            analyser.get_byte_time_domain_data(&mut buffer);
                            let amplitude = buffer.iter().map(|&x| x as f32).sum::<f32>() / buffer.len() as f32;
                            audio_data.set(format!("Average amplitude: {:.2}", amplitude));
                            TimeoutFuture::new(100).await;
                        }
                        audio_data.set("Microphone stopped".to_string());
                    });
                }
                Err(err) => {
                    let err_msg = err.as_string().unwrap_or("Unknown error".to_string());
                    status.set(format!("Microphone access denied: {}", err_msg));
                }
            }
        });
    };

    let stop_microphone = move |_| {
        if let Some(media_stream) = stream() {
            let tracks = media_stream.get_tracks();
            for i in 0..tracks.length() {
                if let Some(track) = tracks.get(i).dyn_into::<MediaStreamTrack>().ok() {
                    track.stop(); // MediaStreamTrack has stop()
                }
            }
            stream.set(None);
            is_running.set(false);
            status.set("Not requested".to_string());
        }
    };

    rsx! {
        button {
            onclick: request_microphone,
            "Request Microphone Access"
        }
        button {
            onclick: stop_microphone,
            disabled: stream().is_none(),
            "Stop Microphone"
        }
        div {
            "Status: {status}"
        }
        div {
            "Audio Data: {audio_data}"
        }
    }
}