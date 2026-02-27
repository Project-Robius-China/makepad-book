# Makepad Component

> https://github.com/Project-Robius-China/makepad-component

面向 [Makepad](https://github.com/makepad/makepad) 的现代 UI 组件库，灵感来自 Longbridge 的 [gpui-component](https://github.com/longbridge/gpui-component)。

![Makepad Component 预览](./images/makepad-component.png)

## 关于 Makepad

[Makepad](https://github.com/makepad/makepad) 是一个用 Rust 编写的新一代 UI 框架，具有以下特性：

- **GPU 加速渲染** - 通过 SDF（有符号距离场）的自定义着色器绘制
- **跨平台** - 桌面端（Windows、macOS、Linux）、移动端（iOS、Android）以及 Web（WebAssembly）
- **实时设计** - 支持热重载的 DSL，快速迭代 UI
- **高性能** - 面向 IDE 与实时工具等高负载应用

### 生产级应用

| 项目 | 说明 |
|---------|-------------|
| [Robrix](https://github.com/project-robius/robrix) | 基于 Makepad 的 Matrix 聊天客户端 |
| [Moly](https://github.com/moxin-org/moly) | AI 模型管理与推理工具 |
| [Makepad Studio](https://github.com/makepad/makepad) | Makepad 官方 IDE |

这些项目由 [Robius](https://github.com/project-robius) 计划推进，致力于跨平台 Rust GUI 开发。

## 特性

### 组件（v0.1.0）

- **Button** - Primary、Secondary、Danger、Ghost 等变体与多种尺寸
- **Checkbox** - 支持标签与半选态
- **Switch** - 带动画的切换开关
- **Radio** - 单选按钮组
- **Divider** - 水平/垂直分隔线
- **Progress** - 线性进度条
- **Slider** - 单值/区间模式、垂直方向、对数刻度、禁用态
- **Badge** - 带变体的通知徽标
- **Tooltip** - 四向定位，边缘检测与自动翻转
- **Input** - 文本输入框

### 即将支持

- Spinner
- Modal
- Dropdown
- Select
- 等等...

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
makepad-component = { git = "https://github.com/Project-Robius-China/makepad-component", branch = "main" }
```

## 用法

```rust
use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_component::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <View> {
                    flow: Down, spacing: 20, padding: 20

                    <MpButtonPrimary> { text: "Primary Button" }
                    <MpCheckbox> { text: "Check me" }
                    <MpSwitch> {}
                    <MpSlider> { value: 50.0, min: 0.0, max: 100.0 }
                }
            }
        }
    }
}
```

## 运行 Demo

### 桌面端

```bash
# 克隆仓库
git clone https://github.com/Project-Robius-China/makepad-component
cd makepad-component

# 运行组件示例
cargo run -p component-zoo
```

### Web（WebAssembly）

```bash
# 安装 cargo-makepad（如未安装）
cargo install --force --git https://github.com/makepad/makepad.git --branch rik cargo-makepad

# 安装 wasm 工具链
cargo makepad wasm install-toolchain

# 构建 Web 版本
cargo makepad wasm build -p component-zoo --release

# 本地启动（需要 Python 3）
python3 serve_wasm.py 8080
# 浏览器访问 http://localhost:8080
```

---

## AI 辅助开发

该组件库与 AI（Claude Code）协作开发，并使用了 [makepad-skills](https://github.com/ZhangHanDong/makepad-skills)。

makepad-skills 是面向 Makepad 开发的 Claude Code 技能集合，覆盖组件创建、着色器编程和生产级模式。

---

## 灵感来源

本项目灵感来源于 Longbridge 的 [gpui-component](https://github.com/longbridge/gpui-component)，它是 GPUI 框架（用于 Zed 编辑器）的组件库。尽管 gpui-component 面向 GPUI，**makepad-component** 将相似的设计理念与组件模式带入 Makepad 生态。

关键差异：
- **Makepad** 使用 `live_design!` DSL，而不是 GPUI 的纯 Rust 方式
- **Makepad** 内置着色器/动画系统
- **Makepad** 支持更多平台（包含移动端与 Web）
