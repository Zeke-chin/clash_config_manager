use std::fs;
use std::result::Result;
use std::error::Error;
use serde_yaml::Value;
use chrono::Local;
use reqwest::get;
use async_std::task;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// clash订阅链接
    #[arg(short, long)]
    clash_url: String,

    /// 订阅文件保存路径
    #[arg(short, long)]
    file_path: String,

    /// 更新间隔
    #[arg(short, long, default_value = "86400")]
    update_interval_by_secs: u64,
}

// 数据获取及处理
async fn fetch_and_process(url: &str, now_local: &chrono::DateTime<Local>) -> Result<String, Box<dyn Error>> {
    let response = get(url).await?;

    let s_char = "#".repeat(20);
    let comment = format!("{}\n#\turl: {}\n#\t#更新时间: {}\n{}",
                          s_char,
                          url,
                          now_local.format("%Y-%m-%d %H:%M:%S"),
                          s_char);

    if response.status().is_success() {
        let resp_bytes = response.bytes().await?;
        let mut data: Value = serde_yaml::from_slice(&resp_bytes)?;

        if let Some(obj) = data.as_mapping_mut() {
            obj.insert(Value::String("external-controller".into()), Value::String("0.0.0.0:9090".into()));
            obj.insert(Value::String("external-ui".into()), Value::String("/opt/clash/ui".into()));
        }

        let yaml_string = serde_yaml::to_string(&data)?;
        let save_string = format!("{}\n{}", comment, yaml_string);

        Ok(save_string)
    } else {
        // 抛出错误 by response
        Err(Box::new(response.error_for_status().unwrap_err()))
    }
}


// 写文件
fn write_to_file(path: &str, data: &str) -> std::io::Result<()> {
    fs::write(path, data)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // let url = args.url;
    // let file_path = "response.yml";
    // let update_interval_by_secs = 60;
    println!("服务已经启动.......... \nurl: {} \nfile_path: {} \nupdate_interval_by_secs: {}s\n\n",
             args.clash_url,
             args.file_path,
             args.update_interval_by_secs);

    loop {
        let now_local = Local::now();
        match fetch_and_process(&args.clash_url, &now_local).await {
            Ok(save_string) => {
                write_to_file(&args.file_path, &save_string)?;
                println!("{}\t文件保存成功 -> {}", now_local.format("%Y-%m-%d %H:%M:%S"), args.file_path);
            }
            Err(e) => {
                println!("{}\t文件保存失败: {}", now_local.format("%Y-%m-%d %H:%M:%S"), e);
            }
        }

        task::sleep(std::time::Duration::from_secs(args.update_interval_by_secs)).await;
    }
}
