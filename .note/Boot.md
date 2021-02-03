# boot
[](https://polkadot.js.org/apps)
./target/release/node-template --dev --tmp

### 生成自己的密钥对，并创建一个使用该密钥对的自定义 chain spec，并基于你的自定义 chain spec 启动一个私有区块链网络。

```shell

# 在尝试启动一个新的网络时，请先清理旧的链数据。
./target/release/node-template purge-chain --base-path /tmp/alice --chain local


./target/release/node-template --help
# --base-path , -d	指定存储数据的目录，有默认值。
# --chain 			指定自己的chain spec文件。 预设: local 即 for a local testnet 。
# --alice			将预定义的Alice密钥(用于产生区块和达到最终确定性) 存入节点的密钥库。 开发者应该自己生成密钥，并通过RPC调用插入。
# --port 			指定监听 p2p 的端口。默认端口 30333 。
# --ws-port 		指定监听 WebSocket 的端口。 默认端口 9944 。
# --rpc-port 		指定监听 RPC 的端口。默认值 9933 。
# --node-key		用于libp2p联网的Ed25519密钥。 一个十六进制编码的32字节密钥，即64个十六进制字符。 该选项的使用应仅限于开发和测试。
# --telemetry-url	节点向特定服务器发送监测数据的地址。
# --validator		表示本节点为 validator ，而不仅仅是同步区块链网络。
# --name            让你可在 telemetry 界面给该节点命名。
#

# Start Alice's node
./target/release/node-template \
  --base-path /tmp/alice \
  --chain local \
  --alice \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator

# Start Bob's node
./target/release/node-template \
  --base-path /tmp/bob \
  --chain local \
  --bob \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
  

# 两个节点运行在同一台物理机上，以下项必须不同 --base-path, --port, --ws-port, --rpc-port 
# --bootnodes，指定引导节点，即3个kv对：ip，tcp，p2p。其中p2p值为引导节点启动日志中 Local node identity is:
#  


#### 重要 log 信息
# Initializing Genesis block/state
# 当你启动下一个节点时，请验证这些值是否相等。
#
# Local node identity is:
# 显示Alice的节点ID, 启动Bob时需要它。 这个值是由--node-key决定的



```

```shell

# https://substrate.dev/docs/zh-CN/knowledgebase/integrate/subkey
# 生成一组助记词{words}，该密钥被 Aura 用于生产区块。
subkey generate --scheme sr25519
# 查看与{words}关联的，该密钥被 GRANDPA 用于达成区块的最终确定性。
subkey inspect --scheme ed25519 "{words}"

# 要启动几个节点就生成几组密钥
```

```shell
# 创建自定义的 chain spec
./target/release/node-template build-spec --disable-default-bootnode --chain local > customSpec.json


#### customSpec.json
# 最长字段 code 就是 runtime 的 Wasm 二进制文件
# 在包含 session 模块的 Substrate 节点中。 json 的 Aura 和 Grandpa 的配置应留空，而在 session 配置里加入这些信息。
# grandpa 协议支持加权投票，grandpa 第二个属性为权重值。 
# 将权限地址 authorities 改为 SS58 Address 。 sr25519 地址放在 aura 部分，ed25519 地址放在 grandpa 部分。

# 编码 customSpec.json
./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
# 将 customSpecRaw.json 与网络中的所有其他验证节点共享。
# chain spec 应由一人单独创建，因为不同人执行 Rust -> Wasm 的优化构建会生成不同hash的binary，如果每个参与者都自己生成文件，将无法达成共识。

```

```shell

./target/release/node-template \
  --base-path /tmp/node01 \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --ws-port 9944 \
  --rpc-port 9933 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode01
  
# 将密钥添加到密钥库
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@rpc_keyA_sr25519.json"
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@rpc_keyA_ed25519.json"


./target/release/node-template \
  --base-path /tmp/node02 \
  --chain ./customSpecRaw.json \
  --port 30334 \
  --ws-port 9945 \
  --rpc-port 9934 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode02 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWDTwSN1t1kfWvUmQRitnKRyX4CQiwTWpZjwmSHKs3tj5c

# 将密钥添加到密钥库
curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d "@rpc_keyB_sr25519.json"
curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d "@rpc_keyB_ed25519.json"


# 只有当超过三分之二的验证者将 GRANDPA 密钥添加到他们的密钥库里时，才能实现区块的最终确定性。
# 即使在为第二个节点添加密钥之后，依然没有区块完成最终确定性 (finalized #0 (0x0ded…9b9d))。 节点在添加 GRANDPA 密钥后需要重启。

```

### 建立一个联盟链
The nodes of Alice and Bob are already configured in genesis storage and serve as well known nodes. 
We will later add Charlie's node into the set of well known nodes. 
Finally we will add the connection between Charlie's node and Dave's node without making Dave's node as a well known node.

[p2p原理](https://docs.rs/sc-network/0.8.0/sc_network/)

```shell
# 生成 PeerId 和 node-key
subkey generate-node-key
# 计算 bs58 decoded peer id in hex 要么自己执行代码，要么 https://whisperd.tech/bs58-codec/

# Start Alice's node
./target/release/node-template --chain=local --base-path ~/tmp/validator1 --alice \
  --node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a \
  --port 30333 --ws-port 9944
# Start Bob's node
./target/release/node-template --chain=local --base-path ~/tmp/validator2 --bob \
  --node-key=6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58 \
  --port 30334 --ws-port 9945
# start Charlie's node ，无法连接到 permissioned network
./target/release/node-template --chain=local --base-path ~/tmp/validator3 \
  --name charlie \
  --node-key=3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e \
  --port 30335 --ws-port=9946 \
  --offchain-worker always
# Charlie 的 node-key 对应的 peer id ：12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ
# 对应的 bs58 decoded peer id in hex: 002408011220876a7b4984f98006dc8d666e28b60de307309835d775e7755cc770328cdacf2e
# 在前端 sudo 界面 nodeAuthorization - add_well_known_node 添加（注意前面加0x）以上值。AccountId 选 Charlie
# 这样 Charlie 就加入了 permissioned network
# 注意：在外网，要加 --reserved-nodes reachable_nodes 启动项，或者 chain_spec 里写上 reachable_nodes ，再 --chain=customSpec.json

# start Dave's node
./target/release/node-template --chain=local --base-path ~/tmp/validator4 \
  --name dave \
  --node-key=a99331ff4f0e0a0434a6263da0a5823ea3afcfffe590c9f3014e6cf620f2b19a \
  --port 30336 --ws-port 9947 \
  --offchain-worker always
# peer id : 12D3KooWPHWFrfaJzxPnqnAYAoRUyAHHKqACmEycGTVmeVhQYuZN
# bs58 decoded peer id in hex : 002408011220c81bc1d7057a1511eb9496f056f6f53cdfe0e14c8bd5ffca47c70a8d76c1326d
# 前端 Charlie 执行 addConnections(Charlie_peerId, Dave_peerId)
# Dave 执行 claimNode(Dave_peerId)
# Dave 执行 addConnections(Dave_peerId, Charlie_peerId)
# Restarting Dave's node

```

### Prometheus + Grafana
https://substrate.dev/docs/zh-CN/tutorials/visualize-node-metrics/
https://github.com/paritytech/substrate/tree/master/utils/prometheus



