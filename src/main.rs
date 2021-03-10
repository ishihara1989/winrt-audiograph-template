// std
use std::env;
use std::path::Path;
// crate
use tokio::time::{sleep, Duration};
// local
use bindings::{
    windows::Result,
    windows::media::audio::*,
    windows::media::render::*,
    windows::storage::*,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("insufficient args");
        return Ok(());
    }
    let filename = Path::new(&args[1])
        .canonicalize()
        .expect("failed to parse path")
        .into_os_string()
        .into_string()
        .expect("failed to parse path")[4..].to_string(); // avoid UNC (\\?\)
    
    let audio_graph_settings = AudioGraphSettings::create(AudioRenderCategory::Media)?;
    let audio_graph = AudioGraph::create_async(audio_graph_settings)?.await?.graph()?;
    println!("audio graph successfully created: {:?}", audio_graph);
    let output_node = audio_graph.create_device_output_node_async()?.await?.device_output_node()?;
    println!("device output successfully created: {:?}", output_node);
    let file = StorageFile::get_file_from_path_async(filename)?.await?;
    let file_input_node = audio_graph.create_file_input_node_async(file)?.await?.file_input_node()?;
    println!("file input successfully created: {:?}", file_input_node);
    file_input_node.add_outgoing_connection(output_node)?;
    audio_graph.start()?;
    println!("duration: {:}(s)", (file_input_node.duration()?.duration as f64)/10_000_000.0);

    sleep(Duration::from_nanos((file_input_node.duration()?.duration*100) as u64)).await;

    Ok(())
}