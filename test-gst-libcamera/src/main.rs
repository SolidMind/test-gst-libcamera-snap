use std::{thread::sleep, time::Duration};

use gstreamer::{
    FlowError, FlowSuccess,
    glib::object::Cast,
    prelude::{ElementExt, GstBinExt},
};

fn main() {
    let WIDTH = 1280;
    let HEIGHT = 720;
    gstreamer::init().unwrap();
    let pipeline = gstreamer::parse::launch(&format!(
        "libcamerasrc ! video/x-raw,format=RGB,width={WIDTH},height={HEIGHT} ! appsink name=sink"
    ))
    .expect("Failed to parse GStreamer pipeline");
    let pipeline = pipeline
        .dynamic_cast::<gstreamer::Pipeline>()
        .expect("Failed to cast to GStreamer Pipeline");

    // appsinkを取得して画像コールバックを設定
    let appsink = pipeline
        .by_name("sink")
        .expect("Could not get sink element");
    let appsink = appsink
        .dynamic_cast::<gstreamer_app::AppSink>()
        .expect("Failed to cast to AppSink");

    // appsink のコールバック登録
    appsink.set_callbacks(
        gstreamer_app::AppSinkCallbacks::builder()
            .new_sample(move |sink| {
                let sample = sink.pull_sample().map_err(|_| {
                    eprintln!("Failed to pull sample");
                    FlowError::Eos
                })?;
                let buffer = sample.buffer().ok_or_else(|| {
                    eprintln!("Failed to get buffer from sample");
                    FlowError::Error
                })?;
                let map = buffer.map_readable().map_err(|_| {
                    eprintln!("Failed to map buffer readable");
                    FlowError::Error
                })?;

                println!("{}", map.len());

                Ok(FlowSuccess::Ok)
            })
            .build(),
    );

    // GStreamerパイプライン再生開始
    pipeline.set_state(gstreamer::State::Playing).unwrap();

    println!("カメラを起動");
    loop {
        sleep(Duration::from_millis(1000));
    }
}
