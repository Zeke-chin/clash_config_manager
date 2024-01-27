# clash_config_manager
用于定时更新clash配置文件的工具

# 使用方法
```shell
❯ ./clash_config_manager -h
Usage: clash_config_manager [OPTIONS] --clash-url <CLASH_URL> --file-path <FILE_PATH>

Options:
  -c, --clash-url <CLASH_URL>                              clash订阅链接
  -f, --file-path <FILE_PATH>                              订阅文件保存路径
  -u, --update-interval-by-secs <UPDATE_INTERVAL_BY_SECS>  更新间隔 [default: 86400]
  -h, --help                                               Print help
  -V, --version                                            Print version
```