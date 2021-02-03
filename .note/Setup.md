Setup
======
 

```sh

sudo apt update
# 基础
sudo apt install -y cmake pkg-config libssl-dev git gcc build-essential git clang libclang-dev curl
# node.js 测试 node -v
sudo snap install node --channel=14/stable --classic
# yarn 测试 yarn --version
curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -
echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
# yarn 依赖
sudo apt-get install libudev-dev
# rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup default stable
# Latest Nightly
rustup update nightly 
rustup target add wasm32-unknown-unknown --toolchain nightly 
export WASM_BUILD_TYPE=release 添加到 /etc/profile
# 生成密钥的工具 subkey
cargo install --force subkey --git https://github.com/paritytech/substrate --version 2.0.0





git clone https://github.com/paritytech/substrate.git
cd substrate
cargo build
# 好好看帮助信息
cargo run --bin substrate -- --help
# 运行单节点 # -d 是--base-path的简写，用于指定数据目录，即~/.local/share/substrate，由库app-dirs实现
cargo run --bin substrate  -- --dev -d .sub --execution=NativeElseWasm

# 前端
git clone -b v2.0.0 --depth 1 https://github.com/substrate-developer-hub/substrate-front-end-template
cd substrate-front-end-template
yarn install
yarn start
```











