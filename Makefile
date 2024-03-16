# 设置基本的Rust构建命令
CARGO_BUILD = cargo build --release --target

# 定义二进制文件名和目标文件夹
BINARY_NAME = clash_config_manager
BIN_DIR = target/bin

# 定义目标平台数组
TARGETS = x86_64-unknown-linux-musl x86_64-unknown-linux-gnu \
          aarch64-unknown-linux-musl aarch64-unknown-linux-gnu \
          x86_64-pc-windows-gnu aarch64-apple-darwin x86_64-apple-darwin

.PHONY: $(TARGETS) all

# 默认命令 - 构建所有目标
all: $(TARGETS)

# 定义构建规则
$(TARGETS):
	@echo "Adding Rust target $@"
	@rustup target add $@
	@echo "Building for target $@"
	@mkdir -p "$(BIN_DIR)"
	@$(CARGO_BUILD) $@
	@cp "target/$@/release/$(BINARY_NAME)" "$(BIN_DIR)/$@-$(BINARY_NAME)"
