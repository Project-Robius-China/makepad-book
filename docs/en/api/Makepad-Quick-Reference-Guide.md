# Makepad Quick Reference Guide

## Live DSL Quick Reference

### **Core Concepts**

- **`live_design!{ ... }`**: Macro for defining UI structure, styles, and components.
- **`use link::widgets::*;`**: Import Makepad's built-in Widget library.
- **`use link::theme::*;`**: Import constants defined in the current theme (colors, font sizes, spacing, etc.).
- **`use link::shaders::*;`**: Import standard Shader library (such as `Sdf2d`, `Pal`, `Math`).
- **`ComponentName = {{RustStructName}} { ... }`**: Bind DSL component to Rust struct. `ComponentName` is the name used in DSL, `RustStructName` is the corresponding Rust struct name.
- **`instance_name = <ComponentName> { ... }`**: Instantiate a component. `instance_name` is the `LiveId` for this instance, which can be referenced in Rust code via `id!(instance_name)`.
- **`<BaseComponent> { ... }`**: Inherit from `BaseComponent` and override or add properties.
- **`propertyName: value`**: Set property value.
- **`propertyName = { ... }`**: Set nested property (typically used for `draw_bg`, `draw_text`, `layout`, `walk`, `animator`, etc.).
- **`id!(...)`**: (In Rust code) Used to get a `LiveId`.
- **`live_id!(...)`**: (In DSL) Used to reference defined `LiveId` (not commonly used, typically just use the name directly).
- **`(CONSTANT_NAME)`**: Reference a constant defined in theme or elsewhere.
- **`dep("path/to/resource")`**: Declare a resource dependency (font, image, SVG). Path is typically relative to `Cargo.toml` or uses `crate://self/`. Pay attention to specifying the actual resource path when packaging.

### **Basic Layout and Dimensions (`Walk` & `Layout`)**

- **`walk`**: Controls the element's own layout within its parent container.
    - **`width`, `height`**: ([Size](https://claude.ai/chat/f104b585-ba73-4386-b683-0db4f4f9b15f#size)) Define dimensions.
        - `Fill`: Fill available space in parent container.
        - `Fit`: Adapt size based on content.
        - `Fixed(value)` or `value` (e.g., `100`): Fixed pixel value.
        - `All`: Occupy all space in parent container, ignoring padding.
    - **`margin`**: ([Margin](https://claude.ai/chat/f104b585-ba73-4386-b683-0db4f4f9b15f#margin)) External spacing.
        - `{top: v, right: v, bottom: v, left: v}`
        - `v` (shorthand, equal on all sides)
        - `<THEME_MSPACE_1>` (reference theme constant)
    - **`abs_pos: vec2(x, y)`**: Set absolute position (typically used with `Overlay` layout).
- **`layout`**: Controls how child elements are laid out within this element.
    - **`flow`**: ([Flow](https://claude.ai/chat/f104b585-ba73-4386-b683-0db4f4f9b15f#flow)) Layout direction for child elements.
        - `Right` (default): Horizontal from left to right.
        - `Down`: Vertical from top to bottom.
        - `Overlay`: Stacked on top of each other.
        - `RightWrap`: Horizontal arrangement, wrapping to new line when space is insufficient.
    - **`spacing: value`**: Gap between child elements (according to `flow` direction).
    - **`line_spacing: value`**: (`RightWrap`, `Down` flow) Spacing between lines.
    - **`padding`**: ([Padding](https://claude.ai/chat/f104b585-ba73-4386-b683-0db4f4f9b15f#padding)) Internal spacing.
        - `{top: v, right: v, bottom: v, left: v}`
        - `v` (shorthand, equal on all sides)
        - `<THEME_MSPACE_2>` (reference theme constant)
    - **`align`**: ([Align](https://claude.ai/chat/f104b585-ba73-4386-b683-0db4f4f9b15f#align)) Alignment of child elements within parent container.
        - `{x: 0.0, y: 0.0}` (top left)
        - `{x: 0.5, y: 0.5}` (center)
        - `{x: 1.0, y: 1.0}` (bottom right)
    - **`clip_x: bool`, `clip_y: bool`**: (default `true`) Whether to clip content that exceeds boundaries.
    - **`scroll: vec2(x, y)`**: Scroll offset for content.

**Alignment Coordinates in Live DSL (`align`)**

- **Scope:** Within `layout` property block, used to control how a **parent container** **aligns** its **child elements**.
    
- **Type:** `Align { x: f64, y: f64 }`
    
- **Coordinate System:** **Normalized Relative Coordinates**, range typically 0.0 to 1.0.
    
- **Meaning:**
    
    - `align.x`: Controls child element alignment in parent container's **available horizontal space**.
        - `0.0`: Left Align
        - `0.5`: Center Align
        - `1.0`: Right Align
    - `align.y`: Controls child element alignment in parent container's **available vertical space**.
        - `0.0`: Top Align
        - `0.5`: Center Align
        - `1.0`: Bottom Align
- **Key Points:**
    
    - `align` applies to **child elements as a whole** (or multiple child elements as a group, depending on `flow`). It determines child elements' position in the parent container's **remaining space**.
    - It does **not** change the child element's internal coordinate system or drawn content.
    - `align` has visible effect only when parent container's space is **larger** than child elements' dimensions. If child elements use `width: Fill` or `height: Fill`, there's typically no remaining space in that direction, so alignment has no meaning.
    - Values can exceed the 0.0-1.0 range to move elements partially or completely outside the parent container's visible area (requires parent container's `clip_x/clip_y: false`).
- **Example:**
    
    ```live_design!
    <View> { // parent container
        width: 200, height: 100,
        show_bg: true, draw_bg: { color: #eee }
        padding: 10, align: {x: 1.0, y: 1.0}
    
        <Button> { // child element
            text: "OK",
            width: Fit, height: Fit // size determined by content
        }
    }
    ```
    

### **Basic Drawing (`Draw*` Types)**

- **`DrawColor`**: Draw solid color background.
    - `color: #RRGGBBAA` or `(THEME_COLOR_...)`
    - `fn pixel(self) -> vec4 { ... }`: Overridable pixel shader.
- **`DrawQuad`**: Basic primitive for drawing quadrilaterals. Other `Draw*` types typically inherit from it. Includes basic vertex transformation and clipping logic.
    - `draw_depth: f32` (default `1.0`): Controls drawing depth/layer.
    - `draw_zbias: f32` (typically used internally): Fine-tune depth to avoid Z-fighting.
- **`DrawText` / `DrawText2`**: Draw text.
    - `text_style: <TextStyleName> { ... }`: Set text style.
    - `color: #RRGGBBAA` or `(THEME_COLOR_...)`
    - `wrap: Word / Line / Ellipsis` (Ellipsis may not be fully supported)
    - `font_scale: f64` (default `1.0`)
    - `brightness: f32` (legacy, may be deprecated)
    - `curve: f32` (legacy, may be deprecated)
    - `fn get_color(self) -> vec4 { ... }`: Overridable color logic (commonly used for animation).
- **`DrawIcon`**: Draw SVG icon.
    - `svg_file: dep("...")` or `svg_path: "..."`
    - `color: #RRGGBBAA` or `(THEME_COLOR_...)` (icon color)
    - `brightness: f32`
    - `curve: f32`
    - `fn get_color(self) -> vec4 { ... }`: Overridable color logic.
- **`DrawLine`**: Draw line (inherits from `DrawQuad`).
    - `color: #RRGGBBAA`
    - `line_width: f64` (set in Rust)
- **`DrawScrollShadow`**: Draw scroll shadow.
    - `shadow_size: f32`

### **SDF Drawing (`Sdf2d` in `fn pixel`)**

Use `Sdf2d` for vector drawing in the `fn pixel` function.

- **Initialization:** `let sdf = Sdf2d::viewport(self.pos * self.rect_size);`

**SDF Drawing (`Sdf2d` in `fn pixel`) - Basic Shapes**

Within the `fn pixel` function, you can use the `Sdf2d` object to define and combine shapes. Here are commonly used basic shape functions and their descriptions:

- **`sdf.circle(cx: float, cy: float, radius: float)`**
    
    - **Description:** Draws a circle.
    - **Parameters:**
        - `cx`, `cy`: x and y coordinates of the circle center.
        - `radius`: Circle radius.
    - **Effect:** Updates `sdf.dist` to the signed distance from the current pixel to the circle boundary. Inside is negative, outside is positive. `sdf.shape` is updated to `min(sdf.shape, sdf.dist)`.
- **`sdf.rect(x: float, y: float, w: float, h: float)`**
    
    - **Description:** Draws a rectangle with right angles.
    - **Parameters:**
        - `x`, `y`: x and y coordinates of the rectangle's bottom-left corner.
        - `w`, `h`: Width and height of the rectangle.
    - **Effect:** Updates `sdf.dist` to the signed distance from the current pixel to the rectangle boundary. `sdf.shape` is updated to `min(sdf.shape, sdf.dist)`.
- **`sdf.box(x: float, y: float, w: float, h: float, radius: float)`**
    
    - **Description:** Draws a rectangle with all corners having the same radius.
    - **Parameters:**
        - `x`, `y`: x and y coordinates of the rectangle's bottom-left corner.
        - `w`, `h`: Width and height of the rectangle.
        - `radius`: Radius for all four corners.
    - **Effect:** Updates `sdf.dist` to the signed distance from the current pixel to the rounded rectangle boundary. `sdf.shape` is updated to `min(sdf.shape, sdf.dist)`.
- **`sdf.box_x(x: float, y: float, w: float, h: float, r_left: float, r_right: float)`**
    
    - **Description:** Draws a rectangle with different corner radii on the left and right sides. Left-top and left-bottom corners use `r_left`, right-top and right-bottom corners use `r_right`.
    - **Parameters:**
        - `x`, `y`: x and y coordinates of the rectangle's bottom-left corner.
        - `w`, `h`: Width and height of the rectangle.
        - `r_left`: Radius for the two left corners.
        - `r_right`: Radius for the two right corners.
    - **Effect:** Updates `sdf.dist` to the signed distance from the current pixel to the specific rounded rectangle boundary. `sdf.shape` is updated to `min(sdf.shape, sdf.dist)`.
- **`sdf.box_y(x: float, y: float, w: float, h: float, r_top: float, r_bottom: float)`**
    
    - **Description:** Draws a rectangle with different corner radii on the top and bottom sides. Left-top and right-top corners use `r_top`, left-bottom and right-bottom corners use `r_bottom`.
    - **Parameters:**
        - `x`, `y`: x and y coordinates of the rectangle's bottom-left corner.
        - `w`, `h`: Width and height of the rectangle.
        - `r_top`: Radius for the two top corners.
        - `r_bottom`: Radius for the two bottom corners.
    - **Effect:** Updates `sdf.dist` to the signed distance from the current pixel to the specific rounded rectangle boundary. `sdf.shape` is updated to `min(sdf.shape, sdf.dist)`.
- **`sdf.box_all(x: float, y: float, w: float, h: float, r_left_top: float, r_right_top: float, r_right_bottom: float, r_left_bottom: float)`**
    
    - **Description:** Draws a rectangle allowing a separate radius for each corner.
    - **Parameters:**
        - `x`, `y`: x and y coordinates of the rectangle's bottom-left corner.
        - `w`, `h`: Width and height of the rectangle.
        - `r_left_top`: Radius for the left-top corner.
        - `r_right_top`: Radius for the right-top corner.
        - `r_right_bottom`: Radius for the right-bottom corner.
        - `r_left_bottom`: Radius for the left-bottom corner.
    - **Effect:** Updates `sdf.dist` to the signed distance from the current pixel to the highly customized rounded rectangle boundary. `sdf.shape` is updated to `min(sdf.shape, sdf.dist)`.
- **`sdf.hexagon(cx: float, cy: float, radius: float)`**
    
    - **Description:** Draws a regular hexagon.
    - **Parameters:**
        - `cx`, `cy`: x and y coordinates of the hexagon center.
        - `radius`: Distance from center to any vertex.
    - **Effect:** Updates `sdf.dist` to the signed distance from the current pixel to the hexagon boundary. `sdf.shape` is updated to `min(sdf.shape, sdf.dist)`.
- **`sdf.hline(y: float, half_thickness: float)`**
    
    - **Description:** Draws a horizontal line. Conceptually, this line is infinitely long but will typically be clipped by the viewport or parent container.
    - **Parameters:**
        - `y`: The y coordinate of the horizontal line's center.
        - `half_thickness`: Half the thickness of the line (total thickness is `2 * half_thickness`).
    - **Effect:** Updates `sdf.dist` to the signed distance from the current pixel to the horizontal line boundary. `sdf.shape` is updated to `min(sdf.shape, sdf.dist)`.
- **`sdf.move_to(x: float, y: float)`**
    
    - **Description:** Starts a new path or subpath, moving the current drawing point to the specified `(x, y)` coordinates **without drawing any line**. It updates `sdf.start_pos` and `sdf.last_pos`.
    - **Parameters:**
        - `x`, `y`: Coordinates of the new path starting point.
    - **Effect:** Sets the path starting point, for subsequent `line_to` or `close_path` operations.
- **`sdf.line_to(x: float, y: float)`**
    
    - **Description:** Draws a straight line segment from the current point (`sdf.last_pos`) to the specified `(x, y)` coordinates. This line segment becomes part of the current shape. It updates `sdf.last_pos`.
    - **Parameters:**
        - `x`, `y`: Coordinates of the line segment endpoint.
    - **Effect:** Adds a straight line segment to the current path, and updates `sdf.dist` and `sdf.shape` to reflect the distance to this line segment. It also updates `sdf.clip` for path filling.
- **`sdf.close_path()`**
    
    - **Description:** Closes the current subpath by drawing a straight line from the current point (`sdf.last_pos`) back to the starting point of the current subpath (`sdf.start_pos`).
    - **Effect:** Completes a closed shape definition.
- **`sdf.arc2(cx: float, cy: float, radius: float, start_angle: float, end_angle: float)`**
    
    - **Description:** Draws a circular arc.
    - **Parameters:**
        - `cx`, `cy`: x and y coordinates of the arc center.
        - `radius`: Arc radius.
        - `start_angle`: Starting angle of the arc (in radians).
        - `end_angle`: Ending angle of the arc (in radians).
    - **Effect:** Updates `sdf.dist` to the signed distance from the current pixel to the arc boundary. `sdf.shape` is updated to `min(sdf.shape, sdf.dist)`.

**Important Notes:**

- These functions only **define** the shape boundaries (by calculating a Signed Distance Field).
    
- To actually see these shapes, you must call a **drawing function** after defining the shape, such as `sdf.fill(color)`, `sdf.stroke(color, width)`, `sdf.glow(color, width)`, etc.
    
- `sdf.shape` stores the minimum SDF value of the currently defined shape (for `union`), while `sdf.old_shape` is used for boolean operations (`intersect`, `subtract`, `gloop`).
    
- `sdf.dist` stores the SDF value of the **most recently** defined basic shape.
    
- **Boolean Operations:** (typically called after drawing shapes)
    
    - `sdf.union()`: Combine current shape and `old_shape`.
    - `sdf.intersect()`: Take intersection of current shape and `old_shape`.
    - `sdf.subtract()`: Subtract current shape from `old_shape`.
- **Fill/Stroke:**
    
    - `sdf.fill(color)`: Fill current shape and reset shape state.
    - `sdf.fill_keep(color)`: Fill current shape, preserve shape state.
    - `sdf.fill_premul(premultiplied_color)`: Fill with premultiplied Alpha and reset.
    - `sdf.fill_keep_premul(premultiplied_color)`: Fill with premultiplied Alpha and preserve state.
    - `sdf.stroke(color, width)`: Stroke current shape and reset shape state.
    - `sdf.stroke_keep(color, width)`: Stroke current shape, preserve shape state.
- **Effects:**
    
    - `sdf.glow(color, width)`: Add glow and reset shape state.
    - `sdf.glow_keep(color, width)`: Add glow, preserve shape state.
    - `sdf.gloop(k)`: Smooth blend with `old_shape` (Metaball effect).
- **Transforms:** (affect subsequent drawing)
    
    - `sdf.translate(dx, dy)`
    - `sdf.rotate(angle_rad, pivot_x, pivot_y)`
    - `sdf.scale(factor, pivot_x, pivot_y)`
- **Clear:** `sdf.clear(color)`: Cover current result with specified color.
    
- **Result:** `sdf.result` (final `vec4` color).
    

**Shader Coordinates (`self.pos`, `self.geom_pos`, `Sdf2d::viewport` parameters)**

- **Scope:** Within `fn pixel` and `fn vertex` functions in `draw_*` blocks.
    
- **Type:** `vec2` (typically `f32` type)
    
- **Coordinate System:** **Normalized Local Coordinates**, range typically 0.0 to 1.0.
    
- **Meaning:**
    
    - **`self.pos` (in `fn pixel`):** Represents the current pixel being calculated, in **normalized coordinates** within its drawing rectangle (`self.rect_size`).
        - Top-left corner typically corresponds to `vec2(0.0, 0.0)`.
        - Bottom-right corner typically corresponds to `vec2(1.0, 1.0)`.
        - This coordinate is interpolated and used for texture sampling or calculating color/shape based on position.
    - **`self.geom_pos` (in `fn vertex`):** Represents the current vertex being processed, in **normalized coordinates** within its input geometry (typically a unit quad).
        - For default `GeometryQuad2D`, vertices are typically `(0,0)`, `(1,0)`, `(1,1)`, `(0,1)`.
        - This coordinate is used to map geometry vertices to the screen rectangle area defined by `rect_pos` and `rect_size`.
    - **`Sdf2d::viewport(pos: vec2)` parameter:** This `pos` parameter is typically **pixel coordinates** or **coordinates relative to some local origin**, not normalized.
        - Common usage is `Sdf2d::viewport(self.pos * self.rect_size)`. Here, `self.pos` is 0-1 normalized coordinate, multiplied by `self.rect_size` to convert it to **pixel-level local coordinates** relative to the top-left corner of the current drawing rectangle. SDF functions (such as `sdf.circle(cx, cy, r)`) expect this kind of pixel-level local coordinate.
- **Key Points:**
    
    - Shader coordinates are **local**, regardless of the Widget's final position on screen (final position determined by `vertex` function return value and transformation matrices).
    - `self.pos` is essential in `fn pixel` as it allows calculating color, pattern, or shape based on the pixel's relative position within the Widget rectangle.
    - `self.geom_pos` is used in `fn vertex` to map the basic geometry (typically a 0-1 quad) to the target rectangle on screen.
    - Coordinates passed to `Sdf2d` functions are typically relative to the coordinate system based on when `Sdf2d::viewport` was created (typically `self.pos * self.rect_size` producing local pixel coordinates).
- **Example:**
    
    ```live_design!
    <View> {
        show_bg: true,
        draw_bg: {
            fn pixel(self) -> vec4 {
                // self.pos.x varies from 0.0 to 1.0 from left to right
                // self.pos.y varies from 0.0 to 1.0 from top to bottom
                // Create a horizontal gradient from left(red) to right(green)
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
                // Draw a circle with radius 5 at local pixel coordinates (10, 10)
                sdf.circle(10.0, 10.0, 5.0);
                sdf.fill(#fff);
                return sdf.result;
            }
        }
    }
    ```
    

**Summary:**

|Concept|Scope|Type|Coordinate System|Meaning|
|:--|:--|:--|:--|:--|
|**`align`**|`layout` block|`Align`|Normalized Relative (0-1)|Controls **child elements** alignment in parent's **remaining space**|
|**`self.pos`**|`fn pixel`|`vec2`|Normalized Local (0-1)|Current pixel's relative position within its drawing rectangle|
|**`self.geom_pos`**|`fn vertex`|`vec2`|Normalized Geometry (0-1)|Current vertex's relative position in input geometry|
|**`Sdf2d` parameters**|`fn pixel`|`float`/`vec2`|Local Pixel/Arbitrary (depends on viewport)|Coordinates for defining SDF shapes|

Understanding the distinction between these coordinate systems is crucial for precise control of Makepad UI layout and visual effects. `align` is used for macro layout, while `self.pos` in Shaders is used for micro, pixel-level drawing.

### **Animation (`animator`)**

The `animator` block defines Widget states and transitions between these states. It allows you to create interactive visual feedback in a declarative way.

#### **Basic Structure**

```live_design!
MyWidget = {{MyWidget}} {
    // ... other properties ...
    animator: {
        // Define one or more state animation tracks
        track_name1 = { ... }
        track_name2 = { ... }
        // ...
    }
}
```

- `animator`: Block containing all animation definitions.
- `track_name`: Name of an animation track (LiveId), typically corresponding to a logical state (like `hover`, `focus`, `open`, `selected`, `visible`, etc.). This name will be referenced in Rust code via `id!(track_name)`.

#### **State Definition (`state_name = { ... }`)**

Inside each `track_name`, typically at least two states are defined, most commonly `on` and `off`, or custom state names.

```rust
animator: {
    hover = { // track name
        default: off, // initial state

        off = { // 'off' state definition
            // ... animation parameters ...
            apply: { /* ... property target values ... */ }
        }

        on = { // 'on' state definition
            // ... animation parameters ...
            apply: { /* ... property target values ... */ }
        }

        // Can define more custom state names
        // custom_state = { ... }
    }
}
```

- **`default: on | off`**: Specifies the default state of this animation track when Widget initializes.
- **`state_name = { ... }`**: Defines a specific state (such as `on`, `off`, `down`, `open`, `close`, etc.).

#### **State Transition Parameters**

Within each specific state definition block (like `on = { ... }`), the following parameters can be set to control how the animation transitions into this state:

- **`from: { ... }`**: Defines how to animate from **other states** to **current state**.
    - **`all: PlayMode`**: Sets default transition animation for all source states not explicitly specified.
    - **`other_state_name: PlayMode`**: Sets animation for transition from specific `other_state_name` state to current state. This overrides the `all` setting.
    - **`PlayMode`**: Defines animation playback mode and duration:
        - `Forward { duration: seconds }`: Animate forward from current value to target value, lasting `seconds` seconds.
        - `Backward { duration: seconds }`: Animate backward from target value to current value (less commonly used directly, typically handled automatically by `Forward`).
        - `Snap`: Immediately jump to target value, no animation.
        - `Loop { duration: seconds, end: value }`: Loop animation, `duration` is time for one cycle, `end` is loop point (typically `1.0`).
        - `ReverseLoop { ... }`: Reverse loop.
        - `BounceLoop { ... }`: Back-and-forth loop (ping-pong effect).
- **`ease: EaseType { ... }`**: Specifies easing function, controlling animation speed curve.
    - **Common:** `Linear`, `InQuad`, `OutQuad`, `InOutQuad`, `InExp`, `OutExp`, `InOutExp`, `ExpDecay {d1:v, d2:v}` (simulates physical decay).
    - **Others:** `In/Out/InOut` + `Cubic`, `Quart`, `Quint`, `Sine`, `Circ`, `Elastic`, `Back`, `Bounce`.
    - **Custom:** `Pow { begin: v, end: v }`, `Bezier { cp0: v, cp1: v, cp2: v, cp3: v }`.
- **`apply: { ... }`**: Defines which `#[live]` properties should have what target values when animation **reaches** this state.
    - **`property: value`**: Sets property `property`'s final value to `value`.
    - **`property: [{ time: t, value: v }, ...]`**: Defines keyframe animation.
        - `time`: Normalized time (0.0 to 1.0).
        - `value`: Value the property should have at this time point.
        - First keyframe `time` is typically `0.0`, representing value at animation start (typically the `from` state's value, but can be overridden).
        - Last keyframe `time` is typically `1.0`, representing value at animation end.
    - **Property paths:** Can be nested, like `draw_bg: { hover: 1.0 }` or `walk: { margin: { left: 10.0 } }`.
- **`redraw: true`**: Marks this animation track as needing continuous redrawing while playing. This is typically necessary for animations that change visually.
- **`cursor: MouseCursor`**: (typically in `on` state) Sets mouse cursor style (e.g., `Hand`, `Default`) when animation enters this state.

#### **How It Works**

1. **State Triggering (Rust):** In Rust code's `handle_event`, based on user interaction (like `Hit::FingerHoverIn`, `Hit::FingerDown`) or other logic, call `self.animator_play(cx, id!(track_name.state_name))` or `self.animator_toggle(cx, condition, animate, id!(track.on), id!(track.off))` to trigger state transitions.
2. **Finding Animation Parameters:** Makepad's animation system finds the target state (e.g., `on`) definition and looks up the corresponding `from` rule based on current state to determine animation `duration` and `ease`.
3. **Interpolation:** Animation system calculates intermediate values for each property specified in the `apply` block based on time, `duration`, and `ease` function.
4. **Applying Values:** Calculated intermediate values are applied back to corresponding `#[live]` fields in the Rust struct.
5. **Redrawing:** If animation track is marked with `redraw: true` or `animator_handle_event` returns `must_redraw()`, Widget redraw is requested.
6. **Drawing:** In `draw_walk`, Widget uses `#[live]` field values updated by animation for drawing.

**Example: Button Hover and Down States**

```live_design!
Button = {{Button}} {
    // ... other properties ...
    animator: {
        hover = {
            default: off,
            off = {
                // From any state back to off state, lasting 0.1 seconds
                from: {all: Forward {duration: 0.1}}
                apply: {
                    draw_bg: {hover: 0.0} // Background hover factor 0
                    draw_text: {hover: 0.0} // Text hover factor 0
                }
            }
            on = {
                cursor: Hand, // Show hand cursor when entering on state
                // From off state to on state, lasting 0.1 seconds
                // From down state to on state, immediate (Snap)
                from: {all: Forward {duration: 0.1}, down: Snap}
                apply: {
                    // Use keyframes, immediately set hover factor to 1.0
                    draw_bg: {hover: [{time: 0.0, value: 1.0}]}
                    draw_text: {hover: [{time: 0.0, value: 1.0}]}
                }
            }
            down = { // Pressed state
                // From any state to down state, lasting 0.2 seconds
                from: {all: Forward {duration: 0.2}}
                apply: {
                    // Use keyframes, immediately set down factor to 1.0, keep hover at 1.0
                    draw_bg: {down: [{time: 0.0, value: 1.0}], hover: 1.0}
                    draw_text: {down: [{time: 0.0, value: 1.0}], hover: 1.0}
                }
            }
        }
        // Can have other animation tracks, like focus, enabled, etc.
    }
}
```

In Makepad, for implementing **smooth visual animations** (e.g., gradual changes in UI element position, size, color, opacity, etc.), **it's strongly recommended to use the `NextFrame` event mechanism rather than `Timer`**.

The `Animator` system internally uses `NextFrame` to drive animation updates and redraws.

**`NextFrame` (for animation)**

- **Purpose:** Primarily used to drive **render-related, frame-by-frame** animations.
- **Triggering:**
    - When a Widget calls `self.redraw(cx)` or `self.area.redraw(cx)`, Makepad sends an `Event::NextFrame` event to that Widget (and other Widgets needing redraw) at the **beginning of the next** render cycle.
    - Widget can capture this `NextFrame` event in `handle_event` (`if let Some(ne) = self.next_frame.is_event(event)`), calculate the next animation state based on time difference (`ne.time`), update `#[live]` properties or `instance` variables, then **call `self.redraw(cx)` again** to request the next frame update.
    - The `Animator` system automates this process: `animator_handle_event` internally checks if animation is in progress, if so, calculates next frame values, applies them, and returns `must_redraw()` as `true`, which typically triggers `self.redraw(cx)` being called externally.
- **Advantages:**
    - **Synchronized with rendering:** `NextFrame` events are tightly coupled with Makepad's render loop, ensuring animations are synchronized with screen refresh for smoother visual effects.
    - **Efficient:** Only triggered when redraw is needed, avoiding unnecessary calculations and wake-ups. When animation ends or Widget is invisible, no more `NextFrame` is sent (unless there's another reason for redraw).
    - **Simple:** For UI animations, typically just need to start animation on state change (`animator_play`), update state in `NextFrame` event (or let `Animator` handle it), then request redraw.
- **Disadvantages:**
    - Not suitable for logic requiring precise time intervals (as it depends on render frame rate).
    - If render frame rate drops, animation speed will slow down too.

**`Timer` (for timed logic)**

- **Purpose:** Used for executing operations based on **fixed time intervals**, **independent of render frame rate**.
- **Triggering:**
    - Start a timer via `cx.start_timer(id, interval, repeats)` or `cx.start_timeout(id, interval)`.
    - When the set time interval is reached, Makepad sends an `Event::Timer` event with the triggered timer ID.
    - Widget captures this `Timer` event in `handle_event` (`if self.my_timer.is_event(event).is_some()`) and executes appropriate logic.
- **Advantages:**
    - **Time-precise:** Suitable for tasks needing to execute at fixed time intervals, like polling, timed checks, throttling/debouncing.
    - **Decoupled from rendering:** Not affected by render frame rate fluctuations (though event handling itself is still on the main thread).
- **Disadvantages:**
    - **Not suitable for smooth animation:** Timer triggering intervals may not synchronize with screen refresh rate, causing animation to appear jerky or inconsistent.
    - **May wake up too frequently:** If interval is set very short, will frequently wake up the event loop even if the interface isn't changing.

**Core Idea: Distinguish "Animation Process" from "Delayed Trigger"**

1. **Animation Process:** Refers to the **entire process** of a property value smoothly transitioning from A to B. For example:
    
    - A popup sliding **from** off-screen (`abs_pos: vec2(-1000.0, 10.0)`) **to** on-screen (`abs_pos: vec2(60.0, 10.0)`).
    - A progress bar's height growing **from** 0 **to** 100%.
    - A button's background color fading **from** gray **to** blue.
    
    For this kind of **smooth, frame-by-frame visual transition**, you should **rely on `Animator` and `NextFrame`**. `Animator` will calculate property intermediate values at each frame (triggered by `NextFrame`), update it, then request next frame redraw, until animation completes. **You should not use `Timer` to manually control this kind of frame-by-frame update**, as `Timer` triggering intervals are unrelated to screen refresh rate, causing jerky animation.
    
2. **Delayed Trigger:** Refers to executing a **one-time** operation **after a period of time**. For example:
    
    - **2.5 seconds after** popup is shown, automatically **start** closing animation (or close immediately).
    - **500 milliseconds after** user stops typing, auto-save draft.
    - **Delay 1 second** after button click before sending network request.
    
    For this kind of **triggering an action at a specific time point**, you should **use `Timer`**. `cx.start_timeout(duration)` is designed for this. When timer triggers (`my_timer.is_event(event).is_some()`), you perform the corresponding action, like calling `self.close(cx)` to **start** closing animation.
    

**For example: the `RobrixPopupNotification` component popup animation in robrix project:**

- **Open animation (slide in):** When calling `self.open(cx)`, you use `self.animator.animator_play(cx, id!(mode.open))` to **start** the slide-in animation. This animation **process** is driven by `Animator` and `NextFrame`. **No `Timer` needed here.**
- **Progress bar animation:** When calling `self.open(cx)`, you use `self.view(id!(content.progress)).animator_play(cx, id!(mode.progress))` to **start** progress bar animation. This animation **process** is also driven by `Animator` and `NextFrame`. **No `Timer` needed here.**
- **Auto-close:** You want the popup to auto-close after showing for a period. This "after a period" operation is a **delayed trigger**. Therefore, starting a `Timer` (`self.animation_timer = cx.start_timeout(2.5);`) in `self.open(cx)` is **correct**. When this `Timer` triggers after 2.5 seconds (`if self.animation_timer.is_event(event).is_some()`), you call `self.close(cx)`.
- **Close animation (slide out):** When calling `self.close(cx)` (whether triggered by timer or close button click), you use `self.animator.animator_play(cx, id!(mode.close))` to **start** the slide-out animation. This animation **process** is again driven by `Animator` and `NextFrame`. **No `Timer` needed here.**

Therefore:

- **Use `Animator` + `NextFrame` for smooth visual transition animations (sliding, fading, scaling, etc.).**
- **Use `Timer` (especially `cx.start_timeout`) to implement triggering a one-time action after a fixed delay (like starting a closing animation).**

**Best Practices:**

- **Use meaningful track and state names:** Makes code more readable (e.g., `hover`, `focus`, `open`, `selected`).
- **Keep animations concise:** Too many or too long animations can make users uncomfortable.
- **Leverage `default`:** Set appropriate initial state.
- **Consider `from` rules:** Define animation behavior for different state transitions to make transitions more natural.
- **Use `redraw: true`:** Ensure visual changes are rendered.
- **Put animation logic in DSL:** Use declarative approach for defining animations whenever possible, rather than manually calculating interpolations in Rust.
- **Animate `instance` variables:** Define `instance` variables in `draw_*` shaders (e.g., `instance hover: float`), modify these variables in `animator`'s `apply` block, then use `mix()` or other logic in `fn pixel` or `fn get_color` to change appearance based on these instance variables.
- **For all visual animations (position, size, color, opacity, etc. gradual changes), prioritize the `NextFrame` mechanism.** The simplest way is to use Makepad's built-in `Animator` system, which encapsulates `NextFrame`-based animation logic.
- **Only use `Timer` when you need precise time intervals for non-rendering related logic.** For example:
    - `PopupNotification` using `Timer` to auto-close after a fixed time is an appropriate use case.
    - Network request polling or timeout.
    - Periodic user data saving.
    - Implementing throttling or debouncing logic.

### **Makepad Live DSL Error Message Quick Reference**

|Error Message Pattern|Possible Causes|Common Scenarios & Tips|
|:--|:--|:--|
|`Enum variant X not found for field Y`|1. Enum variant name misspelled or case incorrect.<br>2. Using a variant not supported by that enum type.|Check spelling of `Flow` (Right, Down, Overlay, RightWrap), `Size` (Fill, Fit, Fixed, All), `Axis` (Horizontal, Vertical), `CheckType`, `RadioType`, `ImageFit`, etc. Consult documentation or Rust definition.|
|`Identifier X not found`|1. Referenced LiveId (component instance name, constant name) misspelled or case incorrect.<br>2. Referenced LiveId not defined in current or parent scope.<br>3. Forgot `use link::...` import.|Check instance names, constant names (e.g., `THEME_COLOR_TEXT`) are correct. Ensure defined before use. Check `use` statements.|
|`field X not found on struct Y`|1. Field name `X` misspelled or case incorrect.<br>2. Component `Y` (or its base) doesn't have `#[live]` field named `X`.<br>3. Trying to set property at wrong level (e.g., setting `text_style` directly on `<Button>` instead of inside `draw_text`).|Check property name spelling. Consult component's Rust definition or documentation for available properties and their hierarchy. E.g., text styles typically in `draw_text = { text_style = { ... } }`.|
|`Cannot find value X to override`|1. Trying to override (`<Base> { X: ... }`) a property `X` that doesn't exist in base class `Base`.<br>2. Property `X` path or name is incorrect.|Confirm base class actually has the property you're trying to override, and path is correct. E.g., overriding background color might be `<Base> { draw_bg: { color: ... } }`.|
|`Value type mismatch for field X`|Assigning wrong type of value to property `X`.|Check assignment types: numbers (`100`, `12.5`), colors (`#f00`, `(THEME_COLOR_...)`), strings (`"text"`), enums (`Fill`, `Down`), booleans (`true`, `false`), dependencies (`dep("...")`), objects (`{...}`).|
|`Cannot cast value X to type Y`|Same as above, type mismatch.|Same as above.|
|`Expected type X found Y`|Same as above, type mismatch.|Same as above.|
|`Unexpected token` / `Expected '}'` / `Expected ':'` / `Expected ','`|DSL syntax error.|Check brackets `{}` `()` `<>` matching, property separators `,`, property name-value separators `:`. Use editor syntax highlighting.|
|`Cannot parse value`|Value assigned to property has incorrect format (e.g., malformed color, malformed number, incorrect enum name).|Check color format (`#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`), number format, enum spelling.|
|`Unexpected field X`|1. Using field name `X` not supported by component.<br>2. Wrong structure level, putting property in wrong object.|Confirm component supports that field. Check curly brace `{}` hierarchy is correct.|
|`Cannot find widget class Y`|1. Component type `<Y>` name misspelled or case incorrect.<br>2. Forgot `use link::widgets::*;` or other component `use` statement.<br>3. Custom component not properly registered in Rust (`live_register`).|Check component name spelling. Confirm `widgets` imported. For custom components, ensure registered in `live_register`.|
|`Expected object value for field Z`|Field `Z` expects an object value (wrapped in `{...}`), but assigned other type (like number, string).|Typically happens with `layout`, `walk`, `draw_bg`, `draw_text`, `animator` properties, ensure using `{}` to wrap their inner properties.|
|`Cannot find dependency` / `File not found at path X`|Resource path in `dep("...")` incorrect or file missing.|Check `crate://self/` prefix is correct. Confirm file path and name are accurate, and file exists in project.|
|(Shader compilation errors)|MPSL code errors in `fn pixel` or `fn vertex`.|Check Shader syntax, variable names, function calls, types. Reference GLSL syntax and Makepad built-in functions (`Sdf2d`, `Pal`, `Math`).|
|`Name collision between X splat and Y`|`#[derive(Live)]` macro detected naming collision. Typically a `#[live]` field with same name as a field implicitly handled by attribute macro (like `#[walk]`).|Remove or rename conflicting `#[live]` field. E.g., if using `#[walk] walk: Walk`, don't also define `#[live] abs_pos: DVec2`.|
|`the trait bound X: LiveApply is not satisfied`|Typically a chain reaction from failed `#[derive(Live)]`, causing `LiveApply` trait not to be implemented, which in turn affects `#[derive(Widget)]`.|Resolve the root cause of `#[derive(Live)]` failure (typically naming collisions or other macro processing errors).|
|`no function or associated item named '...' found for struct X`|Same as above, failed `#[derive(Live)]` causing `LiveNew` trait methods (like `new`, `live_design_with`, `live_type_info`) not to be implemented.|Resolve the root cause of `#[derive(Live)]` failure.|

**Debugging Tips:**

- **Read error messages carefully:** Makepad error messages typically indicate the file, line number, and specific issue (currently requires modifying Live DSL file and reloading for precise line numbers).
- **Check spelling and case:** LiveIds and property names are case-sensitive.
- **Check syntax:** Ensure commas, colons, brackets are used correctly and match.
- **Reference documentation and examples:** Check official documentation or example code for property names, types, and usage.
- **Simplify code:** For hard-to-locate errors, try commenting out parts of the code to narrow down the problem area.
- **Check Rust definition:** For uncertain properties or structures, viewing the corresponding Widget's Rust source code (`#[derive(Live)]` part) is the most authoritative way.
- **Use Live Reload:** Save DSL file and observe if UI updates as expected, quickly validating small modifications. This method typically shows precise DSL line numbers for errors.

## Built-in Widgets

### **Built-in Widgets Overview (DSL Property Focus)**

- **`<View>`**: Basic container.
    - `show_bg: bool`
    - `draw_bg: <DrawColor>` (or override to other Draw* type)
    - `walk`, `layout`
    - `scroll_bars: <ScrollBars>`
    - `optimize: None/DrawList/Texture`
- **`<Label>`**: Display text.
    - `text: "string"`
    - `draw_text: <DrawText2>`
    - `walk`, `layout` (typically `width:Fit, height:Fit`)
- **`<Button>`**: Clickable button.
    - `text: "string"`
    - `draw_text: <DrawText2>`
    - `draw_icon: <DrawIcon>`
    - `draw_bg: <DrawQuad>` (or override)
    - `icon_walk`, `label_walk`, `walk`, `layout`
    - `enabled: bool`
- **`<LinkLabel>`**: Button with underline, for links.
    - Inherits from `<Button>`.
    - `url: "string"` (set in Rust)
    - `open_in_place: bool`
- **`<TextInput>`**: Text input field.
    - `text: "string"`
    - `empty_text: "string"`
    - `is_password`, `is_read_only`, `is_numeric_only: bool`
    - `draw_text`, `draw_bg`, `draw_cursor`, `draw_selection`
    - `walk`, `layout`
- **`<CheckBox>` / `<Toggle>`**: Checkbox/switch.
    - `text: "string"`
    - `draw_bg: <DrawCheckBox>` (can set `check_type: Check/Radio/Toggle/None`)
    - `bind: "path.to.data"`
- **`<RadioButton>`**: Radio button.
    - `text: "string"`
    - `value: EnumVariant`
    - `draw_bg: <DrawRadioButton>` (can set `radio_type: Round/Tab`)
    - Typically used within `<View>` or `<ButtonGroup>`.
- **`<Slider>`**: Slider control.
    - `text: "string"`
    - `min`, `max`, `default`, `step`, `precision`
    - `bind: "path.to.data"`
    - `text_input: <TextInput>` (built-in text input)
    - `draw_bg: <DrawSlider>`
- **`<DropDown>`**: Dropdown menu.
    - `labels: ["...", ...]`
    - `values: [Enum::Val1, ...]`
    - `selected_item: usize`
    - `popup_menu: <PopupMenu>`
    - `popup_menu_position: OnSelected/BelowInput`
    - `bind: "path.to.data"`
- **`<Image>`**: Display bitmap.
    - `source: dep("...")`
    - `fit: Stretch/Horizontal/Vertical/Smallest/Biggest/Size`
- **`<Icon>`**: Display SVG icon.
    - `draw_icon: { svg_file: dep("..."), color: ... }`
    - `icon_walk`
- **`<ScrollBars>`**: Scrollbar container (typically used in `<View>`'s `scroll_bars` property).
    - `show_scroll_x`, `show_scroll_y: bool`
    - `scroll_bar_x`, `scroll_bar_y: <ScrollBar>`
- **`<PortalList>` / `<PortalList2>`**: High-performance list.
    - Needs to be driven in Rust with `next_visible_item` and `item`.
    - `auto_tail: bool`
- **`<Html>` / `<Markdown>`**: Render formatted text.
    - `body: "string"`
- **`<PageFlip>` / `<SlidesView>`**: Page/slide switcher.
    - `active_page: live_id!(page_id)` / `current_slide: value`
    - Child elements define pages.
- **`<Dock>`**: Dockable panel system.
    - Needs to define `root`, `Tabs`, `Tab` structure.
- **`<Splitter>`**: Draggable separator.
    - `axis: Horizontal/Vertical`
    - `align: FromA/FromB/Weighted`
    - `a: <Widget>`, `b: <Widget>`
- **`<Modal>`**: Modal dialog.
    - `content: <Widget>`
- **`<PopupNotification>` / `RobrixPopupNotification`**: Non-modal popup notification.
    - `content: <Widget>`
- **`<AdaptiveView>`**: Responsive view switcher.
    - `Desktop = <Widget>`, `Mobile = <Widget>`
- **`<CachedWidget>`**: Cache child Widget.
    - `child_id = <Widget>`

### **Makepad Live DSL Quick Reference (Advanced Usage Supplement)**

#### `<View>` / `<ViewBase>` and Variants

- **Advanced Usage:**
    - **Custom Background Shader:** Write `fn pixel` or `fn vertex` directly in `draw_bg` to create complex background effects, gradients, patterns, or visual changes responding to state.
    - **Optimization (`optimize: Texture` or `DrawList`):** For `View`s containing a lot of static or infrequently changing content, can significantly improve performance. `Texture` mode draws content to a texture, `DrawList` mode caches draw commands.
    - **Nesting & Complex Layout:** Combine different `flow`, `align`, `spacing`, `padding`, `margin`, and `Size` modes (`Fill`, `Fit`, `Fixed`) to build complex, responsive layout structures.
    - **Event Handling & State:** Though `View` itself doesn't directly handle many interactions, it can capture `FingerDown`, `FingerHover` events and, combined with `animator`, change `instance` variables in `draw_bg` for visual feedback.
    - **Scrolling:** Use `<ScrollXView>`, `<ScrollYView>`, `<ScrollXYView>`, or add `scroll_bars: <ScrollBars> {}` to a regular `<View>` to create scrollable areas.
- **Example (Custom Shader & Optimization):**
    
    ```rust
    MyAdvancedView = <View> {
        width: 300, height: 200,
        optimize: Texture, // Optimize as texture cache
        show_bg: true,
        draw_bg: {
            texture image: texture2d // Need to set texture in Rust
            varying scale: vec2
            varying shift: vec2
    
            uniform time: float // Animation uniform
    
            fn vertex(self) -> vec4 { // Custom vertex shader (from CachedView)
                let dpi = self.dpi_factor;
                let ceil_size = ceil(self.rect_size * dpi) / dpi;
                let floor_pos = floor(self.rect_pos * dpi) / dpi;
                self.scale = self.rect_size / ceil_size;
                self.shift = (self.rect_pos - floor_pos) / ceil_size;
                return self.clip_and_transform_vertex(self.rect_pos, self.rect_size)
            }
    
            fn pixel(self) -> vec4 { // Custom pixel shader
                // Combine cached texture and time to create dynamic effect
                let cached_color = sample2d_rt(self.image, self.pos * self.scale + self.shift);
                let dynamic_color = Pal::hsv2rgb(vec4(fract(self.pos.x + self.time), 0.8, 0.8, 1.0));
                return mix(cached_color, dynamic_color, 0.5);
            }
        }
        // ... child components ...
    }
    ```
    

**`<View>`** is the only built-in Widget that can have the `event_order` property set.

1. **`<View>` Component:**
    
    - Documentation (`DSL/Widgets/View.md`) explicitly lists `event_order ([EventOrder](#eventorder))` as a field.
    - `EventOrder` enum allows you to specify event propagation order among its child components:
        - `Up` (default): From last child to first (similar to HTML DOM bubbling).
        - `Down`: From first child to last.
        - `List(Vec<LiveId>)`: In order of `LiveId`s specified in the list.
    - **Usage:** This is crucial for controlling which child component gets priority for receiving and handling events, especially in cases with overlapping areas or specific processing order needs (e.g., ensuring top-layer elements handle events first).
2. **Other Built-in Components:**
    
    - Most other built-in Widgets (like `<Button>`, `<Label>`, `<TextInput>`, `<Slider>`, etc.) **typically don't directly expose the `event_order` property** for users to set in DSL.
    - These components may internally use `<View>` as part of their base structure, but their own event handling logic is typically fixed or depends on the parent `<View>`'s `event_order`.
    - For example, a `<Button>` might internally have a `<View>` for layout of icon and text, but you can't directly set `event_order` on the `<Button>` to change the order in which icon and text receive events. You'd need to modify the internal `live_design!` definition of `<Button>` (if it's custom) or accept its default behavior.
    - For complex container Widgets like `<Dock>`, `<PortalList>`, `<Window>`, they have their own specific event dispatching logic to manage their children (Tabs, List Items, Child Windows), typically not controlled through a simple `event_order` property.

**Summary:**

- **`<View>` is the only core built-in Widget that can directly have `event_order` configured in `live_design!`.**
- You can indirectly control event handling order for other Widgets by nesting them within a `<View>` with specific `event_order`.
- For Makepad-provided preset Widget variants (like `<ButtonFlat>`, `<H1>`, etc.), they typically inherit from base Widgets (like `<Button>`, `<Label>`), and their event handling order is primarily determined by their internal structure and parent container.

If you need fine-grained control over event order for child elements of a specific component (not `<View>`), you might need to:

1. Wrap those child elements in a `<View>` and set that `<View>`'s `event_order`.
2. Create a custom version of the Widget and adjust its structure or use `<View>` with specific `event_order` in its internal `live_design!` definition.

#### `<Label>`

- **Advanced Usage:**
    
    - **Custom Text Shader:** Override `fn get_color` or `fn pixel` in `draw_text` to implement text color gradients, special effects, or color changes based on state.
    - **Rich Text (via TextFlow/Html/Markdown):** For text with mixed styles (bold, italic, links, etc.), use `<Html>` or `<Markdown>` Widgets, which internally use `<TextFlow>`, which in turn uses `<Label>` (or its DrawText) for underlying drawing.
    - **Responding to Hover Events:** Set `hover_actions_enabled: true`, then listen for `LabelAction::HoverIn/HoverOut` in Rust code, typically used for displaying Tooltips.
- **Example (Custom Color Shader):**
    
    ```rust
    GradientLabel = <Label> {
        text: "Gradient Text"
        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 24.0 }
            fn get_color(self) -> vec4 {
                // Gradient from red to blue, left to right
                return mix(#f00, #00f, self.pos.x);
            }
        }
    }
    ```
    

#### `<Button>`

- **Advanced Usage:**
    
    - **Fully Custom Appearance:** Override `draw_bg`, `draw_text`, `draw_icon` with `fn pixel` or `fn get_color` to completely change button appearance and state feedback.
    - **Complex Layout:** Use `<View>` and other Widgets inside `<Button>` to create complex button layouts with multiple elements (icon, text, status indicators) (requires Button's `flow` not to be `Overlay`, or custom drawing logic).
    - **Animated States:** Combine with `animator` for complex visual transitions between pressed, hover, disabled states.
    - **Data Binding/Actions:** In Rust, respond to `ButtonAction::Clicked/Pressed/Released` and execute different logic based on `action_data`.
- **Example (Custom Background and Animation):** (see Advanced example in documentation)
    

**`<LinkLabel>`**

- **Advanced Usage:**
    
    - **Custom Styling:** Since it inherits from `<Button>`, can override `draw_bg`, `draw_text` like Button to change underline style, text color, and hover/pressed effects.
    - **Dynamic URL:** Dynamically set `url` property in Rust code based on application state.
    - **Handle Clicks:** Capture `clicked` action in Rust to perform additional application logic besides opening URL.
- **Example (Custom Underline and Colors):**
    
    ```rust
    MyLink = <LinkLabel> {
        text: "Custom Link"
        draw_text: {
            color: #08A // Default color
            color_hover: #0CF // Hover color
            color_down: #048 // Pressed color
        }
        draw_bg: { // Control underline
            uniform color: #08A
            uniform color_hover: #0CF
            uniform color_down: #048
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let offset_y = 1.0;
                sdf.move_to(0., self.rect_size.y - offset_y);
                sdf.line_to(self.rect_size.x, self.rect_size.y - offset_y);
                // Use thicker, color-changing underline
                return sdf.stroke(mix(
                    mix(self.color, self.color_hover, self.hover),
                    self.color_down,
                    self.down
                ), 1.5); // Thicker underline
            }
        }
    }
    ```
    

#### `<TextInput>`

- **Advanced Usage:**
    
    - **Custom Appearance:** Override `draw_bg`, `draw_text`, `draw_cursor`, `draw_selection` to completely control input field visual style, including background, border, cursor, selection highlight.
    - **Input Validation/Filtering:** While DSL has `is_numeric_only`, more complex validation (like email format, max length) needs to be done in Rust's `handle_actions` by listening to `TextInputAction::Changed` and processing.
    - **Two-way Data Binding:** Use `bind` property to synchronize input field value with Rust data structure.
    - **Responding to Special Keys:** Listen for `TextInputAction::KeyDownUnhandled` in Rust to handle key events not processed internally by TextInput.
    - **Dynamic Read-only/Password Mode:** Call `set_is_read_only` or `set_is_password` in Rust code based on conditions.
- **Example (Custom Background and Cursor):** (see Advanced example in documentation)
    

#### `<CheckBox>` / `<Toggle>`

- **Advanced Usage:**
    
    - **Custom Mark:** Override `draw_bg` with `fn pixel` to draw any shape or icon instead of default check mark, circle, or switch slider. `check_type: None` can completely remove default drawing, letting you use `draw_icon` or other child Widgets to represent state.
    - **Custom Icon State:** In `<CheckBoxCustom>`, override `draw_icon`'s `fn get_color` or `fn pixel` to change icon color or appearance based on `active` (checked state), `hover`, `focus` instance variables.
    - **Data Binding:** Use `bind` property to synchronize checkbox's boolean state with data model.
- **Example (Custom Toggle Appearance):**
    
    ```rust
    MyToggle = <Toggle> {
        text: "My Toggle"
        draw_bg: {
            size: 10.0 // Control slider size base
            uniform mark_size: 0.8 // Control slider size relative to background
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.size;
                let left = sz + 1.;
                let c = vec2(left + sz, self.rect_size.y * 0.5);
                // Background track (Pill shape)
                sdf.box(left, c.y - sz, sz * 3.0, sz * 2.0, sz); // Corner radius equals half height
                sdf.fill(mix(#555, #0A0, self.active)); // Background color changes based on state
    
                // Slider (Circle)
                let isz = sz * self.mark_size;
                sdf.circle(left + sz + self.active * sz, c.y, isz); // Position changes based on active state
                sdf.fill(mix(mix(#AAA, #FFF, self.hover), #EEE, self.down)); // Slider color
    
                return sdf.result
            }
        }
    }
    ```
    

#### `<RadioButton>`

- **Advanced Usage:**
    
    - **Custom Mark/Appearance:** Similar to CheckBox, can override `draw_bg` with `fn pixel` to create custom radio button appearance (e.g., different selection mark, background shape). `radio_type: Tab` creates tab-style radio buttons.
    - **Grouping & State Management:** RadioButton itself doesn't handle exclusivity logic. Need to place them under same parent `<View>` and listen for `RadioButtonAction::Clicked` in Rust, then manually deselect other RadioButtons in same group (using `RadioButtonSet` or manual iteration).
    - **Custom Icon/Image:** Use `<RadioButtonCustom>` or `<RadioButtonImage>` variants, configure `draw_icon` or `image` property. Can combine with `animator` to change icon/image appearance based on `active` state.
    - **Data Binding:** Use `bind` and `value` to synchronize selected value to data model.
- **Example (Tab Style & State Management):**
    
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
        // Here you can update application state or bound data based on selected_index
        // RadioButtonSet::selected has already handled deselecting other buttons
    }
    ```
    

#### `<Slider>`

- **Advanced Usage:**
    
    - **Custom Appearance:** Override `draw_bg` with `fn pixel` to completely change slider track and handle appearance, including shape, color, gradients. You can use the `slide_pos` (0.0-1.0) instance variable to determine drawing position.
    - **Bipolar/Dual Slider:** Set `draw_bg: { bipolar: 1.0 }` to make value bar draw from center to both sides, useful for representing -1 to 1 or similar ranges.
    - **Custom Text Input:** Override `text_input` styling, or listen for `SliderAction::Slide` in Rust and manually update a separate `<Label>` or `<TextInput>`.
    - **Stepping & Precision:** Combine `step` and `precision` to control slider's discrete values and display format.
- **Example (Custom Rotary Appearance):** (see RotarySolid example in documentation)
    

#### `<DropDown>`

- **Advanced Usage:**
    
    - **Custom Appearance:** Override `draw_bg` and `draw_text` to change button style itself.
    - **Custom Popup Menu:** Override `popup_menu: <PopupMenu> { ... }` to fully customize popup menu background, border, and menu items (`menu_item: <PopupMenuItem> { ... }`) appearance (background, text, selection mark).
    - **Dynamic Options:** Dynamically modify `labels` and `values` lists in Rust code, then call `redraw(cx)` to update dropdown options.
    - **Data Binding:** Use `bind` to synchronize selected `value` to data model.
- **Example (Custom Popup Menu Items):**
    
    ```rust
    MyDropDown = <DropDown> {
        labels: ["A", "B", "C"],
        values: [OptionA, OptionB, OptionC],
        popup_menu: <PopupMenu> {
            // Custom menu background
            draw_bg: { color: #333, border_color: #666, border_size: 1.0 }
            menu_item: <PopupMenuItem> {
                // Custom menu item
                padding: {left: 20, top: 8, bottom: 8, right: 10}
                draw_bg: {
                    color: #333
                    color_hover: #555 // Hover background
                    color_active: #888 // Selected background
                }
                draw_text: {
                    color: #EEE // Default text color
                    color_hover: #FFF // Hover text color
                    color_active: #FFF // Selected text color
                }
            }
        }
    }
    ```
    

#### `<Image>` / `<Icon>` / `<ImageBlend>`

- **Advanced Usage:**
    
    - **Custom Shader:** Override `draw_bg` (Image) or `draw_icon` (Icon) with `fn pixel` to apply filters, color adjustments, blending effects, or other image processing.
    - **Animation:** Use `animator` to change `opacity`, `image_scale`, `image_pan` (DrawImage/DrawWebView) or `draw_icon` `instance` variables to achieve fade-in/out, scaling, panning, or color animation. `ImageBlend` has built-in `blend` animator for cross-fade.
    - **Dynamic Loading:** Use `load_image_dep_by_path`, `load_image_file_by_path`, `load_image_file_by_path_async`, or `set_texture` in Rust to dynamically change displayed image.
- **Example (ImageBlend Switch):** (see ImageBlend example in documentation and App code)
    

#### `<PortalList>` / `<PortalList2>`

- **Advanced Usage:**
    - **Multiple Templates:** Define multiple different list item templates (like `ListItemTypeA`, `ListItemTypeB`) in `live_design!`, then in Rust's `next_visible_item` loop, decide based on data whether to call `list.item(cx, item_id, live_id!(ListItemTypeA))` or `list.item(cx, item_id, live_id!(ListItemTypeB))` for each `item_id`.
    - **Infinite Scrolling:** Detect scrollbar approaching end in `handle_event` (via `ScrollBarsAction` or comparing `first_id` and `range_end`), then asynchronously load more data and update `range_end`.
    - **Pull-to-Refresh:** Combine `max_pull_down` and detection of `first_scroll > 0.0` to implement pull-to-refresh interaction.
    - **Preserving Item State:** If list items have complex state themselves (like icon loading, animation), ensure state is preserved when `item()` returns existing item. If using `reuse_items: true`, need to reset state when reused item is obtained.

#### `<Dock>` / `<Splitter>` / `<Tab>` / `<TabBar>`

- **Advanced Usage:**
    - **Dynamic Add/Remove Tabs:** Modify `Dock`'s `dock_items` state in Rust code (add/remove `Tab` and `Tabs` definitions), then call `redraw(cx)`. Need careful management of `LiveId`.
    - **Custom Tab Appearance:** Override `Tab` component's `draw_bg`, `draw_name`, `draw_icon` styles.
    - **Custom TabBar Appearance:** Override `TabBar`'s `draw_bg`, `draw_fill`, `draw_drag` styles.
    - **Custom Splitter Appearance:** Override `Splitter`'s `draw_bg` style.
    - **Save/Load Layout:** Serialize/deserialize `Dock`'s `dock_items` `HashMap` to save and restore user-customized layout. Need to handle `LiveId` conflicts (as shown in PR).

#### `<Html>` / `<Markdown>` / `<TextFlow>`

- **Advanced Usage:**
    - **Custom Styling:** Override `draw_normal`, `draw_italic`, `draw_bold` and other `DrawText2` properties, as well as `draw_block` (`DrawFlowBlock`) colors and `fn pixel` to change rendering style.
    - **Custom Block Layout:** Modify `code_layout`, `quote_layout`, `list_item_layout` and other `Layout` properties.
    - **Custom Links:** Override `link: <MyLink>` to use custom link component (needs to inherit from `LinkLabel` or `Button`).
    - **Interactive Elements:** Embed custom Widgets (like `<Button>`) in `Html` or `Markdown`, and handle their events in Rust. This typically requires manual handling of custom tags in `draw_walk`.

#### `<Modal>` / `<PopupNotification>`

- **Advanced Usage:**
    - **Custom Background:** Override `Modal`'s `bg_view` or `PopupNotification`'s `draw_bg` to change background appearance (e.g., different blur effect, color, or fully transparent).
    - **Custom Content:** `content` can be any complex Widget combination.
    - **Custom Animation:** Override `animator` to change popup/dismiss animation effects (e.g., fade-in/out, scaling, different easing functions).
    - **Conditional Open/Close:** Call `open(cx)` and `close(cx)` in Rust based on application logic.
    - **Handle Internal Actions:** Listen for actions emitted from Modal/Popup content area in parent Widget's `handle_actions`.

#### `<AdaptiveView>`

- **Advanced Usage:**
    - **Custom Selector:** Use `set_variant_selector` to provide complex logic to select view variant based on multiple factors (not just screen size, but also platform, device features, application state).
    - **State Retention:** Set `retain_unused_variants: true` to preserve non-active view state, avoiding re-initialization when switching back. Be mindful of memory usage.
    - **Nested AdaptiveView:** Can be nested for more complex responsive layouts.

#### `<CachedWidget>`

- **Advanced Usage:**
    - **Sharing Complex State:** Used for Widgets that need to be displayed in different parts of UI but are logically the same instance (e.g., a global music player control bar).
    - **Performance Optimization:** Cache expensive Widget initialization or drawing process.
    - **Note:** Being a singleton, all instances share state. Ensure this is desired behavior.

**Best practice is typically to define as much as possible declaratively in DSL, reserving complex logic and state management for Rust code.**

## Makepad Shader Language (MPSL) Quick Reference

Focus on features and built-in functions available in `fn pixel` and `fn vertex` functions within `draw_*` blocks in `live_design!`.

### **Core Concepts**

- **`fn pixel(self) -> vec4`**: Pixel shader function. Calculates and returns final color (RGBA) for current pixel. `self` contains `uniform`, `instance`, `varying` variables and built-in variables like `pos` (normalized 0-1 coordinates), `rect_pos`, `rect_size`.
- **`fn vertex(self) -> vec4`**: Vertex shader function. Calculates and returns final clip space position of vertex. Typically needs to set `varying` variables to pass to `pixel` function. `self` contains `uniform`, `instance`, `varying` and built-in variables like `geom_pos` (geometry normalized 0-1 coordinates), `rect_pos`, `rect_size`, `camera_projection`, `camera_view`, `view_transform`, `draw_clip`, `view_clip`, `draw_zbias`, `draw_depth`.
- **`self`**: Within shader functions, `self` contains all available `uniform`, `instance`, `varying` variables and built-in variables. Can be accessed directly via `self.propertyName`.
- **Variable Types**:
    - `float`: Single-precision floating point.
    - `vec2`, `vec3`, `vec4`: 2/3/4-dimensional float vectors.
    - `mat2`, `mat3`, `mat4`: 2x2, 3x3, 4x4 float matrices.
    - `int`, `ivec2`, `ivec3`, `ivec4`: Integer and vectors.
    - `bool`, `bvec2`, `bvec3`, `bvec4`: Boolean and vectors.
    - `texture2d`: 2D texture sampler.
    - `textureOES`: (Android-specific) OES texture sampler, typically for video.
- **Built-in Variables (Common):**
    - **`self.pos` (vec2, pixel shader):** Current pixel's normalized coordinates (0.0 to 1.0) within `rect_size`.
    - **`self.geom_pos` (vec2, vertex shader):** Input geometry (typically quad) normalized vertex coordinates (0.0 to 1.0).
    - **`self.rect_pos` (vec2):** Current drawing rectangle's top-left screen coordinates.
    - **`self.rect_size` (vec2):** Current drawing rectangle's dimensions (width, height).
    - **`self.draw_clip` (vec4):** Drawing clip region (xy=min, zw=max).
    - **`self.view_clip` (vec4):** View clip region (xy=min, zw=max).
    - **`self.view_shift` (vec2):** View scroll offset.
    - **`self.camera_projection` (mat4):** Camera projection matrix.
    - **`self.camera_view` (mat4):** Camera view matrix.
    - **`self.view_transform` (mat4):** View transform matrix.
    - **`self.draw_depth` (float):** Base drawing depth.
    - **`self.draw_zbias` (float):** Depth offset.
    - **`self.dpi_factor` (float):** Current DPI factor.
- **Variable Declaration:**
    - `uniform name: type`: Defined in DSL, shared by all instances (overridable).
    - `instance name: type`: Defined in DSL, unique per instance (commonly used for animation/state).
    - `varying name: type`: Defined in DSL, for passing data between `vertex` and `pixel`.
    - `let name: type = value;`: Declare local variable in function.
    - `var name: type = value;`: Declare mutable local variable in function.
- **Color Representation:**
    - `#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`: Hexadecimal colors.
    - `vec4(r, g, b, a)`: RGBA in 0.0 to 1.0 range.
    - `(THEME_COLOR_...)`: Reference theme color constant.

### **Built-in Function Libraries (`Sdf2d`, `Pal`, `Math`, `GaussShadow`)**

- **`Sdf2d` (Signed Distance Field 2D)**: For vector drawing.
    
    - `Sdf2d::viewport(pos: vec2) -> Self`: Create SDF context.
    - `sdf.clear(color: vec4)`: Cover result with color.
    - **Shape Definition:** (update `sdf.dist` and `sdf.shape`)
        - `sdf.circle(cx, cy, radius)`
        - `sdf.rect(x, y, w, h)`
        - `sdf.box(x, y, w, h, radius)`
        - `sdf.box_x(x, y, w, h, r_left, r_right)`
        - `sdf.box_y(x, y, w, h, r_top, r_bottom)`
        - `sdf.box_all(x, y, w, h, r_lt, r_rt, r_rb, r_lb)`
        - `sdf.hexagon(cx, cy, radius)`
        - `sdf.hline(y, half_thickness)`
        - `sdf.move_to(x, y)`
        - `sdf.line_to(x, y)`
        - `sdf.close_path()`
        - `sdf.arc2(cx, cy, radius, start_angle_rad, end_angle_rad)`
        - `sdf.arc_round_caps(cx, cy, radius, start_angle, end_angle, thickness)`
        - `sdf.arc_flat_caps(...)`
    - **Boolean Operations:** (typically called after shape definition)
        - `sdf.union()`: `shape = min(dist, old_shape)`
        - `sdf.intersect()`: `shape = max(dist, old_shape)`
        - `sdf.subtract()`: `shape = max(-dist, old_shape)`
    - **Drawing:**
        - `sdf.fill(color: vec4) -> vec4`: Fill and reset shape.
        - `sdf.fill_keep(color: vec4) -> vec4`: Fill and preserve shape.
        - `sdf.fill_premul(premultiplied_color: vec4) -> vec4`: Premultiplied Alpha fill and reset.
        - `sdf.fill_keep_premul(premultiplied_color: vec4) -> vec4`: Premultiplied Alpha fill and preserve.
        - `sdf.stroke(color: vec4, width: float) -> vec4`: Stroke and reset shape.
        - `sdf.stroke_keep(color: vec4, width: float) -> vec4`: Stroke and preserve shape.
        - `sdf.glow(color: vec4, width: float) -> vec4`: Add glow and reset shape.
        - `sdf.glow_keep(color: vec4, width: float) -> vec4`: Add glow, preserve shape.
    - **Effects:**
        - `sdf.gloop(k: float)`: Smooth blend with `old_shape` (Metaball).
        - `sdf.blend(k: float)`: Linear blend.
    - **Transforms:**
        - `sdf.translate(dx, dy)`
        - `sdf.rotate(angle_rad, pivot_x, pivot_y)`
        - `sdf.scale(factor, pivot_x, pivot_y)`
    - **Result:** `sdf.result` (final `vec4` color).
- **`Pal` (Palette)**: Color processing functions.
    
    - `Pal::premul(color: vec4) -> vec4`: Convert to premultiplied Alpha.
    - `Pal::hsv2rgb(hsv: vec4) -> vec4`: HSV to RGB conversion.
    - `Pal::rgb2hsv(rgb: vec4) -> vec4`: RGB to HSV conversion.
    - `Pal::iqX(t: float) -> vec3`: Inigo Quilez's palette functions (X=0..7).
- **`Math`**: Math functions.
    
    - `Math::rotate_2d(v: vec2, angle_rad: float) -> vec2`: Rotate 2D vector.
    - `Math::random_2d(v: vec2) -> float`: Generate pseudo-random number (0-1) based on input vector.
- **`GaussShadow`**: Gaussian blur shadow calculation.
    
    - `GaussShadow::box_shadow(lower: vec2, upper: vec2, point: vec2, sigma: float) -> float`: Calculate rectangle shadow blur value.
    - `GaussShadow::rounded_box_shadow(lower: vec2, upper: vec2, point: vec2, sigma: float, corner: float) -> float`: Calculate rounded rectangle shadow blur value.

### **GLSL Built-in Functions (Common)**

(Based on files in `builtin` directory in documentation)

- **Math:** `abs`, `ceil`, `clamp`, `degrees`, `distance`, `dot`, `exp`, `exp2`, `faceforward`, `floor`, `fract`, `inversesqrt`, `length`, `log`, `log2`, `max`, `min`, `mix`, `mod`, `normalize`, `pow`, `radians`, `reflect`, `refract`, `sign`, `smoothstep`, `sqrt`, `step`.
- **Trigonometry:** `acos`, `asin`, `atan`, `cos`, `sin`, `tan`.
- **Vector/Matrix:** `cross`, `matrixCompMult`, `transpose`, `inverse`.
- **Logic:** `all`, `any`, `equal`, `greaterThan`, `greaterThanEqual`, `lessThan`, `lessThanEqual`, `not`, `notEqual`.
- **Texture Sampling:**
    - `sample2d(sampler: texture2d, coord: vec2) -> vec4`: Sample 2D texture.
    - `sample2d_rt(sampler: texture2d, coord: vec2) -> vec4`: (Makepad-specific) May be for render target texture sampling, behavior similar to or with specific optimizations compared to `sample2d`.
    - `sample2dOES(sampler: textureOES, coord: vec2) -> vec4`: (Android-specific) Sample OES texture.
- **Derivatives:** `dFdx(p)`, `dFdy(p)`: Calculate partial derivatives of `p` with respect to screen x/y coordinates (Fragment Shader only).

### **Shader Writing Best Practices**

1. **Understand `self`:** Know what built-in variables are available in `self` (`pos`, `rect_pos`, `rect_size`, etc.) and what `uniform`/`instance`/`varying` variables you defined in DSL.
2. **Coordinate Systems:**
    - `self.pos`: Typically 0-1 normalized coordinates, relative to `self.rect_size`.
    - `self.geom_pos`: 0-1 geometry coordinates, typically used in vertex shader.
    - `Sdf2d::viewport(self.pos * self.rect_size)`: Converts normalized coordinates to pixel-level coordinates relative to current drawing rectangle, common starting point for SDF drawing.
3. **Leverage `Sdf2d`:** For vector graphics, prioritize `Sdf2d` provided functions, typically more convenient and optimized than manual calculations.
4. **State Management:** Understand `fill`/`stroke` reset `sdf.shape`, while `fill_keep`/`stroke_keep` don't. Boolean operations (`union`, `intersect`, `subtract`) update `sdf.shape` and `sdf.old_shape`.
5. **Premultiplied Alpha:** Makepad's rendering pipeline typically uses premultiplied Alpha. `Pal::premul` for conversion, `fill_premul`/`fill_keep_premul` for direct handling. Returning `sdf.result` is typically already premultiplied.
6. **Performance:**
    - Avoid overly complex calculations or loops in `pixel` function.
    - Texture sampling is relatively expensive.
    - Use `varying` to calculate values in `vertex` that can be interpolated between pixels.
7. **Animation & State:** Use `instance` variables to receive values from `animator` and use `mix()` or other logic in `pixel` or `vertex` function to change appearance based on these values.
8. **Debugging:**
    - Return fixed colors (`return #f00;`) to test code block execution.
    - Output intermediate values to color channels (`return vec4(value, 0.0, 0.0, 1.0);`).
    - Use `View`'s `debug` property.

## Appendix

### Appendix A: Terminology Concepts

#### "Normalized Relative Coordinates" (Normalized Relative Coordinates)

**Core Idea: Proportional Positioning**

Imagine a parent container, like a `<View>`, with certain width and height. When you place a child element in it, if the parent container's space is larger than the child element, the child element needs to know where to be placed in this "remaining space".

"Normalized Relative Coordinates" is a way to describe this position, not using specific pixel values but using **proportions**.

- **Normalized:** Means coordinate values are constrained to a standard range, usually **0.0 to 1.0**.
    - `0.0` represents the starting end (left or top).
    - `1.0` represents the ending end (right or bottom).
    - `0.5` represents the middle.
- **Relative:** Means this coordinate is **relative to the parent container's available space**, not relative to the absolute pixel position of the entire window or screen.

**Specifically for `align: {x: f64, y: f64}`:**

- **`align.x` (horizontal direction):**
    
    - `0.0`: Align child element's **left edge** with parent container's available space's **left edge**.
    - `0.5`: Align child element's **horizontal center** with parent container's available space's **horizontal center**.
    - `1.0`: Align child element's **right edge** with parent container's available space's **right edge**.
    - `0.25`: Place child element at position 1/4 from the left.
    - `0.75`: Place child element at position 3/4 from the left.
- **`align.y` (vertical direction):**
    
    - `0.0`: Align child element's **top edge** with parent container's available space's **top edge**.
    - `0.5`: Align child element's **vertical center** with parent container's available space's **vertical center**.
    - `1.0`: Align child element's **bottom edge** with parent container's available space's **bottom edge**.
    - `0.25`: Place child element at position 1/4 from the top.
    - `0.75`: Place child element at position 3/4 from the top.

**Available Space (Available Space):**

This is key to understanding relative coordinates. Available space refers to what's left in the parent container after placing all **non-aligned** child elements (e.g., elements arranged by `flow: Right` or `flow: Down`) and accounting for its own `padding`, **remaining** for placing **aligned** child elements (typically those with `width: Fit`, `height: Fit` or `width: Fixed`, `height: Fixed`).

- **For `flow: Right`:** Available horizontal space is parent container width minus all child element widths, spacing, and parent container left/right padding. Available vertical space is typically parent container height minus its top/bottom padding.
- **For `flow: Down`:** Available vertical space is parent container height minus all child element heights, spacing, and parent container top/bottom padding. Available horizontal space is typically parent container width minus its left/right padding.
- **For `flow: Overlay`:** Available space is typically the entire internal area of parent container minus padding.

**Why Use Normalized Relative Coordinates?**

1. **Responsive Layout:** This approach makes layout independent of specific pixel dimensions. No matter how the parent container scales or resizes, child elements always maintain their relative position (e.g., always centered, always right-aligned).
2. **Simplicity:** Using 0.0 to 1.0 range to express alignment is simpler and more intuitive than complex pixel calculations or constraints.
3. **Relationship with `Size::Fill`:** When a child element uses `Size::Fill`, it occupies all available space, making `align` in that direction typically ineffective since there's no remaining space for alignment.

**Example Understanding:**

```live_design!
<View> { // parent container
    width: 200, height: 100,
    padding: 10, // 10 padding on all sides
    show_bg: true, draw_bg: { color: #eee }
    // Available space: width 180 (200-10-10), height 80 (100-10-10)

    align: { x: 0.5, y: 0.5 } // Child element centered horizontally and vertically

    <Button> {
        text: "OK",
        width: Fit, height: Fit // Assume button size is 60x30
    }
}
```

In this example:

- Parent container's available internal width is 180, available height is 80.
- Button size is 60x30.
- Horizontal remaining space is 180 - 60 = 120.
- Vertical remaining space is 80 - 30 = 50.
- `align: {x: 0.5, y: 0.5}` means button's center point should align with available space's center point.
- Button's top-left x coordinate = parent container left padding + horizontal remaining space * `align.x` = 10 + 120 * 0.5 = 70.
- Button's top-left y coordinate = parent container top padding + vertical remaining space * `align.y` = 10 + 50 * 0.5 = 35.
- So button will ultimately be placed at position `(70, 35)` relative to parent container's top-left corner.

In summary, normalized relative coordinates provide a resolution-independent, proportion-based way to define child element alignment within parent container's available space.