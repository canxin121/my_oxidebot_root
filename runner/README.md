# Canxin OxideBot runner

这个 runner 是 `my_oxidebot_root` 的实际业务程序，只包含：

- `telegram_bot_oxidebot` Telegram 适配器；
- `china_unicom_oxidebot` 中国联通余量查询、凭据续期和通知 Handler；
- Android 四 ABI 交叉编译脚本。

修改依赖后运行：

```sh
cd runner
cargo check
cargo generate-lockfile
```

不要在这里硬编码 Telegram Token 或联通登录 JSON。Telegram Token 通过模块 WebUI/App
写入 `env.conf`；联通登录 JSON 只通过 Telegram 私聊命令导入并存入模块私有数据目录。

Cargo 包名保持为 `oxidebot_app`，因为模块构建脚本按这个名字收集四种 ABI 的二进制。
