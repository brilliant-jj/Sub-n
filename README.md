# Sub-n
1. 首先，在 substrate-node-template 项目中创建存证模块的文件夹，例如 `claims`。
2. 在 `claims` 文件夹中创建 `lib.rs` 文件，定义存证模块结构体：
3. 在 `claims` 文件夹中创建 `mod.rs` 文件，引入依赖和定义存证模块 Trait：
4. 在 substrate-node-template 项目的 `runtime/src/lib.rs` 文件中引用存证模块，注册模块并添加模块事件：
这样，就完成了存证模块的创建和撤销功能的实现。可以通过 `create_claim` 函数来创建存证，
通过 `revoke_claim` 函数来撤销存证。同时，通过事件 `ClaimCreated` 和 `ClaimRevoked` 来方便地监控数据的变化。
