# Changelog

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
