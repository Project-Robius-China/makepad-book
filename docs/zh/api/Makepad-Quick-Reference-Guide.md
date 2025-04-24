

# Makepad 速查手册

## Live DSL 速查手册

### **核心概念**

*   **`live_design!{ ... }`**: 定义 UI 结构、样式和组件的宏。
*   **`use link::widgets::*;`**: 导入 Makepad 内置 Widget 库。
*   **`use link::theme::*;`**: 导入当前主题定义的常量（颜色、字体大小、间距等）。
*   **`use link::shaders::*;`**: 导入标准 Shader 库（如 `Sdf2d`, `Pal`, `Math`）。
*   **`ComponentName = {{RustStructName}} { ... }`**: 将 DSL 组件与 Rust 结构体绑定。`ComponentName` 是在 DSL 中使用的名字，`RustStructName` 是对应的 Rust 结构体名。
*   **`instance_name = <ComponentName> { ... }`**: 实例化一个组件。`instance_name` 是这个实例的 `LiveId`，可以在 Rust 代码中通过 `id!(instance_name)` 引用。
*   **`<BaseComponent> { ... }`**: 继承自 `BaseComponent`，并覆盖或添加属性。
*   **`propertyName: value`**: 设置属性值。
*   **`propertyName = { ... }`**: 设置嵌套属性（通常用于 `draw_bg`, `draw_text`, `layout`, `walk`, `animator` 等）。
*   **`id!(...)`**: (在 Rust 代码中) 用于获取 `LiveId`。
*   **`live_id!(...)`**: (在 DSL 中) 用于引用已定义的 `LiveId`（不常用，通常直接用名字）。
*   **`(CONSTANT_NAME)`**: 引用在主题或其他地方定义的常量。
*   **`dep("path/to/resource")`**: 声明一个资源依赖（字体、图片、SVG）。路径通常相对于 `Cargo.toml` 或使用 `crate://self/`，打包的时候需要注意指定资源的真实路径。

### **基础布局与尺寸 (`Walk` & `Layout`)**

*   **`walk`**: 控制元素自身在父容器中的布局。
    *   **`width`, `height`**: ([Size](#size)) 定义尺寸。
        *   `Fill`: 填充父容器可用空间。
        *   `Fit`: 根据内容自适应尺寸。
        *   `Fixed(value)` 或 `value` (如 `100`): 固定像素值。
        *   `All`: 占据父容器所有空间，忽略 padding。
    *   **`margin`**: ([Margin](#margin)) 外边距。
        *   `{top: v, right: v, bottom: v, left: v}`
        *   `v` (简写，四边相等)
        *   `<THEME_MSPACE_1>` (引用主题常量)
    *   **`abs_pos: vec2(x, y)`**: 设置绝对位置（通常用于 `Overlay` 布局）。
*   **`layout`**: 控制元素内部子元素的布局。
    *   **`flow`**: ([Flow](#flow)) 子元素的布局流向。
        *   `Right` (默认): 水平从左到右。
        *   `Down`: 垂直从上到下。
        *   `Overlay`: 堆叠在一起。
        *   `RightWrap`: 水平排列，空间不足时换行。
    *   **`spacing: value`**: 子元素之间的间距（根据 `flow` 方向）。
    *   **`line_spacing: value`**: (`RightWrap`, `Down` flow) 行间距。
    *   **`padding`**: ([Padding](#padding)) 内边距。
        *   `{top: v, right: v, bottom: v, left: v}`
        *   `v` (简写，四边相等)
        *   `<THEME_MSPACE_2>` (引用主题常量)
    *   **`align`**: ([Align](#align)) 子元素在父容器中的对齐方式。
        *   `{x: 0.0, y: 0.0}` (左上)
        *   `{x: 0.5, y: 0.5}` (居中)
        *   `{x: 1.0, y: 1.0}` (右下)
    *   **`clip_x: bool`, `clip_y: bool`**: (默认 `true`) 是否裁剪超出边界的内容。
    *   **`scroll: vec2(x, y)`**: 内容的滚动偏移量。


**Live DSL 中的对齐坐标 (`align`)**

*   **作用域:** `layout` 属性块内，用于控制**父容器**如何**对齐**其**子元素**。
*   **类型:** `Align { x: f64, y: f64 }`
*   **坐标系:** **归一化相对坐标 (Normalized Relative Coordinates)**，范围通常是 0.0 到 1.0。
*   **含义:**
    *   `align.x`: 控制子元素在父容器**可用水平空间**中的对齐方式。
        *   `0.0`: 左对齐 (Left Align)
        *   `0.5`: 水平居中 (Center Align)
        *   `1.0`: 右对齐 (Right Align)
    *   `align.y`: 控制子元素在父容器**可用垂直空间**中的对齐方式。
        *   `0.0`: 顶对齐 (Top Align)
        *   `0.5`: 垂直居中 (Center Align)
        *   `1.0`: 底对齐 (Bottom Align)
*   **关键点:**
    *   `align` 作用于**子元素作为一个整体**（或多个子元素作为一个组，取决于 `flow`）。它决定的是子元素在父容器**剩余空间**中的位置。
    *   它**不**改变子元素内部的坐标系或绘制内容。
    *   只有当父容器的空间**大于**子元素的尺寸时，`align` 才有可见效果。如果子元素设置了 `width: Fill` 或 `height: Fill`，那么在该方向上通常没有剩余空间，对齐也就没有意义了。
    *   值的范围可以超出 0.0-1.0，用于将元素部分或完全移出父容器的可见区域（需要父容器 `clip_x/clip_y: false`）。

*   **示例:**
    ```live_design!
    <View> { // 父容器
        width: 200, height: 100,
        show_bg: true, draw_bg: { color: #eee }
        padding: 10,
        // 子元素将在父容器的可用空间内右下角对齐
        align: { x: 1.0, y: 1.0 }

        <Button> { // 子元素
            text: "OK",
            width: Fit, height: Fit // 尺寸由内容决定
        }
    }
    ```


### **基础绘制 (`Draw*` 类型)**

*   **`DrawColor`**: 绘制纯色背景。
    *   `color: #RRGGBBAA` 或 `(THEME_COLOR_...)`
    *   `fn pixel(self) -> vec4 { ... }`: 可覆盖的像素着色器。
*   **`DrawQuad`**: 绘制四边形的基础，其他 `Draw*` 通常继承自它。包含基本的顶点变换和裁剪逻辑。
    *   `draw_depth: f32` (默认 `1.0`): 控制绘制深度/层级。
    *   `draw_zbias: f32` (通常内部使用): 微调深度以避免 Z-fighting。
*   **`DrawText` / `DrawText2`**: 绘制文本。
    *   `text_style: <TextStyleName> { ... }`: 设置文本样式。
    *   `color: #RRGGBBAA` 或 `(THEME_COLOR_...)`
    *   `wrap: Word / Line / Ellipsis` (Ellipsis 可能未完全支持)
    *   `font_scale: f64` (默认 `1.0`)
    *   `brightness: f32` (旧版，可能已弃用)
    *   `curve: f32` (旧版，可能已弃用)
    *   `fn get_color(self) -> vec4 { ... }`: 可覆盖的颜色逻辑（常用于动画）。
*   **`DrawIcon`**: 绘制 SVG 图标。
    *   `svg_file: dep("...")` 或 `svg_path: "..."`
    *   `color: #RRGGBBAA` 或 `(THEME_COLOR_...)` (图标颜色)
    *   `brightness: f32`
    *   `curve: f32`
    *   `fn get_color(self) -> vec4 { ... }`: 可覆盖的颜色逻辑。
*   **`DrawLine`**: 绘制线条 (继承自 `DrawQuad`)。
    *   `color: #RRGGBBAA`
    *   `line_width: f64` (在 Rust 中设置)
*   **`DrawScrollShadow`**: 绘制滚动阴影。
    *   `shadow_size: f32`

### **SDF 绘制 (`Sdf2d` in `fn pixel`)**

在 `fn pixel` 中使用 `Sdf2d` 进行矢量绘制。

*   **初始化:** `let sdf = Sdf2d::viewport(self.pos * self.rect_size);`

**SDF 绘制 (`Sdf2d` in `fn pixel`) - 基本形状**

在 `fn pixel` 函数中，你可以使用 `Sdf2d` 对象来定义和组合形状。以下是常用的基本形状函数及其说明：

*   **`sdf.circle(cx: float, cy: float, radius: float)`**
    *   **说明:** 绘制一个圆形。
    *   **参数:**
        *   `cx`, `cy`: 圆心的 x 和 y 坐标。
        *   `radius`: 圆的半径。
    *   **作用:** 更新 `sdf.dist` 为当前像素点到该圆形边界的有符号距离。内部为负，外部为正。`sdf.shape` 会被更新为 `min(sdf.shape, sdf.dist)`。

*   **`sdf.rect(x: float, y: float, w: float, h: float)`**
    *   **说明:** 绘制一个直角矩形。
    *   **参数:**
        *   `x`, `y`: 矩形左下角的 x 和 y 坐标。
        *   `w`, `h`: 矩形的宽度和高度。
    *   **作用:** 更新 `sdf.dist` 为当前像素点到该矩形边界的有符号距离。`sdf.shape` 会被更新为 `min(sdf.shape, sdf.dist)`。

*   **`sdf.box(x: float, y: float, w: float, h: float, radius: float)`**
    *   **说明:** 绘制一个所有角都具有相同圆角半径的矩形。
    *   **参数:**
        *   `x`, `y`: 矩形左下角的 x 和 y 坐标。
        *   `w`, `h`: 矩形的宽度和高度。
        *   `radius`: 所有四个角的圆角半径。
    *   **作用:** 更新 `sdf.dist` 为当前像素点到该圆角矩形边界的有符号距离。`sdf.shape` 会被更新为 `min(sdf.shape, sdf.dist)`。

*   **`sdf.box_x(x: float, y: float, w: float, h: float, r_left: float, r_right: float)`**
    *   **说明:** 绘制一个矩形，其左右两侧可以有不同的圆角半径。左上和左下角使用 `r_left`，右上和右下角使用 `r_right`。
    *   **参数:**
        *   `x`, `y`: 矩形左下角的 x 和 y 坐标。
        *   `w`, `h`: 矩形的宽度和高度。
        *   `r_left`: 左侧两个角的圆角半径。
        *   `r_right`: 右侧两个角的圆角半径。
    *   **作用:** 更新 `sdf.dist` 为当前像素点到该特定圆角矩形边界的有符号距离。`sdf.shape` 会被更新为 `min(sdf.shape, sdf.dist)`。

*   **`sdf.box_y(x: float, y: float, w: float, h: float, r_top: float, r_bottom: float)`**
    *   **说明:** 绘制一个矩形，其上下两侧可以有不同的圆角半径。左上和右上角使用 `r_top`，左下和右下角使用 `r_bottom`。
    *   **参数:**
        *   `x`, `y`: 矩形左下角的 x 和 y 坐标。
        *   `w`, `h`: 矩形的宽度和高度。
        *   `r_top`: 顶部两个角的圆角半径。
        *   `r_bottom`: 底部两个角的圆角半径。
    *   **作用:** 更新 `sdf.dist` 为当前像素点到该特定圆角矩形边界的有符号距离。`sdf.shape` 会被更新为 `min(sdf.shape, sdf.dist)`。

*   **`sdf.box_all(x: float, y: float, w: float, h: float, r_left_top: float, r_right_top: float, r_right_bottom: float, r_left_bottom: float)`**
    *   **说明:** 绘制一个矩形，允许为每个角指定单独的圆角半径。
    *   **参数:**
        *   `x`, `y`: 矩形左下角的 x 和 y 坐标。
        *   `w`, `h`: 矩形的宽度和高度。
        *   `r_left_top`: 左上角的圆角半径。
        *   `r_right_top`: 右上角的圆角半径。
        *   `r_right_bottom`: 右下角的圆角半径。
        *   `r_left_bottom`: 左下角的圆角半径。
    *   **作用:** 更新 `sdf.dist` 为当前像素点到该高度自定义圆角矩形边界的有符号距离。`sdf.shape` 会被更新为 `min(sdf.shape, sdf.dist)`。

*   **`sdf.hexagon(cx: float, cy: float, radius: float)`**
    *   **说明:** 绘制一个正六边形。
    *   **参数:**
        *   `cx`, `cy`: 六边形中心的 x 和 y 坐标。
        *   `radius`: 从中心到任一顶点的距离。
    *   **作用:** 更新 `sdf.dist` 为当前像素点到该六边形边界的有符号距离。`sdf.shape` 会被更新为 `min(sdf.shape, sdf.dist)`。

*   **`sdf.hline(y: float, half_thickness: float)`**
    *   **说明:** 绘制一条水平线。这条线在概念上是无限长的，但通常会被视口或父容器裁剪。
    *   **参数:**
        *   `y`: 水平线的中心 y 坐标。
        *   `half_thickness`: 线条厚度的一半（总厚度为 `2 * half_thickness`）。
    *   **作用:** 更新 `sdf.dist` 为当前像素点到该水平线边界的有符号距离。`sdf.shape` 会被更新为 `min(sdf.shape, sdf.dist)`。

*   **`sdf.move_to(x: float, y: float)`**
    *   **说明:** 开始一个新的路径或子路径，将当前绘图点移动到指定的 `(x, y)` 坐标，**不绘制任何线条**。它会更新 `sdf.start_pos` 和 `sdf.last_pos`。
    *   **参数:**
        *   `x`, `y`: 新路径起点的坐标。
    *   **作用:** 设置路径起点，用于后续的 `line_to` 或 `close_path`。

*   **`sdf.line_to(x: float, y: float)`**
    *   **说明:** 从当前点 (`sdf.last_pos`) 绘制一条直线段到指定的 `(x, y)` 坐标。这条线段会成为当前形状的一部分。它会更新 `sdf.last_pos`。
    *   **参数:**
        *   `x`, `y`: 线段终点的坐标。
    *   **作用:** 向当前路径添加一条直线段，并更新 `sdf.dist` 和 `sdf.shape` 以反映到这条线段的距离。同时，它还会更新 `sdf.clip` 用于路径填充。

*   **`sdf.close_path()`**
    *   **说明:** 通过绘制一条从当前点 (`sdf.last_pos`) 回到当前子路径起点 (`sdf.start_pos`) 的直线段来闭合当前的子路径。
    *   **作用:** 完成一个封闭形状的定义。

*   **`sdf.arc2(cx: float, cy: float, radius: float, start_angle: float, end_angle: float)`**
    *   **说明:** 绘制一个圆弧。
    *   **参数:**
        *   `cx`, `cy`: 圆弧圆心的 x 和 y 坐标。
        *   `radius`: 圆弧的半径。
        *   `start_angle`: 圆弧的起始角度（弧度）。
        *   `end_angle`: 圆弧的结束角度（弧度）。
    *   **作用:** 更新 `sdf.dist` 为当前像素点到该圆弧边界的有符号距离。`sdf.shape` 会被更新为 `min(sdf.shape, sdf.dist)`。

**重要提示:**

*   这些函数仅仅是**定义**形状的边界（通过计算 Signed Distance Field）。
*   要实际看到这些形状，你必须在定义形状之后调用一个**绘制函数**，如 `sdf.fill(color)`, `sdf.stroke(color, width)`, `sdf.glow(color, width)` 等。
*   `sdf.shape` 存储了当前已定义形状的最小 SDF 值（用于 `union`），而 `sdf.old_shape` 用于布尔运算（`intersect`, `subtract`, `gloop`）。
*   `sdf.dist` 存储了**最新**定义的那个基本形状的 SDF 值。

*   **布尔运算:** (通常在绘制形状后调用)
    *   `sdf.union()`: 合并当前形状和 `old_shape`。
    *   `sdf.intersect()`: 取当前形状和 `old_shape` 的交集。
    *   `sdf.subtract()`: 从 `old_shape` 中减去当前形状。
*   **填充/描边:**
    *   `sdf.fill(color)`: 填充当前形状并重置形状状态。
    *   `sdf.fill_keep(color)`: 填充当前形状，保留形状状态。
    *   `sdf.fill_premul(premultiplied_color)`: 使用预乘 Alpha 填充并重置。
    *   `sdf.fill_keep_premul(premultiplied_color)`: 使用预乘 Alpha 填充并保留状态。
    *   `sdf.stroke(color, width)`: 描边当前形状并重置形状状态。
    *   `sdf.stroke_keep(color, width)`: 描边当前形状，保留形状状态。
*   **效果:**
    *   `sdf.glow(color, width)`: 添加辉光并重置形状状态。
    *   `sdf.glow_keep(color, width)`: 添加辉光，保留形状状态。
    *   `sdf.gloop(k)`: 与 `old_shape` 进行平滑混合 (Metaball 效果)。
*   **变换:** (影响后续绘制)
    *   `sdf.translate(dx, dy)`
    *   `sdf.rotate(angle_rad, pivot_x, pivot_y)`
    *   `sdf.scale(factor, pivot_x, pivot_y)`
*   **清除:** `sdf.clear(color)`: 用指定颜色覆盖当前结果。
*   **结果:** `sdf.result` (最终的 `vec4` 颜色)。



**Shader 中的坐标 (`self.pos`, `self.geom_pos`, `Sdf2d::viewport` 参数)**

*   **作用域:** `draw_*` 块内的 `fn pixel` 和 `fn vertex` 函数。
*   **类型:** `vec2` (通常是 `f32` 类型)
*   **坐标系:** **归一化局部坐标 (Normalized Local Coordinates)**，范围通常是 0.0 到 1.0。
*   **含义:**
    *   **`self.pos` (在 `fn pixel` 中):** 表示当前正在计算的像素点，在其所属的绘制矩形 (`self.rect_size`) 内的**归一化坐标**。
        *   左上角通常对应 `vec2(0.0, 0.0)`。
        *   右下角通常对应 `vec2(1.0, 1.0)`。
        *   这个坐标是插值计算得到的，用于纹理采样或根据位置计算颜色/形状。
    *   **`self.geom_pos` (在 `fn vertex` 中):** 表示当前正在处理的顶点，在其所属的输入几何体（通常是一个单位四边形）内的**归一化坐标**。
        *   对于默认的 `GeometryQuad2D`，顶点通常是 `(0,0)`, `(1,0)`, `(1,1)`, `(0,1)`。
        *   这个坐标用于将几何体顶点映射到屏幕上的 `rect_pos` 和 `rect_size` 定义的矩形区域。
    *   **`Sdf2d::viewport(pos: vec2)` 的参数:** 这个 `pos` 参数通常是**像素坐标**或**相对于某个局部原点的坐标**，而不是归一化的。
        *   常见的用法是 `Sdf2d::viewport(self.pos * self.rect_size)`。这里 `self.pos` 是 0-1 的归一化坐标，乘以 `self.rect_size` 将其转换为相对于当前绘制矩形左上角的**像素级局部坐标**。SDF 函数（如 `sdf.circle(cx, cy, r)`）期望接收这种像素级的局部坐标。

*   **关键点:**
    *   Shader 中的坐标是**局部的**，与 Widget 在屏幕上的最终位置无关（最终位置由 `vertex` 函数的返回值和变换矩阵决定）。
    *   `self.pos` 在 `fn pixel` 中非常重要，因为它允许你根据像素在 Widget 矩形内的相对位置来计算颜色、图案或形状。
    *   `self.geom_pos` 在 `fn vertex` 中用于将基础几何体（通常是 0-1 的四边形）映射到屏幕上的目标矩形。
    *   传递给 `Sdf2d` 函数的坐标通常是相对于 `Sdf2d::viewport` 创建时所基于的那个坐标系的（通常是 `self.pos * self.rect_size` 产生的局部像素坐标）。

*   **示例:**
    ```live_design!
    <View> {
        show_bg: true,
        draw_bg: {
            fn pixel(self) -> vec4 {
                // self.pos.x 从左到右从 0.0 渐变到 1.0
                // self.pos.y 从上到下从 0.0 渐变到 1.0
                // 创建一个从左(红)到右(绿)的水平渐变
                return mix(#f00, #0f0, self.pos.x);
            }
        }
    }
    ```
    ```live_design!
    <View> {
        show_bg: true,
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // 在局部像素坐标 (10, 10) 处绘制一个半径为 5 的圆
                sdf.circle(10.0, 10.0, 5.0);
                sdf.fill(#fff);
                return sdf.result;
            }
        }
    }
    ```

**总结:**

| 概念                  | 作用域         | 类型             | 坐标系                   | 含义                        |
| :------------------ | :---------- | :------------- | :-------------------- | :------------------------ |
| **`align`**         | `layout` 块  | `Align`        | 归一化相对 (0-1)           | 控制**子元素**在父容器**剩余空间**中的对齐 |
| **`self.pos`**      | `fn pixel`  | `vec2`         | 归一化局部 (0-1)           | 当前像素在自身绘制矩形内的相对位置         |
| **`self.geom_pos`** | `fn vertex` | `vec2`         | 归一化几何体 (0-1)          | 当前顶点在输入几何体内的相对位置          |
| **`Sdf2d` 参数**      | `fn pixel`  | `float`/`vec2` | 局部像素/任意（取决于 viewport） | 用于定义 SDF 形状的坐标            |

理解这两种坐标系的区别对于精确控制 Makepad UI 的布局和视觉效果至关重要。`align` 用于宏观布局，而 Shader 中的 `self.pos` 用于微观的像素级绘制。

### **动画 (`animator`)**

`animator` 块用于定义 Widget 的状态以及这些状态之间的过渡动画。它允许你以声明式的方式创建交互式的视觉反馈。

#### **基本结构**

```live_design!
MyWidget = {{MyWidget}} {
    // ... 其他属性 ...
    animator: {
        // 定义一个或多个状态动画轨迹
        track_name1 = { ... }
        track_name2 = { ... }
        // ...
    }
}
```

*   `animator`: 包含所有动画定义的块。
*   `track_name`: 一个动画轨迹的名称（LiveId），通常对应一个逻辑状态（如 `hover`, `focus`, `open`, `selected`, `visible` 等）。这个名称将在 Rust 代码中通过 `id!(track_name)` 引用。

#### **状态定义 (`state_name = { ... }`)**

在每个 `track_name` 内部，通常定义至少两个状态，最常见的是 `on` 和 `off`，也可以是自定义的状态名。

```rust
animator: {
    hover = { // 轨迹名称
        default: off, // 初始状态

        off = { // 'off' 状态的定义
            // ... 动画参数 ...
            apply: { /* ... 属性目标值 ... */ }
        }

        on = { // 'on' 状态的定义
            // ... 动画参数 ...
            apply: { /* ... 属性目标值 ... */ }
        }

        // 可以定义更多自定义状态名
        // custom_state = { ... }
    }
}
```

*   **`default: on | off`**: 指定 Widget 初始化时此动画轨迹的默认状态。
*   **`state_name = { ... }`**: 定义一个具体状态（如 `on`, `off`, `down`, `open`, `close` 等）。

#### **状态转换参数**

在每个具体的状态定义块（如 `on = { ... }`）内部，可以设置以下参数来控制动画如何进入该状态：

*   **`from: { ... }`**: 定义从**其他状态**转换到**当前状态**时的动画行为。
    *   **`all: PlayMode`**: 为所有未明确指定的来源状态设置默认的过渡动画。
    *   **`other_state_name: PlayMode`**: 为从特定的 `other_state_name` 状态转换到当前状态设置动画。这会覆盖 `all` 的设置。
    *   **`PlayMode`**: 定义动画的播放方式和时长：
        *   `Forward { duration: seconds }`: 从当前值正向动画到目标值，持续 `seconds` 秒。
        *   `Backward { duration: seconds }`: 从目标值反向动画到当前值（较少直接使用，通常由 `Forward` 自动处理反向）。
        *   `Snap`: 立即跳转到目标值，没有动画。
        *   `Loop { duration: seconds, end: value }`: 循环播放动画，`duration` 是一个周期的时长，`end` 是循环点（通常是 `1.0`）。
        *   `ReverseLoop { ... }`: 反向循环。
        *   `BounceLoop { ... }`: 来回循环（乒乓效果）。
*   **`ease: EaseType { ... }`**: 指定缓动函数，控制动画的速度曲线。
    *   **常用:** `Linear`, `InQuad`, `OutQuad`, `InOutQuad`, `InExp`, `OutExp`, `InOutExp`, `ExpDecay {d1:v, d2:v}` (模拟物理衰减)。
    *   **其他:** `In/Out/InOut` + `Cubic`, `Quart`, `Quint`, `Sine`, `Circ`, `Elastic`, `Back`, `Bounce`。
    *   **自定义:** `Pow { begin: v, end: v }`, `Bezier { cp0: v, cp1: v, cp2: v, cp3: v }`。
*   **`apply: { ... }`**: 定义当动画**到达**这个状态时，哪些 `#[live]` 属性应该具有的目标值。
    *   **`property: value`**: 将属性 `property` 的最终值设置为 `value`。
    *   **`property: [{ time: t, value: v }, ...]`**: 定义关键帧动画。
        *   `time`: 归一化时间 (0.0 到 1.0)。
        *   `value`: 在该时间点属性应具有的值。
        *   第一个关键帧的 `time` 通常是 `0.0`，代表动画开始时的值（通常是 `from` 状态的值，但可以覆盖）。
        *   最后一个关键帧的 `time` 通常是 `1.0`，代表动画结束时的值。
    *   **属性路径:** 可以是嵌套的，如 `draw_bg: { hover: 1.0 }` 或 `walk: { margin: { left: 10.0 } }`。
*   **`redraw: true`**: 标记这个动画轨迹在播放时需要不断重绘 Widget。对于视觉上会改变的动画，这通常是必需的。
*   **`cursor: MouseCursor`**: (通常在 `on` 状态中) 当动画进入此状态时，设置鼠标光标样式（如 `Hand`, `Default`）。

#### **工作原理**

1.  **状态触发 (Rust):** 在 Rust 代码的 `handle_event` 中，根据用户交互（如 `Hit::FingerHoverIn`, `Hit::FingerDown`）或其他逻辑，调用 `self.animator_play(cx, id!(track_name.state_name))` 或 `self.animator_toggle(cx, condition, animate, id!(track.on), id!(track.off))` 来触发状态转换。
2.  **查找动画参数:** Makepad 的动画系统查找目标状态（如 `on`）的定义，并根据当前状态查找对应的 `from` 规则来确定动画的 `duration` 和 `ease`。
3.  **插值计算:** 动画系统根据时间、`duration` 和 `ease` 函数，计算出 `apply` 块中指定的每个属性在当前帧应该具有的中间值。
4.  **应用值:** 将计算出的中间值应用回 Rust 结构体中对应的 `#[live]` 字段。
5.  **重绘:** 如果动画轨迹标记了 `redraw: true` 或 `animator_handle_event` 返回 `must_redraw()`，则请求重绘 Widget。
6.  **绘制:** 在 `draw_walk` 中，Widget 使用被动画更新后的 `#[live]` 字段值进行绘制。

**示例：按钮的 Hover 和 Down 状态**

```live_design!
Button = {{Button}} {
    // ... 其他属性 ...
    animator: {
        hover = {
            default: off,
            off = {
                // 从任何状态回到 off 状态，持续 0.1 秒
                from: {all: Forward {duration: 0.1}}
                apply: {
                    draw_bg: {hover: 0.0} // 背景 hover 因子为 0
                    draw_text: {hover: 0.0} // 文本 hover 因子为 0
                }
            }
            on = {
                cursor: Hand, // 进入 on 状态时显示手形光标
                // 从 off 状态进入 on 状态，持续 0.1 秒
                // 从 down 状态进入 on 状态，立即完成 (Snap)
                from: {all: Forward {duration: 0.1}, down: Snap}
                apply: {
                    // 使用关键帧，立即将 hover 因子设为 1.0
                    draw_bg: {hover: [{time: 0.0, value: 1.0}]}
                    draw_text: {hover: [{time: 0.0, value: 1.0}]}
                }
            }
            down = { // 按下状态
                // 从任何状态进入 down 状态，持续 0.2 秒
                from: {all: Forward {duration: 0.2}}
                apply: {
                    // 使用关键帧，立即将 down 因子设为 1.0，保持 hover 为 1.0
                    draw_bg: {down: [{time: 0.0, value: 1.0}], hover: 1.0}
                    draw_text: {down: [{time: 0.0, value: 1.0}], hover: 1.0}
                }
            }
        }
        // 可以有其他动画轨迹，如 focus, enabled 等
    }
}
```


在 Makepad 中，实现**平滑的视觉动画**（例如 UI 元素的位置、大小、颜色、透明度等的渐变）**强烈推荐使用 `NextFrame` 事件机制，而不是 `Timer`**。

`Animator` 系统内部就是基于 `NextFrame` 来驱动动画更新和重绘的。

**`NextFrame` (用于动画)**

*   **目的:** 主要用于驱动**渲染相关的、逐帧更新**的动画。
*   **触发方式:**
    *   当 Widget 调用 `self.redraw(cx)` 或 `self.area.redraw(cx)` 时，Makepad 会在**下一帧**渲染循环开始时发送一个 `Event::NextFrame` 事件给该 Widget（以及其他需要重绘的 Widget）。
    *   Widget 可以在 `handle_event` 中捕获这个 `NextFrame` 事件 (`if let Some(ne) = self.next_frame.is_event(event)`), 根据时间差 (`ne.time`) 计算动画的下一状态，更新 `#[live]` 属性或 `instance` 变量，然后**再次调用 `self.redraw(cx)`** 来请求再下一帧的更新。
    *   `Animator` 系统自动化了这个过程：`animator_handle_event` 内部检查动画是否在进行，如果是，则计算下一帧的值，应用它们，并返回 `must_redraw()` 为 `true`，这通常会触发外部调用 `self.redraw(cx)`。
*   **优点:**
    *   **与渲染同步:** `NextFrame` 事件与 Makepad 的渲染循环紧密耦合，确保动画更新与屏幕刷新同步，从而获得最平滑的视觉效果。
    *   **高效:** 只有在需要重绘时才会触发，避免了不必要的计算和唤醒。当动画结束或 Widget 不可见时，不会再发送 `NextFrame`（除非有其他原因需要重绘）。
    *   **简单:** 对于 UI 动画，通常只需要在状态改变时启动动画 (`animator_play`)，并在 `NextFrame` 事件中更新状态（或让 `Animator` 处理），然后请求重绘即可。
*   **缺点:**
    *   不适合需要精确时间间隔的逻辑（因为它依赖于渲染帧率）。
    *   如果渲染帧率下降，动画速度也会变慢。

**`Timer` (用于定时逻辑)**

*   **目的:** 用于执行**与渲染帧率无关**的、基于**固定时间间隔**的操作。
*   **触发方式:**
    *   通过 `cx.start_timer(id, interval, repeats)` 或 `cx.start_timeout(id, interval)` 启动一个定时器。
    *   当设定的时间间隔到达后，Makepad 会发送一个 `Event::Timer` 事件，其中包含触发的定时器 ID。
    *   Widget 在 `handle_event` 中捕获这个 `Timer` 事件 (`if self.my_timer.is_event(event).is_some()`) 并执行相应的逻辑。
*   **优点:**
    *   **时间精确:** 适合需要按固定时间间隔执行的任务，如轮询、定时检查、节流/防抖等。
    *   **与渲染解耦:** 不受渲染帧率波动的影响（但事件处理本身仍在主线程）。
*   **缺点:**
    *   **不适合平滑动画:** 定时器的触发间隔可能与屏幕刷新率不同步，导致动画看起来卡顿或不连贯。
    *   **可能唤醒过多:** 如果间隔设置得很短，即使界面没有变化，也会频繁唤醒事件循环。


**核心思想：区分“动画过程”和“延时触发”**

1.  **动画过程 (Animation Process):** 指的是一个属性值从 A 平滑地过渡到 B 的**整个过程**。例如：
    *   弹窗从屏幕外 (`abs_pos: vec2(-1000.0, 10.0)`) **滑动到**屏幕内 (`abs_pos: vec2(60.0, 10.0)`)。
    *   进度条的高度从 0 **增长到** 100%。
    *   按钮的背景色从灰色**渐变到**蓝色。

    对于这种**平滑的、逐帧变化的视觉过渡**，你应该**依赖 `Animator` 和 `NextFrame`**。`Animator` 会在每一帧（由 `NextFrame` 触发）计算出属性的中间值，并更新它，然后请求下一帧重绘，直到动画完成。**你不应该使用 `Timer` 来手动控制这种逐帧更新**，因为 `Timer` 的触发间隔与屏幕刷新率无关，会导致动画卡顿。

2.  **延时触发 (Delayed Trigger):** 指的是在**一段时间之后**执行一个**一次性**的操作。例如：
    *   弹窗显示 **2.5 秒后**，自动**开始**关闭动画（或者立即关闭）。
    *   用户停止输入 **500 毫秒后**，自动保存草稿。
    *   点击按钮后，**延迟 1 秒** 才发送网络请求。

    对于这种**在特定时间点触发某个动作**的需求，你应该**使用 `Timer`**。`cx.start_timeout(duration)` 就是为此设计的。当定时器触发时 (`my_timer.is_event(event).is_some()`)，你执行相应的操作，比如调用 `self.close(cx)` 来**启动**关闭动画。

**比如：robrix 项目中  `RobrixPopupNotification` 组件弹框动画:**

*   **打开动画（滑动进入）:** 当调用 `self.open(cx)` 时，你使用 `self.animator.animator_play(cx, id!(mode.open))` 来**启动**滑动动画。这个动画的**过程**是由 `Animator` 和 `NextFrame` 驱动的。**这里不需要 `Timer`。**
*   **进度条动画:** 当调用 `self.open(cx)` 时，你使用 `self.view(id!(content.progress)).animator_play(cx, id!(mode.progress))` 来**启动**进度条动画。这个动画的**过程**也是由 `Animator` 和 `NextFrame` 驱动的。**这里不需要 `Timer`。**
*   **自动关闭:** 你希望弹窗显示一段时间后自动关闭。这个“一段时间后”的操作，就是**延时触发**。因此，在 `self.open(cx)` 中启动一个 `Timer` (`self.animation_timer = cx.start_timeout(2.5);`) 是**正确**的。当这个 `Timer` 在 2.5 秒后触发时 (`if self.animation_timer.is_event(event).is_some()`)，你调用 `self.close(cx)`。
*   **关闭动画（滑出）:** 当调用 `self.close(cx)` 时（无论是被定时器触发还是被点击关闭按钮触发），你使用 `self.animator.animator_play(cx, id!(mode.close))` 来**启动**滑出动画。这个动画的**过程**同样是由 `Animator` 和 `NextFrame` 驱动的。**这里不需要 `Timer`。**

因此

*   **用 `Animator` + `NextFrame` 实现平滑的视觉过渡动画（滑动、渐变、缩放等）。**
*   **用 `Timer` (特别是 `cx.start_timeout`) 来实现在固定延迟后触发某个一次性动作（比如启动一个关闭动画）。**


**最佳实践:**

*   **使用有意义的轨迹和状态名:** 让代码更易读（如 `hover`, `focus`, `open`, `selected`）。
*   **保持动画简洁:** 过多或过长的动画可能会让用户感到不适。
*   **利用 `default`:** 设置好初始状态。
*   **考虑 `from` 规则:** 定义不同状态转换时的动画行为，使过渡更自然。
*   **使用 `redraw: true`:** 确保视觉变化能够被渲染。
*   **将动画逻辑放在 DSL 中:** 尽可能用声明式的方式定义动画，而不是在 Rust 中手动计算插值。
*   **动画 `instance` 变量:** 在 `draw_*` shader 中定义 `instance` 变量（如 `instance hover: float`），并在 `animator` 的 `apply` 块中修改这些变量，然后在 `fn pixel` 或 `fn get_color` 中使用 `mix()` 或其他逻辑来根据这些 instance 变量改变外观。
*   **对于所有视觉动画（位置、大小、颜色、透明度等渐变），优先使用 `NextFrame` 机制。** 最简单的方式是利用 Makepad 内置的 `Animator` 系统，它封装了基于 `NextFrame` 的动画逻辑。
*   **只有在需要精确时间间隔执行非渲染相关的逻辑时，才使用 `Timer`。** 例如：
    *    `PopupNotification` 使用 `Timer` 在固定时间后自动关闭，这是 `Timer` 的一个合适用例。
    *   网络请求的轮询或超时。
    *   定期保存用户数据。
    *   实现节流（Throttling）或防抖（Debouncing）逻辑。



### **Makepad Live DSL 错误信息速查表**

| 错误信息模式 (Error Message Pattern)                                  | 可能的原因 (Possible Causes)                                                                                                                               | 常见场景与提示 (Common Scenarios & Tips)                                                                                                                                                              |
| :-------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Enum variant X not found for field Y`                                | 1. 枚举变体名称拼写错误或大小写错误。<br>2. 使用了该枚举类型不支持的变体。                                                                                       | 检查 `Flow` (Right, Down, Overlay, RightWrap), `Size` (Fill, Fit, Fixed, All), `Axis` (Horizontal, Vertical), `CheckType`, `RadioType`, `ImageFit` 等枚举的拼写和可用值。查阅文档或 Rust 定义。 |
| `Identifier X not found`                                              | 1. 引用的 LiveId（组件实例名、常量名）拼写错误或大小写错误。<br>2. 引用的 LiveId 未在当前或父作用域定义。<br>3. 忘记 `use link::...` 导入。                               | 检查实例名、常量名（如 `THEME_COLOR_TEXT`）是否正确。确保在使用前已定义。检查 `use` 语句。                                                                                                     |
| `field X not found on struct Y`                                       | 1. 字段名 `X` 拼写错误或大小写错误。<br>2. 组件 `Y` (或其基类) 没有名为 `X` 的 `#[live]` 字段。<br>3. 尝试在错误的层级设置属性（例如，直接在 `<Button>` 上设置 `text_style` 而不是在 `draw_text` 内部）。 | 检查属性名的拼写。查阅组件的 Rust 定义或文档确认可用属性及其层级。例如，文本样式通常在 `draw_text = { text_style = { ... } }` 内部。                                                              |
| `Cannot find value X to override`                                     | 1. 尝试覆盖 (`<Base> { X: ... }`) 一个在基类 `Base` 中不存在的属性 `X`。<br>2. 属性 `X` 的路径或名称错误。                                                              | 确认基类中确实存在你要覆盖的属性，并且路径正确。例如，覆盖背景色可能是 `<Base> { draw_bg: { color: ... } }`。                                                                               |
| `Value type mismatch for field X`                                     | 为属性 `X` 赋了错误类型的值。                                                                                                                               | 检查赋值类型：数字 (`100`, `12.5`)，颜色 (`#f00`, `(THEME_COLOR_...)`)，字符串 (`"text"`), 枚举 (`Fill`, `Down`)，布尔 (`true`, `false`)，依赖 (`dep("...")`)，对象 (`{...}`)。 |
| `Cannot cast value X to type Y`                                       | 同上，类型不匹配。                                                                                                                                         | 同上。                                                                                                                                                                                                |
| `Expected type X found Y`                                             | 同上，类型不匹配。                                                                                                                                         | 同上。                                                                                                                                                                                                |
| `Unexpected token` / `Expected '}'` / `Expected ':'` / `Expected ','` | DSL 语法错误。                                                                                                                                             | 检查括号 `{}` `()` `<>` 是否匹配，属性间是否有逗号 `,`，属性名和值之间是否有冒号 `:`。利用编辑器语法高亮。                                                                                   |
| `Cannot parse value`                                                  | 赋给属性的值格式不正确（例如，颜色格式错误、数字格式错误、枚举名称错误）。                                                                                     | 检查颜色格式 (`#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`)，数字格式，枚举拼写。                                                                                                              |
| `Unexpected field X`                                                  | 1. 在组件内部使用了它不支持的字段名 `X`。<br>2. 结构层级错误，将属性放在了错误的对象内。                                                                         | 确认组件支持该字段。检查花括号 `{}` 的层级是否正确。                                                                                                                                             |
| `Cannot find widget class Y`                                          | 1. 组件类型 `<Y>` 名称拼写错误或大小写错误。<br>2. 忘记 `use link::widgets::*;` 或其他包含该组件的 `use` 语句。<br>3. 自定义组件未在 Rust 中正确注册 (`live_register`)。 | 检查组件名称拼写。确认已导入 `widgets`。如果是自定义组件，确保已在 `live_register` 中注册。                                                                                              |
| `Expected object value for field Z`                                   | 字段 `Z` 期望一个对象值（用 `{...}` 包裹），但赋了其他类型的值（如数字、字符串）。                                                                               | 通常发生在 `layout`, `walk`, `draw_bg`, `draw_text`, `animator` 等属性上，确保使用 `{}` 包裹其内部属性。                                                                                    |
| `Cannot find dependency` / `File not found at path X`                 | `dep("...")` 中的资源路径错误或文件不存在。                                                                                                                   | 检查 `crate://self/` 前缀是否正确。确认文件路径和文件名无误，且文件存在于项目中。                                                                                                         |
| (Shader 编译错误)                                                     | `fn pixel` 或 `fn vertex` 中的 MPSL 代码错误。                                                                                                             | 检查 Shader 语法、变量名、函数调用、类型。参考 GLSL 语法和 Makepad 内置函数 (`Sdf2d`, `Pal`, `Math`)。                                                                                       |
| `Name collision between X splat and Y`                                | `#[derive(Live)]` 宏检测到命名冲突。通常是一个 `#[live]` 字段与一个由属性宏（如 `#[walk]`）隐式处理的字段同名。                                                  | 移除或重命名冲突的 `#[live]` 字段。例如，如果使用了 `#[walk] walk: Walk`，就不要再定义 `#[live] abs_pos: DVec2`。                                                                          |
| `the trait bound X: LiveApply is not satisfied`                       | 通常是 `#[derive(Live)]` 失败的连锁反应，导致 `LiveApply` trait 未被实现，进而影响 `#[derive(Widget)]`。                                                          | 解决导致 `#[derive(Live)]` 失败的根本错误（通常是命名冲突或其他宏处理错误）。                                                                                                             |
| `no function or associated item named '...' found for struct X`       | 同上，`#[derive(Live)]` 失败导致 `LiveNew` trait 的方法（如 `new`, `live_design_with`, `live_type_info`）未实现。                                                 | 解决导致 `#[derive(Live)]` 失败的根本错误。                                                                                                                                             |

**调试技巧:**

*   **仔细阅读错误信息:** Makepad 的错误信息通常会指出发生错误的文件、行号和具体问题（目前需要修改 Live DSL 文件 reload 才能精确展示行号）。
*   **检查拼写和大小写:** LiveId 和属性名是区分大小写的。
*   **检查语法:** 确保逗号、冒号、括号都正确使用和匹配。
*   **参考文档和示例:** 对照官方文档或示例代码检查属性名称、类型和用法。
*   **简化代码:** 如果遇到难以定位的错误，尝试注释掉部分代码，逐步缩小问题范围。
*   **查看 Rust 定义:** 对于不确定的属性或结构，查看对应 Widget 的 Rust 源代码 (`#[derive(Live)]` 部分) 是最权威的方式。
*   **利用 Live Reload:** 保存 DSL 文件，观察 UI 是否按预期更新，可以快速验证小的修改。通常 这种方法会精确显示错误的 dsl 行号。

## 内置 Widget 

### **内置 Widgets 概览 (DSL 属性重点)**

*   **`<View>`**: 基础容器。
    *   `show_bg: bool`
    *   `draw_bg: <DrawColor>` (或覆盖为其他 Draw* 类型)
    *   `walk`, `layout`
    *   `scroll_bars: <ScrollBars>`
    *   `optimize: None/DrawList/Texture`
*   **`<Label>`**: 显示文本。
    *   `text: "string"`
    *   `draw_text: <DrawText2>`
    *   `walk`, `layout` (通常 `width:Fit, height:Fit`)
*   **`<Button>`**: 可点击按钮。
    *   `text: "string"`
    *   `draw_text: <DrawText2>`
    *   `draw_icon: <DrawIcon>`
    *   `draw_bg: <DrawQuad>` (或覆盖)
    *   `icon_walk`, `label_walk`, `walk`, `layout`
    *   `enabled: bool`
*   **`<LinkLabel>`**: 带下划线的按钮，用于链接。
    *   继承自 `<Button>`。
    *   `url: "string"` (Rust 中设置)
    *   `open_in_place: bool`
*   **`<TextInput>`**: 文本输入框。
    *   `text: "string"`
    *   `empty_text: "string"`
    *   `is_password`, `is_read_only`, `is_numeric_only: bool`
    *   `draw_text`, `draw_bg`, `draw_cursor`, `draw_selection`
    *   `walk`, `layout`
*   **`<CheckBox>` / `<Toggle>`**: 复选框/开关。
    *   `text: "string"`
    *   `draw_bg: <DrawCheckBox>` (可设置 `check_type: Check/Radio/Toggle/None`)
    *   `bind: "path.to.data"`
*   **`<RadioButton>`**: 单选按钮。
    *   `text: "string"`
    *   `value: EnumVariant`
    *   `draw_bg: <DrawRadioButton>` (可设置 `radio_type: Round/Tab`)
    *   通常在 `<View>` 或 `<ButtonGroup>` 内使用。
*   **`<Slider>`**: 滑块。
    *   `text: "string"`
    *   `min`, `max`, `default`, `step`, `precision`
    *   `bind: "path.to.data"`
    *   `text_input: <TextInput>` (内置文本输入)
    *   `draw_bg: <DrawSlider>`
*   **`<DropDown>`**: 下拉菜单。
    *   `labels: ["...", ...]`
    *   `values: [Enum::Val1, ...]`
    *   `selected_item: usize`
    *   `popup_menu: <PopupMenu>`
    *   `popup_menu_position: OnSelected/BelowInput`
    *   `bind: "path.to.data"`
*   **`<Image>`**: 显示位图。
    *   `source: dep("...")`
    *   `fit: Stretch/Horizontal/Vertical/Smallest/Biggest/Size`
*   **`<Icon>`**: 显示 SVG 图标。
    *   `draw_icon: { svg_file: dep("..."), color: ... }`
    *   `icon_walk`
*   **`<ScrollBars>`**: 滚动条容器（通常在 `<View>` 的 `scroll_bars` 属性中使用）。
    *   `show_scroll_x`, `show_scroll_y: bool`
    *   `scroll_bar_x`, `scroll_bar_y: <ScrollBar>`
*   **`<PortalList>` / `<PortalList2>`**: 高性能列表。
    *   需要在 Rust 中驱动 `next_visible_item` 和 `item`。
    *   `auto_tail: bool`
*   **`<Html>` / `<Markdown>`**: 渲染格式化文本。
    *   `body: "string"`
*   **`<PageFlip>` / `<SlidesView>`**: 页面/幻灯片切换器。
    *   `active_page: live_id!(page_id)` / `current_slide: value`
    *   子元素定义页面。
*   **`<Dock>`**: 可停靠面板系统。
    *   需要定义 `root`, `Tabs`, `Tab` 结构。
*   **`<Splitter>`**: 可拖动分隔器。
    *   `axis: Horizontal/Vertical`
    *   `align: FromA/FromB/Weighted`
    *   `a: <Widget>`, `b: <Widget>`
*   **`<Modal>`**: 模态对话框。
    *   `content: <Widget>`
*   **`<PopupNotification>` / `RobrixPopupNotification`**: 非模态弹出通知。
    *   `content: <Widget>`
*   **`<AdaptiveView>`**: 响应式视图切换。
    *   `Desktop = <Widget>`, `Mobile = <Widget>`
*   **`<CachedWidget>`**: 缓存子 Widget。
    *   `child_id = <Widget>`



### **Makepad Live DSL 速查手册 (高级用法补充)**

#### **`<View>` / `<ViewBase>` 及其变体**

*   **高级用法:**
    *   **自定义背景 Shader:** 直接在 `draw_bg` 中编写 `fn pixel` 或 `fn vertex` 来创建复杂的背景效果、渐变、图案或响应状态的视觉变化。
    *   **优化 (`optimize: Texture` 或 `DrawList`):** 用于包含大量静态或不经常变化内容的 `View`，可以显著提高性能。`Texture` 模式将内容绘制到纹理上，`DrawList` 模式缓存绘制命令。
    *   **嵌套与复杂布局:** 结合不同的 `flow`, `align`, `spacing`, `padding`, `margin` 以及 `Size` 模式 (`Fill`, `Fit`, `Fixed`) 来构建复杂的、响应式的布局结构。
    *   **事件处理与状态:** 虽然 `View` 本身不直接处理很多交互，但可以捕获 `FingerDown`, `FingerHover` 等事件，并结合 `animator` 改变 `draw_bg` 中的 `instance` 变量，实现视觉反馈。
    *   **滚动:** 使用 `<ScrollXView>`, `<ScrollYView>`, `<ScrollXYView>` 或在普通 `<View>` 中添加 `scroll_bars: <ScrollBars> {}` 来创建滚动区域。

*   **示例 (自定义 Shader & 优化):**
    ```rust
    MyAdvancedView = <View> {
        width: 300, height: 200,
        optimize: Texture, // 优化为纹理缓存
        show_bg: true,
        draw_bg: {
            texture image: texture2d // 需要在 Rust 中设置纹理
            varying scale: vec2
            varying shift: vec2

            uniform time: float // 动画 uniform

            fn vertex(self) -> vec4 { // 自定义顶点着色器 (来自 CachedView)
                let dpi = self.dpi_factor;
                let ceil_size = ceil(self.rect_size * dpi) / dpi;
                let floor_pos = floor(self.rect_pos * dpi) / dpi;
                self.scale = self.rect_size / ceil_size;
                self.shift = (self.rect_pos - floor_pos) / ceil_size;
                return self.clip_and_transform_vertex(self.rect_pos, self.rect_size)
            }

            fn pixel(self) -> vec4 { // 自定义像素着色器
                // 结合缓存纹理和时间创建动态效果
                let cached_color = sample2d_rt(self.image, self.pos * self.scale + self.shift);
                let dynamic_color = Pal::hsv2rgb(vec4(fract(self.pos.x + self.time), 0.8, 0.8, 1.0));
                return mix(cached_color, dynamic_color, 0.5);
            }
        }
        // ... 子组件 ...
    }
    ```


**`<View>`** 是唯一可以设置 `event_order` 属性的内置 Widget。

1.  **`<View>` 组件:**
    *   文档 (`DSL/Widgets/View.md`) 明确列出了 `event_order ([EventOrder](#eventorder))` 作为一个字段。
    *   `EventOrder` 枚举允许你指定事件在其子组件中的传播顺序：
        *   `Up` (默认): 从最后一个子组件到第一个（类似 HTML DOM 的冒泡）。
        *   `Down`: 从第一个子组件到最后一个。
        *   `List(Vec<LiveId>)`: 按照列表中指定的 `LiveId` 顺序。
    *   **用途:** 这对于控制哪个子组件优先接收和处理事件至关重要，特别是在有重叠区域或需要特定处理顺序的情况下（例如，确保顶层元素先处理事件）。

2.  **其他内置组件:**
    *   大多数其他内置 Widget（如 `<Button>`, `<Label>`, `<TextInput>`, `<Slider>` 等）**通常不直接暴露 `event_order` 属性** 让用户在 DSL 中设置。
    *   这些组件内部可能会使用 `<View>` 作为其基础结构的一部分，但它们自身的事件处理逻辑通常是固定的，或者依赖于父级 `<View>` 的 `event_order`。
    *   例如，一个 `<Button>` 内部可能有一个 `<View>` 来布局图标和文本，但你不能直接在 `<Button>` 上设置 `event_order` 来改变图标和文本接收事件的顺序。你需要修改 `<Button>` 的内部 `live_design!` 定义（如果它是自定义的）或者接受其默认行为。
    *   对于像 `<Dock>`, `<PortalList>`, `<Window>` 这样的复杂容器 Widget，它们有自己特定的事件分发逻辑来管理其子项（Tabs, List Items, Child Windows），通常不通过简单的 `event_order` 属性控制。

**总结:**

*   **`<View>` 是唯一可以直接在 `live_design!` 中配置 `event_order` 的核心内置 Widget。**
*   你可以通过将其他 Widget 嵌套在具有特定 `event_order` 的 `<View>` 中来间接控制它们的事件处理顺序。
*   对于 Makepad 提供的预设 Widget 变体（如 `<ButtonFlat>`, `<H1>` 等），它们通常继承自基础 Widget（如 `<Button>`, `<Label>`），其事件处理顺序也主要由其内部结构和父容器决定。

如果你需要对特定组件（非 `<View>`）的子元素事件顺序进行精细控制，你可能需要：

1.  将这些子元素包裹在一个 `<View>` 中，并设置该 `<View>` 的 `event_order`。
2.  创建自定义版本的 Widget，并在其内部 `live_design!` 定义中调整结构或使用带有特定 `event_order` 的 `<View>`。


#### **`<Label>`**

*   **高级用法:**
    *   **自定义文本 Shader:** 在 `draw_text` 中覆盖 `fn get_color` 或 `fn pixel` 来实现文本颜色渐变、特殊效果或基于状态的颜色变化。
    *   **富文本 (通过 TextFlow/Html/Markdown):** 对于需要混合样式（粗体、斜体、链接等）的文本，使用 `<Html>` 或 `<Markdown>` Widget，它们内部使用 `<TextFlow>`，而 `<TextFlow>` 又使用 `<Label>` (或其 DrawText) 进行底层绘制。
    *   **响应 Hover 事件:** 设置 `hover_actions_enabled: true`，然后在 Rust 代码中监听 `LabelAction::HoverIn/HoverOut`，通常用于显示 Tooltip。

*   **示例 (自定义颜色 Shader):**
    ```rust
    GradientLabel = <Label> {
        text: "Gradient Text"
        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 24.0 }
            fn get_color(self) -> vec4 {
                // 从左到右从红到蓝的渐变
                return mix(#f00, #00f, self.pos.x);
            }
        }
    }
    ```

#### **`<Button>`**

*   **高级用法:**
    *   **完全自定义外观:** 覆盖 `draw_bg`, `draw_text`, `draw_icon` 中的 `fn pixel` 或 `fn get_color`，可以完全改变按钮的外观和状态反馈。
    *   **复杂布局:** 在 `<Button>` 内部使用 `<View>` 和其他 Widget 来创建包含多个元素（如图标、文本、状态指示器）的复杂按钮布局（需要 Button 的 `flow` 不是 `Overlay`，或者自定义绘制逻辑）。
    *   **动画状态:** 结合 `animator` 创建复杂的按下、悬停、禁用等状态的视觉过渡效果。
    *   **数据绑定/动作:** 在 Rust 中响应 `ButtonAction::Clicked/Pressed/Released`，并根据 `action_data` 执行不同的逻辑。

*   **示例 (自定义背景和动画):** (参考文档中的 Advanced 示例)

**`<LinkLabel>`**

*   **高级用法:**
    *   **自定义样式:** 因为它继承自 `<Button>`，可以像 Button 一样覆盖 `draw_bg`, `draw_text` 来改变下划线样式、文本颜色和悬停/按下效果。
    *   **动态 URL:** 在 Rust 代码中根据应用状态动态设置 `url` 属性。
    *   **处理点击:** 在 Rust 中捕获 `clicked` 动作，除了打开 URL 外，还可以执行其他应用逻辑。

*   **示例 (自定义下划线和颜色):**
    ```rust
    MyLink = <LinkLabel> {
        text: "Custom Link"
        draw_text: {
            color: #08A // 默认颜色
            color_hover: #0CF // 悬停颜色
            color_down: #048 // 按下颜色
        }
        draw_bg: { // 控制下划线
            uniform color: #08A
            uniform color_hover: #0CF
            uniform color_down: #048
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let offset_y = 1.0;
                sdf.move_to(0., self.rect_size.y - offset_y);
                sdf.line_to(self.rect_size.x, self.rect_size.y - offset_y);
                // 使用更粗、颜色变化的下划线
                return sdf.stroke(mix(
                    mix(self.color, self.color_hover, self.hover),
                    self.color_down,
                    self.down
                ), 1.5); // 更粗的下划线
            }
        }
    }
    ```

#### **`<TextInput>`**

*   **高级用法:**
    *   **自定义外观:** 覆盖 `draw_bg`, `draw_text`, `draw_cursor`, `draw_selection` 来完全控制输入框的视觉样式，包括背景、边框、光标、选区高亮等。
    *   **输入验证/过滤:** 虽然 DSL 中有 `is_numeric_only`，但更复杂的验证（如邮箱格式、最大长度）需要在 Rust 的 `handle_actions` 中监听 `TextInputAction::Changed` 并进行处理。
    *   **与数据模型双向绑定:** 使用 `bind` 属性将输入框的值与 Rust 数据结构同步。
    *   **响应特殊按键:** 在 Rust 中监听 `TextInputAction::KeyDownUnhandled` 来处理未被 TextInput 内部处理的按键事件。
    *   **动态只读/密码模式:** 在 Rust 代码中根据条件调用 `set_is_read_only` 或 `set_is_password`。

*   **示例 (自定义背景和光标):** (参考文档中的 Advanced 示例)

#### **`<CheckBox>` / `<Toggle>`**

*   **高级用法:**
    *   **自定义标记:** 覆盖 `draw_bg` 中的 `fn pixel`，可以绘制任意形状或图标来代替默认的勾选标记、圆形或开关滑块。`check_type: None` 可以完全移除默认绘制，让你用 `draw_icon` 或其他子 Widget 来表示状态。
    *   **自定义图标状态:** 在 `<CheckBoxCustom>` 中，通过覆盖 `draw_icon` 的 `fn get_color` 或 `fn pixel`，可以根据 `active` (选中状态)、`hover`、`focus` 实例变量来改变图标的颜色或外观。
    *   **数据绑定:** 使用 `bind` 属性将复选框的布尔状态与数据模型同步。

*   **示例 (自定义 Toggle 外观):**
    ```rust
    MyToggle = <Toggle> {
        text: "My Toggle"
        draw_bg: {
            size: 10.0 // 控制滑块大小基准
            uniform mark_size: 0.8 // 控制滑块相对于背景的大小比例
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.size;
                let left = sz + 1.;
                let c = vec2(left + sz, self.rect_size.y * 0.5);
                // 背景轨道 (Pill shape)
                sdf.box(left, c.y - sz, sz * 3.0, sz * 2.0, sz); // 圆角半径等于高度一半
                sdf.fill(mix(#555, #0A0, self.active)); // 背景颜色根据状态变化

                // 滑块 (Circle)
                let isz = sz * self.mark_size;
                sdf.circle(left + sz + self.active * sz, c.y, isz); // 位置根据 active 状态变化
                sdf.fill(mix(mix(#AAA, #FFF, self.hover), #EEE, self.down)); // 滑块颜色

                return sdf.result
            }
        }
    }
    ```

#### **`<RadioButton>`**

*   **高级用法:**
    *   **自定义标记/外观:** 类似于 CheckBox，可以覆盖 `draw_bg` 的 `fn pixel` 来创建自定义的单选按钮外观（例如，不同的选中标记、背景形状）。`radio_type: Tab` 用于创建标签页样式的单选按钮。
    *   **分组与状态管理:** RadioButton 本身不处理互斥逻辑。你需要将它们放在同一个父 `<View>` 下，并在 Rust 代码中监听 `RadioButtonAction::Clicked`，然后手动取消选中同一组中的其他 RadioButton (使用 `RadioButtonSet` 或手动迭代)。
    *   **自定义图标/图片:** 使用 `<RadioButtonCustom>` 或 `<RadioButtonImage>` 变体，并配置 `draw_icon` 或 `image` 属性。可以结合 `animator` 根据 `active` 状态改变图标/图片的外观。
    *   **数据绑定:** 使用 `bind` 和 `value` 将选中的值同步到数据模型。

*   **示例 (Tab 样式与状态管理):**
    ```rust
    MyRadioGroup = <View> {
        flow: Right, spacing: 0
        radio1 = <RadioButtonTab> { text: "Tab 1", value: Option1 }
        radio2 = <RadioButtonTab> { text: "Tab 2", value: Option2 }
        radio3 = <RadioButtonTab> { text: "Tab 3", value: Option3 }
    }
    ```
    ```rust
    // In App::handle_actions
    let radio_set = self.ui.radio_button_set(ids!(
        radio1,
        radio2,
        radio3,
    ));
    if let Some(selected_index) = radio_set.selected(cx, actions) {
        log!("Selected index: {}", selected_index);
        // 在这里可以根据 selected_index 更新应用状态或绑定的数据
        // RadioButtonSet::selected 已经处理了取消选中其他按钮的逻辑
    }
    ```

#### **`<Slider>`**

*   **高级用法:**
    *   **自定义外观:** 覆盖 `draw_bg` 的 `fn pixel` 来完全改变滑块轨道和滑块手柄的外观，包括形状、颜色、渐变等。可以使用 `slide_pos` (0.0-1.0) 实例变量来确定绘制位置。
    *   **双向/极性滑块:** 设置 `draw_bg: { bipolar: 1.0 }` 可以让滑块的值条从中间向两边绘制，适用于表示 -1 到 1 或类似范围的值。
    *   **自定义文本输入:** 覆盖 `text_input` 的样式，或者在 Rust 中监听 `SliderAction::Slide` 并手动更新一个独立的 `<Label>` 或 `<TextInput>`。
    *   **步进与精度:** 结合 `step` 和 `precision` 来控制滑块的离散值和显示格式。

*   **示例 (自定义 Rotary 外观):** (参考文档中的 RotarySolid 示例)

#### **`<DropDown>`**

*   **高级用法:**
    *   **自定义外观:** 覆盖 `draw_bg` 和 `draw_text` 来改变按钮本身的样式。
    *   **自定义弹出菜单:** 通过覆盖 `popup_menu: <PopupMenu> { ... }` 来完全自定义弹出菜单的背景、边框以及菜单项 (`menu_item: <PopupMenuItem> { ... }`) 的外观（背景、文本、选中标记）。
    *   **动态选项:** 在 Rust 代码中动态修改 `labels` 和 `values` 列表，然后调用 `redraw(cx)` 来更新下拉选项。
    *   **数据绑定:** 使用 `bind` 将选中的 `value` 同步到数据模型。

*   **示例 (自定义弹出菜单项):**
    ```rust
    MyDropDown = <DropDown> {
        labels: ["A", "B", "C"],
        values: [OptionA, OptionB, OptionC],
        popup_menu: <PopupMenu> {
            // 自定义菜单背景
            draw_bg: { color: #333, border_color: #666, border_size: 1.0 }
            menu_item: <PopupMenuItem> {
                // 自定义菜单项
                padding: {left: 20, top: 8, bottom: 8, right: 10}
                draw_bg: {
                    color: #333
                    color_hover: #555 // 悬停背景
                    color_active: #888 // 选中背景
                }
                draw_text: {
                    color: #EEE // 默认文字颜色
                    color_hover: #FFF // 悬停文字颜色
                    color_active: #FFF // 选中文字颜色
                }
            }
        }
    }
    ```

#### **`<Image>` / `<Icon>` / `<ImageBlend>`**

*   **高级用法:**
    *   **自定义 Shader:** 覆盖 `draw_bg` (Image) 或 `draw_icon` (Icon) 的 `fn pixel` 来应用滤镜、颜色调整、混合效果或其他图像处理。
    *   **动画:** 使用 `animator` 改变 `opacity`, `image_scale`, `image_pan` (DrawImage/DrawWebView) 或 `draw_icon` 中的 `instance` 变量来实现淡入淡出、缩放、平移或颜色动画。`ImageBlend` 内置了 `blend` 动画器用于交叉淡入淡出。
    *   **动态加载:** 在 Rust 中使用 `load_image_dep_by_path`, `load_image_file_by_path`, `load_image_file_by_path_async` 或 `set_texture` 来动态更改显示的图像。

*   **示例 (ImageBlend 切换):** (参考文档中的 ImageBlend 示例和 App 代码)

#### **`<PortalList>` / `<PortalList2>`**

*   **高级用法:**
    *   **多种模板:** 在 `live_design!` 中定义多个不同的列表项模板 (如 `ListItemTypeA`, `ListItemTypeB`)，然后在 Rust 的 `next_visible_item` 循环中，根据数据决定为每个 `item_id` 调用 `list.item(cx, item_id, live_id!(ListItemTypeA))` 还是 `list.item(cx, item_id, live_id!(ListItemTypeB))`。
    *   **无限滚动:** 在 `handle_event` 中检测滚动条接近末尾（通过 `ScrollBarsAction` 或比较 `first_id` 和 `range_end`），然后异步加载更多数据并更新 `range_end`。
    *   **下拉刷新:** 结合 `max_pull_down` 和对 `first_scroll > 0.0` 的检测来实现下拉刷新交互。
    *   **保持项状态:** 如果列表项自身有复杂状态（如图标加载、动画），确保在 `item()` 返回已存在项时，这些状态被保留。如果使用了 `reuse_items: true`，则需要在获取到重用项时重置其状态。

#### **`<Dock>` / `<Splitter>` / `<Tab>` / `<TabBar>`**

*   **高级用法:**
    *   **动态添加/移除 Tab:** 在 Rust 代码中修改 `Dock` 的 `dock_items` 状态（添加/移除 `Tab` 和 `Tabs` 定义），然后调用 `redraw(cx)`。需要仔细管理 `LiveId`。
    *   **自定义 Tab 外观:** 覆盖 `Tab` 组件的 `draw_bg`, `draw_name`, `draw_icon` 样式。
    *   **自定义 TabBar 外观:** 覆盖 `TabBar` 的 `draw_bg`, `draw_fill`, `draw_drag` 样式。
    *   **自定义 Splitter 外观:** 覆盖 `Splitter` 的 `draw_bg` 样式。
    *   **保存/加载布局:** 序列化/反序列化 `Dock` 的 `dock_items` `HashMap` 来保存和恢复用户自定义的布局。需要处理 `LiveId` 冲突（如 PR 中所示）。

#### **`<Html>` / `<Markdown>` / `<TextFlow>`**

*   **高级用法:**
    *   **自定义样式:** 覆盖 `draw_normal`, `draw_italic`, `draw_bold` 等 `DrawText2` 属性，以及 `draw_block` (`DrawFlowBlock`) 的颜色和 `fn pixel` 来改变渲染样式。
    *   **自定义块布局:** 修改 `code_layout`, `quote_layout`, `list_item_layout` 等 `Layout` 属性。
    *   **自定义链接:** 覆盖 `link: <MyLink>` 来使用自定义的链接组件（需要继承自 `LinkLabel` 或 `Button`）。
    *   **交互式元素:** 在 `Html` 或 `Markdown` 中嵌入自定义 Widget（如 `<Button>`），并在 Rust 中处理它们的事件。这通常需要在 `draw_walk` 中手动处理自定义标签。

#### **`<Modal>` / `<PopupNotification>`**

*   **高级用法:**
    *   **自定义背景:** 覆盖 `Modal` 的 `bg_view` 或 `PopupNotification` 的 `draw_bg` 来改变背景外观（例如，不同的模糊效果、颜色或完全透明）。
    *   **自定义内容:** `content` 可以是任何复杂的 Widget 组合。
    *   **自定义动画:** 覆盖 `animator` 来改变弹出/消失的动画效果（例如，淡入淡出、缩放、不同的缓动函数）。
    *   **条件打开/关闭:** 在 Rust 中根据应用逻辑调用 `open(cx)` 和 `close(cx)`。
    *   **处理内部动作:** 在父 Widget 的 `handle_actions` 中监听 Modal/Popup 内容区域发出的动作。

#### **`<AdaptiveView>`**

*   **高级用法:**
    *   **自定义选择器:** 使用 `set_variant_selector` 提供复杂的逻辑来根据多种因素（不仅仅是屏幕尺寸，还可以是平台、设备特性、应用状态等）选择要显示的视图变体。
    *   **状态保持:** 设置 `retain_unused_variants: true` 来保留非活动视图的状态，避免在切换回来时重新初始化。需要注意内存使用。
    *   **嵌套 AdaptiveView:** 可以嵌套使用以实现更复杂的响应式布局。

#### **`<CachedWidget>`**

*   **高级用法:**
    *   **共享复杂状态:** 用于需要在 UI 不同部分显示但逻辑上是同一个实例的 Widget（例如，一个全局的音乐播放器控制条）。
    *   **性能优化:** 缓存昂贵的 Widget 初始化或绘制过程。
    *   **注意:** 由于是单例，所有实例共享状态。确保这是期望的行为。

**最佳实践通常是在 DSL 中尽可能多地进行声明式定义，并将复杂的逻辑和状态管理保留在 Rust 代码中**。


##  Makepad Shader Language (MPSL) 速查手册



重点关注在 `live_design!` 的 `draw_*` 块中 `fn pixel` 和 `fn vertex` 函数内可用的特性和内置函数。

### **核心概念**

*   **`fn pixel(self) -> vec4`**: 像素着色器函数。计算并返回当前像素的最终颜色 (RGBA)。`self` 包含了 `uniform`, `instance`, `varying` 变量以及内置变量如 `pos` (归一化坐标 0-1), `rect_pos`, `rect_size`。
*   **`fn vertex(self) -> vec4`**: 顶点着色器函数。计算并返回顶点的最终裁剪空间位置 (Clip Space)。通常需要设置 `varying` 变量传递给 `pixel` 函数。`self` 包含 `uniform`, `instance`, `varying` 以及内置变量如 `geom_pos` (几何体归一化坐标 0-1), `rect_pos`, `rect_size`, `camera_projection`, `camera_view`, `view_transform`, `draw_clip`, `view_clip`, `draw_zbias`, `draw_depth`。
*   **`self`**: 在 shader 函数内部，`self` 包含了所有可用的 `uniform`, `instance`, `varying` 变量以及内置变量。可以直接通过 `self.propertyName` 访问。
*   **变量类型**:
    *   `float`: 单精度浮点数。
    *   `vec2`, `vec3`, `vec4`: 2/3/4 维浮点向量。
    *   `mat2`, `mat3`, `mat4`: 2x2, 3x3, 4x4 浮点矩阵。
    *   `int`, `ivec2`, `ivec3`, `ivec4`: 整数及向量。
    *   `bool`, `bvec2`, `bvec3`, `bvec4`: 布尔值及向量。
    *   `texture2d`: 2D 纹理采样器。
    *   `textureOES`: (Android 特定) OES 纹理采样器，通常用于视频。
*   **内置变量 (常用):**
    *   **`self.pos` (vec2, pixel shader):** 当前像素在 `rect_size` 内的归一化坐标 (0.0 到 1.0)。
    *   **`self.geom_pos` (vec2, vertex shader):** 输入几何体（通常是四边形）的归一化顶点坐标 (0.0 到 1.0)。
    *   **`self.rect_pos` (vec2):** 当前绘制矩形的左上角屏幕坐标。
    *   **`self.rect_size` (vec2):** 当前绘制矩形的尺寸（宽、高）。
    *   **`self.draw_clip` (vec4):** 绘制裁剪区域 (xy=min, zw=max)。
    *   **`self.view_clip` (vec4):** 视图裁剪区域 (xy=min, zw=max)。
    *   **`self.view_shift` (vec2):** 视图滚动偏移。
    *   **`self.camera_projection` (mat4):** 摄像机投影矩阵。
    *   **`self.camera_view` (mat4):** 摄像机视图矩阵。
    *   **`self.view_transform` (mat4):** 视图变换矩阵。
    *   **`self.draw_depth` (float):** 基础绘制深度。
    *   **`self.draw_zbias` (float):** 深度偏移。
    *   **`self.dpi_factor` (float):** 当前 DPI 因子。
*   **变量声明:**
    *   `uniform name: type`: 在 DSL 中定义，所有实例共享（可覆盖）。
    *   `instance name: type`: 在 DSL 中定义，每个实例独立（常用于动画/状态）。
    *   `varying name: type`: 在 DSL 中定义，用于在 `vertex` 和 `pixel` 之间传递数据。
    *   `let name: type = value;`: 在函数内部声明局部变量。
    *   `var name: type = value;`: 在函数内部声明可变局部变量。
*   **颜色表示:**
    *   `#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`: 十六进制颜色。
    *   `vec4(r, g, b, a)`: 0.0 到 1.0 范围的 RGBA。
    *   `(THEME_COLOR_...)`: 引用主题颜色常量。

### **内置函数库 (`Sdf2d`, `Pal`, `Math`, `GaussShadow`)**

*   **`Sdf2d` (Signed Distance Field 2D)**: 用于矢量绘图。
    *   `Sdf2d::viewport(pos: vec2) -> Self`: 创建 SDF 上下文。
    *   `sdf.clear(color: vec4)`: 用颜色覆盖结果。
    *   **形状定义:** (更新 `sdf.dist` 和 `sdf.shape`)
        *   `sdf.circle(cx, cy, radius)`
        *   `sdf.rect(x, y, w, h)`
        *   `sdf.box(x, y, w, h, radius)`
        *   `sdf.box_x(x, y, w, h, r_left, r_right)`
        *   `sdf.box_y(x, y, w, h, r_top, r_bottom)`
        *   `sdf.box_all(x, y, w, h, r_lt, r_rt, r_rb, r_lb)`
        *   `sdf.hexagon(cx, cy, radius)`
        *   `sdf.hline(y, half_thickness)`
        *   `sdf.move_to(x, y)`
        *   `sdf.line_to(x, y)`
        *   `sdf.close_path()`
        *   `sdf.arc2(cx, cy, radius, start_angle_rad, end_angle_rad)`
        *   `sdf.arc_round_caps(cx, cy, radius, start_angle, end_angle, thickness)`
        *   `sdf.arc_flat_caps(...)`
    *   **布尔运算:** (通常在形状定义后调用)
        *   `sdf.union()`: `shape = min(dist, old_shape)`
        *   `sdf.intersect()`: `shape = max(dist, old_shape)`
        *   `sdf.subtract()`: `shape = max(-dist, old_shape)`
    *   **绘制:**
        *   `sdf.fill(color: vec4) -> vec4`: 填充并重置 shape。
        *   `sdf.fill_keep(color: vec4) -> vec4`: 填充并保留 shape。
        *   `sdf.fill_premul(premultiplied_color: vec4) -> vec4`: 预乘 Alpha 填充并重置。
        *   `sdf.fill_keep_premul(premultiplied_color: vec4) -> vec4`: 预乘 Alpha 填充并保留。
        *   `sdf.stroke(color: vec4, width: float) -> vec4`: 描边并重置 shape。
        *   `sdf.stroke_keep(color: vec4, width: float) -> vec4`: 描边并保留 shape。
        *   `sdf.glow(color: vec4, width: float) -> vec4`: 辉光并重置 shape。
        *   `sdf.glow_keep(color: vec4, width: float) -> vec4`: 辉光并保留 shape。
    *   **效果:**
        *   `sdf.gloop(k: float)`: 平滑混合 (Metaball)。
        *   `sdf.blend(k: float)`: 线性混合。
    *   **变换:**
        *   `sdf.translate(dx, dy)`
        *   `sdf.rotate(angle_rad, pivot_x, pivot_y)`
        *   `sdf.scale(factor, pivot_x, pivot_y)`
    *   **结果:** `sdf.result` (最终 `vec4` 颜色)。

*   **`Pal` (Palette)**: 颜色处理函数。
    *   `Pal::premul(color: vec4) -> vec4`: 转换为预乘 Alpha。
    *   `Pal::hsv2rgb(hsv: vec4) -> vec4`: HSV 转 RGB。
    *   `Pal::rgb2hsv(rgb: vec4) -> vec4`: RGB 转 HSV。
    *   `Pal::iqX(t: float) -> vec3`: Inigo Quilez 的调色板函数 (X=0..7)。

*   **`Math`**: 数学函数。
    *   `Math::rotate_2d(v: vec2, angle_rad: float) -> vec2`: 旋转 2D 向量。
    *   `Math::random_2d(v: vec2) -> float`: 基于输入向量生成伪随机数 (0-1)。

*   **`GaussShadow`**: 高斯模糊阴影计算。
    *   `GaussShadow::box_shadow(lower: vec2, upper: vec2, point: vec2, sigma: float) -> float`: 计算矩形阴影的模糊值。
    *   `GaussShadow::rounded_box_shadow(lower: vec2, upper: vec2, point: vec2, sigma: float, corner: float) -> float`: 计算圆角矩形阴影的模糊值。

### **GLSL 内置函数 (常用)**

(基于文档中 `builtin` 目录下的文件)

*   **数学:** `abs`, `ceil`, `clamp`, `degrees`, `distance`, `dot`, `exp`, `exp2`, `faceforward`, `floor`, `fract`, `inversesqrt`, `length`, `log`, `log2`, `max`, `min`, `mix`, `mod`, `normalize`, `pow`, `radians`, `reflect`, `refract`, `sign`, `smoothstep`, `sqrt`, `step`。
*   **三角函数:** `acos`, `asin`, `atan`, `cos`, `sin`, `tan`。
*   **向量/矩阵:** `cross`, `matrixCompMult`, `transpose`, `inverse`。
*   **逻辑:** `all`, `any`, `equal`, `greaterThan`, `greaterThanEqual`, `lessThan`, `lessThanEqual`, `not`, `notEqual`。
*   **纹理采样:**
    *   `sample2d(sampler: texture2d, coord: vec2) -> vec4`: 对 2D 纹理进行采样。
    *   `sample2d_rt(sampler: texture2d, coord: vec2) -> vec4`: (Makepad 特定) 可能用于渲染目标纹理的采样，行为可能与 `sample2d` 类似或有特定优化。
    *   `sample2dOES(sampler: textureOES, coord: vec2) -> vec4`: (Android 特定) 对 OES 纹理进行采样。
*   **导数:** `dFdx(p)`, `dFdy(p)`: 计算变量 `p` 相对于屏幕 x/y 坐标的偏导数（仅 Fragment Shader）。

### **Shader 编写最佳实践**

1.  **理解 `self`:** 知道 `self` 中有哪些可用的内置变量 (`pos`, `rect_pos`, `rect_size` 等) 和你在 DSL 中定义的 `uniform`/`instance`/`varying` 变量。
2.  **坐标系统:**
    *   `self.pos`: 通常是 0-1 的归一化坐标，相对于 `self.rect_size`。
    *   `self.geom_pos`: 0-1 的几何体坐标，通常用于顶点着色器。
    *   `Sdf2d::viewport(self.pos * self.rect_size)`: 将归一化坐标转换为相对于当前绘制矩形的像素级坐标，这是 SDF 绘图的常用起点。
3.  **善用 `Sdf2d`:** 对于矢量图形，优先使用 `Sdf2d` 提供的函数，它们通常比手动计算更方便、更优化。
4.  **状态管理:** 理解 `fill`/`stroke` 会重置 `sdf.shape`，而 `fill_keep`/`stroke_keep` 不会。布尔运算 (`union`, `intersect`, `subtract`) 会更新 `sdf.shape` 和 `sdf.old_shape`。
5.  **预乘 Alpha:** Makepad 的渲染管线通常使用预乘 Alpha。`Pal::premul` 用于转换，`fill_premul`/`fill_keep_premul` 直接处理预乘颜色。直接返回 `sdf.result` 通常已经是预乘的。
6.  **性能:**
    *   避免在 `pixel` 函数中进行过于复杂的计算或循环。
    *   纹理采样是相对昂贵的操作。
    *   利用 `varying` 在 `vertex` 中计算可以在像素间插值的值。
7.  **动画与状态:** 使用 `instance` 变量接收来自 `animator` 的值，并在 `pixel` 或 `vertex` 函数中使用 `mix()` 或其他逻辑来根据这些值改变外观。
8.  **调试:**
    *   直接返回固定颜色 (`return #f00;`) 来测试代码块是否执行。
    *   输出中间值到颜色通道 (`return vec4(value, 0.0, 0.0, 1.0);`)。
    *   使用 `View` 的 `debug` 属性。



## 附录



### 附录 A: 术语概念

#### “归一化相对坐标”（Normalized Relative Coordinates）

**核心思想：比例定位**

想象一个父容器，比如一个 `<View>`，它有一定的宽度和高度。当你在里面放置一个子元素时，如果父容器的空间比子元素大，子元素就需要知道放在这个“剩余空间”的哪个位置。

“归一化相对坐标”就是一种描述这个位置的方式，它不使用具体的像素值，而是使用**比例**。

*   **归一化 (Normalized):** 意味着坐标值的范围被限制在一个标准区间内，通常是 **0.0 到 1.0**。
    *   `0.0` 代表一端的开始（左边或顶部）。
    *   `1.0` 代表另一端的结束（右边或底部）。
    *   `0.5` 代表正中间。
*   **相对 (Relative):** 意味着这个坐标是**相对于父容器的可用空间**来计算的，而不是相对于整个窗口或屏幕的绝对像素位置。

**具体到 `align: {x: f64, y: f64}`:**

*   **`align.x` (水平方向):**
    *   `0.0`: 将子元素的**左边缘**与父容器可用空间的**左边缘**对齐。
    *   `0.5`: 将子元素的**水平中心**与父容器可用空间的**水平中心**对齐。
    *   `1.0`: 将子元素的**右边缘**与父容器可用空间的**右边缘**对齐。
    *   `0.25`: 将子元素放置在从左边算起 1/4 处的位置。
    *   `0.75`: 将子元素放置在从左边算起 3/4 处的位置。

*   **`align.y` (垂直方向):**
    *   `0.0`: 将子元素的**上边缘**与父容器可用空间的**上边缘**对齐。
    *   `0.5`: 将子元素的**垂直中心**与父容器可用空间的**垂直中心**对齐。
    *   `1.0`: 将子元素的**下边缘**与父容器可用空间的**下边缘**对齐。
    *   `0.25`: 将子元素放置在从顶部算起 1/4 处的位置。
    *   `0.75`: 将子元素放置在从顶部算起 3/4 处的位置。

**可用空间 (Available Space):**

这是理解相对坐标的关键。可用空间是指父容器在放置完所有**非对齐**子元素（例如，按 `flow: Right` 或 `flow: Down` 排列的元素）以及考虑了自身的 `padding` 之后，**剩余**的用于放置**对齐**子元素（通常是那些 `width: Fit`, `height: Fit` 或 `width: Fixed`, `height: Fixed` 的元素）的空间。

*   **对于 `flow: Right`:** 可用水平空间是父容器宽度减去所有子元素宽度、间距和父容器左右 padding 后的剩余宽度。可用垂直空间通常是父容器的高度减去其上下 padding。
*   **对于 `flow: Down`:** 可用垂直空间是父容器高度减去所有子元素高度、间距和父容器上下 padding 后的剩余高度。可用水平空间通常是父容器的宽度减去其左右 padding。
*   **对于 `flow: Overlay`:** 可用空间通常就是父容器减去 padding 后的整个内部区域。

**为什么使用归一化相对坐标？**

1.  **响应式布局:** 这种方式使得布局不依赖于具体的像素尺寸。无论父容器如何缩放或调整大小，子元素总能按照设定的比例保持其相对位置（例如，始终居中，始终靠右）。
2.  **简洁性:** 用 0.0 到 1.0 的范围来表达对齐比使用复杂的像素计算或约束更简单直观。
3.  **与 `Size::Fill` 的关系:** 当子元素使用 `Size::Fill` 时，它会占据所有可用空间，此时 `align` 在该方向上通常不起作用，因为没有剩余空间可以用来对齐。

**示例理解:**

```live_design!
<View> { // 父容器
    width: 200, height: 100,
    padding: 10, // 上下左右各 10 padding
    show_bg: true, draw_bg: { color: #eee }
    // 可用空间: 宽 180 (200-10-10), 高 80 (100-10-10)

    align: { x: 0.5, y: 0.5 } // 子元素水平垂直居中

    <Button> {
        text: "OK",
        width: Fit, height: Fit // 假设按钮尺寸为 60x30
    }
}
```

在这个例子中：

*   父容器内部可用宽度是 180，可用高度是 80。
*   按钮尺寸是 60x30。
*   水平剩余空间是 180 - 60 = 120。
*   垂直剩余空间是 80 - 30 = 50。
*   `align: {x: 0.5, y: 0.5}` 意味着按钮的中心点应该对齐到可用空间的中心点。
*   按钮的左上角 x 坐标 = 父容器左 padding + 水平剩余空间 * `align.x` = 10 + 120 * 0.5 = 70。
*   按钮的左上角 y 坐标 = 父容器上 padding + 垂直剩余空间 * `align.y` = 10 + 50 * 0.5 = 35。
*   所以按钮最终会被放置在父容器内 `(70, 35)` 的位置（相对于父容器的左上角）。

总之，归一化相对坐标提供了一种与分辨率无关、基于比例的方式来定义子元素在父容器可用空间内的对齐位置。


