# Changelog

## 1.0.2

- 中国联通定时通知现在优先展示账号、实际查询跨度以及这段时间新增的通用/免流用量，Android 横幅无需展开即可读到关键信息。
- 到达定时提醒但没有流量变化时，通知会明确说明“近 X 分钟无流量变化”并显示当前余量。
- 手动查询将“本次”改为实际时间跨度（例如“近 5 分钟”），初次查询不会再显示无意义的零秒变化。
- 识别联通 `xsbresources` 附加流量资源组；不再把已成功解析的上游字段名作为用户警告发送。

## 1.0.1

- 修复 China Unicom 插件与 Telegram adapter 同时启用不同 Rustls 加密后端时的启动 panic。
- Telegram adapter 现在为自身 TLS 客户端显式选择 `ring`，不再依赖 Rustls 的进程级全局推断。

## 1.0.0

- 升级至 OxideBot 1.0、官方 Telegram 适配器 1.0 与 China Unicom OxideBot 插件 1.0。
- 使用新插件 API；保留既有联通 SQLite 数据库与账户数据迁移。
- Telegram 配置新增必填的稳定数字 `TELEGRAM_BOT_ID`。
- Android 管理 App 构建升级至 Android Gradle Plugin 9.3.1 与 Gradle 9.6.1。

## 0.1.2

- 修复一条 Telegram 联通命令会被处理两次、导致回复和操作重复执行的问题。
- 只将标准消息事件交给联通文本命令处理器；框架仍可为其他功能保留命令交互事件。

## 0.1.1

- 修复 Android 独立 ELF 启动时 `rustls-platform-verifier` 缺少 JNI 初始化而崩溃的问题。
- 所有联通 HTTP 客户端改用内置 Mozilla WebPKI 根证书，保持完整 TLS 证书和主机名验证。
- 首页运行状态改为每 3 秒自动刷新，不再依赖 Root 管理器 WebView 的可见性报告。
- 页面重新显示、获得焦点、切回首页和 Android App 恢复前台时立即刷新状态。
- 防止多个自动刷新请求并发执行。

## 0.1.0

- 首个 Canxin OxideBot Android Root 版本。
- 集成 Telegram Bot 适配器和中国联通多账号余量查询插件。
- 支持 Magisk、KernelSU、SukiSU Ultra 和 APatch。
- 提供 Root 管理器 WebUI 与独立 Android 管理 App。
