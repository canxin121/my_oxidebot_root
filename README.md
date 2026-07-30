# Canxin OxideBot Root

这是基于 [`oxidebot_root`](https://github.com/canxin121/oxidebot_root) 模板构建的个人
Android Root 应用，只包含 Telegram 接入和中国联通余量查询插件。

安装包同时提供：

- 兼容 Magisk、KernelSU、SukiSU Ultra、APatch 的 Root 模块；
- 可直接在支持 WebUI 的 Root 管理器中打开的控制页面；
- 独立 Android 管理 App，用于查看状态、编辑环境变量、启动、停止和重启 Bot；
- `arm64-v8a`、`armeabi-v7a`、`x86_64`、`x86` 四种 Android ABI。

## 安装与启动

1. 从 [Releases](https://github.com/canxin121/my_oxidebot_root/releases/latest) 下载并安装
   `canxin_oxidebot-v*.zip`。
2. 在 Root 管理器中打开模块 WebUI，或安装并打开配套的
   `canxin_oxidebot-manager-v*.apk`。
3. 在环境变量编辑器中填写从 BotFather 获取的 Token 和 Bot 的稳定数字 ID：

   ```properties
   TELEGRAM_BOT_TOKEN=123456789:replace-with-your-token
   # Token 中冒号前的数字，不是 @username
   TELEGRAM_BOT_ID=123456789
   RUST_LOG=info
   ```

4. 保存配置并点击“启动”。

Token 和 Bot ID 只保存在设备的 `/data/adb/canxin_oxidebot/env.conf`，不应提交到 GitHub、
粘贴到 Issue 或写进 Rust 源码。

## 中国联通账号

所有账号和凭据操作都应在 Telegram 私聊中完成。账号 ID 只能包含 1–32 个 ASCII 字母、
数字、下划线或连字符。

先添加账号：

```text
/china_unicom account add main --name 主卡
```

Bot 提示后，发送由受信任的 China Unicom Login 页面生成的完整四字段 JSON：

```json
{
  "token_online": "...",
  "app_id": "...",
  "cookie": "ecs_token=...; ecs_acc=...",
  "captured_at": "2026-07-27T03:00:07+08:00"
}
```

常用命令：

```text
/china_unicom account list
/china_unicom query
/china_unicom query main
/china_unicom config show main
/china_unicom config set main
/china_unicom task status
/china_unicom task start main
/china_unicom task stop main
/china_unicom account login main
```

四字段 JSON 等同于账号密码，只能在自己的 Bot 私聊中发送。Bot 会拒绝群聊中的联通命令。
数据库位于 `/data/adb/canxin_oxidebot/data/china_unicom/data.db`，卸载模块时默认保留。

完整命令说明见
[`china_unicom_oxidebot`](https://github.com/canxin121/china_unicom_oxidebot#readme)。

## 本地验证

需要 Rust 1.97.1、Android SDK、Android NDK 29 和 Java 17：

```sh
cd runner
cargo check --locked
cargo test --locked

cd ..
bash runner/scripts/build-android.sh aarch64-linux-android
BINARY_DIR=runner/target bash build.sh

cd manager-app
./gradlew assembleDebug
```

## 发布

推送到 `main` 时，GitHub Actions 会运行测试、构建四种 ABI、模块 ZIP 和 Debug 管理 App，
但不会创建 Release。推送与 `template.properties` 中版本一致的 tag，例如 `v1.0.4`，才会
构建签名 APK并发布正式 Release。

正式发布资产包括：

```text
canxin_oxidebot-v1.0.4.zip
canxin_oxidebot-manager-v1.0.4.apk
update.json
```

## 上游与许可证

- [OxideBot](https://github.com/canxin121/oxidebot)
- [OxideBot Telegram Adapter](https://github.com/canxin121/oxidebot/tree/main/crates/oxidebot-adapter-telegram)
- [China Unicom OxideBot](https://github.com/canxin121/china_unicom_oxidebot)
- [OxideBot Root Template](https://github.com/canxin121/oxidebot_root)

本项目使用 `GPL-3.0-only` 许可证。
