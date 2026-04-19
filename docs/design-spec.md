# Snap-rs: macOS Dock 快捷键管理器 — 实现方案

## Context

Snap 是一款 macOS 应用（App Store id418073146），核心功能是自动为 Dock 栏中的应用分配键盘快捷键（Cmd+1 到 Cmd+0），按下快捷键即可启动/切换到对应应用。本方案使用 Tauri v2 + Svelte + TypeScript + Rust 重新实现其全部功能。

## 核心功能

1. 读取 macOS Dock 应用列表（persistent-apps）
2. 为前 10 个 Dock 应用分配全局快捷键 Cmd+1 ~ Cmd+0
3. 按快捷键启动/切换到对应应用
4. 菜单栏常驻运行（无 Dock 图标）
5. 设置窗口：显示 Dock 应用与快捷键映射、开关控制
6. 支持开机启动
7. 轮询检测 Dock 变化（每 5 秒）

## 技术栈

- **后端**: Rust + Tauri v2.10
- **前端**: Svelte 5 + TypeScript
- **包管理器**: pnpm
- **关键依赖**: plist (读 Dock 配置), icns + image (图标提取), base64 (编码图标)

## 项目结构

```
snap-rs/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   ├── capabilities/
│   │   └── default.json
│   ├── icons/                     # tray 图标等
│   └── src/
│       ├── main.rs                # 入口 (~60 行)
│       ├── tray.rs                # 系统托盘 (~80 行)
│       ├── commands.rs            # IPC 命令 (~70 行)
│       ├── dock/
│       │   ├── mod.rs
│       │   ├── reader.rs          # Dock plist 解析 (~120 行)
│       │   └── types.rs           # DockApp 结构体 (~40 行)
│       ├── shortcut/
│       │   ├── mod.rs
│       │   └── manager.rs         # 全局快捷键管理 (~100 行)
│       ├── icon/
│       │   ├── mod.rs
│       │   └── extractor.rs       # 图标提取 (~110 行)
│       └── app_launcher.rs        # 启动/激活应用 (~50 行)
├── src/
│   ├── main.ts
│   ├── App.svelte
│   ├── app.css                    # macOS 原生风格 (~80 行)
│   ├── lib/
│   │   ├── api.ts                 # Tauri invoke 封装 (~60 行)
│   │   └── types.ts               # TS 类型定义 (~30 行)
│   └── components/
│       ├── DockAppList.svelte     # Dock 应用列表 (~90 行)
│       ├── SettingsPanel.svelte   # 设置面板 (~80 行)
│       └── AboutSection.svelte    # 关于信息 (~50 行)
├── index.html
├── package.json
├── tsconfig.json
├── svelte.config.js
└── vite.config.ts
```

## 关键技术决策

| 决策 | 方案 | 理由 |
|------|------|------|
| 读取 Dock 应用 | `plist` crate 直接读取二进制 plist | 无需 spawn 进程，类型安全 |
| 激活应用 | `open -b <bundle_id>` | 简单可靠，自动处理启动/切换 |
| 应用图标 | icns → PNG → base64 data URI | 自包含，无安全问题 |
| Dock 变化检测 | 轮询 5 秒 | plist 仅 15KB，FSEvents 不稳定 |
| 设置持久化 | tauri-plugin-store | Tauri 生态一致性 |
| 菜单栏应用 | ActivationPolicy::Accessory | 标准 macOS 菜单栏应用模式 |

## Rust 后端实现要点

### dock/reader.rs — Dock plist 解析
- 读取 `~/Library/Preferences/com.apple.dock.plist`
- 导航 `persistent-apps` 数组
- 提取 `tile-data` 中的 `file-label`、`bundle-identifier`、`_CFURLString`
- 过滤空白/分隔符条目（`tile-type != "file-tile"` 的跳过）
- 前 10 个分配 Cmd+1~9 和 Cmd+0

### icon/extractor.rs — 图标提取
- 从 app bundle 的 `Contents/Info.plist` 读取 `CFBundleIconFile`
- 加载 `.icns` 文件，提取 64x64 或 128x128 图标
- 编码为 base64 PNG data URI
- 失败时返回 None，前端显示占位图标

### shortcut/manager.rs — 全局快捷键
- 使用 `tauri-plugin-global-shortcut` 注册 `CmdOrCtrl+1` ~ `CmdOrCtrl+0`
- 每个快捷键绑定对应的 `open -b <bundle_id>` 操作
- Dock 变化时先 unregister_all 再重新注册
- 注册失败时 log 错误并跳过

### main.rs — 入口
- 注册 plugins: global-shortcut, autostart, store
- setup 中: 创建托盘、读取 Dock、注册快捷键
- 启动后台轮询线程（5 秒间隔检测 Dock 变化）
- 设置 `ActivationPolicy::Accessory`（无 Dock 图标）
- 窗口关闭事件 → 隐藏而非销毁

### tray.rs — 系统托盘
- 菜单: Settings... / Refresh Dock / About / Quit
- Settings 点击 → 显示/聚焦设置窗口
- Refresh → 重读 Dock + 重注册快捷键

### commands.rs — IPC 命令
- `get_dock_apps` / `refresh_dock_apps`
- `get_shortcuts_enabled` / `set_shortcuts_enabled`
- `get_autostart_enabled` / `set_autostart_enabled`

## Svelte 前端实现要点

### DockAppList.svelte — 应用列表
- 表格展示: 图标 (32x32) + 应用名 + 快捷键标识 (⌘1 等)
- 监听 `dock-changed` 事件自动刷新
- 提供手动 Refresh 按钮

### SettingsPanel.svelte — 设置面板
- "启用快捷键" 开关
- "开机启动" 开关
- macOS 风格 toggle switch

### 样式 — macOS 原生风格
- 字体: `-apple-system, BlinkMacSystemFont`
- 浅色背景 `#f5f5f7`，支持深色模式
- 圆角卡片、微妙阴影

## Tauri 插件与权限

**Plugins:**
- `tauri-plugin-global-shortcut` — 全局快捷键
- `tauri-plugin-autostart` — 开机启动
- `tauri-plugin-store` — 设置持久化

**Capabilities (default.json):**
- `core:default`
- `global-shortcut:allow-register/unregister/unregister-all/is-registered`
- `autostart:allow-enable/disable/is-enabled`
- `store:allow-get/set/save`

## 实现步骤

1. **项目脚手架**: `pnpm create tauri-app` 初始化 Svelte + TS 模板
2. **安装依赖**: npm 插件包 + Cargo.toml Rust 依赖
3. **dock/types.rs + dock/reader.rs**: Dock plist 解析核心
4. **icon/extractor.rs**: 图标提取管线
5. **app_launcher.rs**: 应用启动/切换
6. **shortcut/manager.rs**: 全局快捷键注册
7. **commands.rs**: IPC 命令定义
8. **tray.rs**: 系统托盘
9. **main.rs**: 整合所有模块
10. **前端 types.ts + api.ts**: TS 类型和 API 封装
11. **前端组件**: DockAppList, SettingsPanel, AboutSection, App.svelte
12. **tauri.conf.json + capabilities**: 配置与权限
13. **app.css**: macOS 原生样式
14. **编译测试**: `pnpm tauri dev` 验证所有功能

## 边界情况处理

- Unicode 应用名（中文等）→ 正常支持
- 非标准路径应用 → IO 错误优雅处理
- Dock 分隔符 → 过滤 `dock-extra` 条目
- 不足 10 个应用 → 只注册实际数量的快捷键
- 图标缺失 → 前端显示占位 SVG
- 快捷键冲突 → 日志记录，跳过该快捷键

## 验证方法

1. `pnpm tauri dev` 启动开发模式
2. 验证系统托盘图标和菜单出现
3. 验证设置窗口正确显示 Dock 应用列表
4. 验证 Cmd+1~Cmd+0 快捷键能启动/切换应用
5. 修改 Dock 排列后验证 5 秒内自动更新
6. 验证开机启动开关功能
7. 验证启用/禁用快捷键开关功能
8. `pnpm tauri build` 验证生产构建
