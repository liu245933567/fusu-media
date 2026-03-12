pub mod config;

use crate::dto::M3u8MergeRequest;
use crate::services::config::get_app_config;
use tokio::process::Command;
use std::path::Path;

pub async fn m3u8merge(params: M3u8MergeRequest) -> anyhow::Result<()> {
    let config = get_app_config().await;
    let ffmpeg_path = config.ffmpeg_path;
    let mut input_path = params.input_path;

    // 判断 input_path 是否是个文件夹，如果是，则寻找文件夹下的 index.m3u8 文件
    if Path::new(&input_path).is_dir() {
        let index_path = Path::new(&input_path).join("index.m3u8");
        if !index_path.exists() {
            return Err(anyhow::anyhow!("index.m3u8 file not found"));
        }
        input_path = index_path.to_str().unwrap().to_string();
    }

    
    let mut output_path = params.output_path;
    // 如果没有以 .mp4 结尾，则强制追加 .mp4 作为最终容器
    if !output_path.to_lowercase().ends_with(".mp4") {
        output_path.push_str(".mp4");
    }
    let output = Command::new(ffmpeg_path)
        // 对齐命令行：
        // ffmpeg -protocol_whitelist file,http,https,tcp,tls,crypto -i <m3u8> -c copy -bsf:a aac_adtstoasc <output.mp4>
        .arg("-protocol_whitelist")
        .arg("file,http,https,tcp,tls,crypto")
        .arg("-i")
        .arg(&input_path)
        .arg("-c")
        .arg("copy")
        .arg("-bsf:a")
        .arg("aac_adtstoasc")
        .arg(&output_path)
        .output()
        .await?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("ffmpeg command failed: {stderr}"));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {stdout}");
    Ok(())
}
