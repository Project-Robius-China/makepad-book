# Makepad Chart

> https://github.com/mofa-org/makepad-chart

GPU-accelerated charting library for [Makepad](https://github.com/makepad/makepad) with ~75% Chart.js parity.

![Chart Zoo](https://img.shields.io/badge/Charts-11%20Types-blue)
![Platform](https://img.shields.io/badge/Platform-Cross--Platform-green)

## Features

- **11 Chart Types**: Bar, Line, Pie, Doughnut, Scatter, Bubble, Radar, Polar Area, Combo, Horizontal Bar, Chord
- **GPU Accelerated**: All rendering done via Makepad's GPU shader system
- **Animations**: Smooth animations with 28 easing functions + delay animation + progressive animation
- **Gradients**: Vertical, radial, and angular gradients for all chart types
- **Progressive Animation**: Dense time-series with 1000+ points drawing left-to-right
- **Chart Overlays**: Layer multiple chart types using absolute positioning
- **Multi-Dataset Colors**: Color-coded datasets with custom color support
- **Interactive**: Hover effects and click detection
- **Cross-Platform**: Works on Desktop, Web (WASM), iOS, and Android

## Run Example

```bash
cargo run --example chart_zoo
```
## Renderings

![Chart Zoo](./images/makepad-charts-zoo.png)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "main" }
makepad-charts = { git = "https://github.com/mofa-org/makepad-chart", branch = "main" }
```

## Quick Start

### 1. Register the library

```rust
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_charts::live_design(cx);  // Add this
    }
}
```

### 2. Add chart to your UI

```rust
live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_charts::chart::bar_chart::BarChart;

    App = {{App}} {
        ui: <Window> {
            body = <View> {
                my_chart = <BarChart> {
                    width: Fill,
                    height: 300,
                }
            }
        }
    }
}
```

### 3. Set chart data

```rust
use makepad_charts::*;
use makepad_charts::chart::bar_chart::BarChart;

impl App {
    fn setup_chart(&mut self, cx: &mut Cx) {
        let data = ChartData::new()
            .with_labels(vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
            .add_dataset(
                Dataset::new("Sales")
                    .with_data(vec![65.0, 59.0, 80.0, 81.0, 56.0, 72.0])
            );

        if let Some(mut chart) = self.ui.widget(id!(my_chart)).borrow_mut::<BarChart>() {
            chart.set_data(data);
            chart.set_options(ChartOptions::new().with_begin_at_zero(true));
        }

        self.ui.redraw(cx);
    }
}
```

## Chart Types

### Bar Chart
```rust
use makepad_charts::chart::bar_chart::BarChart;

// Vertical bars, supports stacked and grouped modes
chart.set_stacked(true);  // Enable stacking
chart.set_delay_animation(true);  // Staggered animation
chart.set_gradient(true);  // Vertical gradient on bars
```

### Line Chart
```rust
use makepad_charts::chart::line_chart::{LineChart, SteppedMode};

chart.set_fill(true);  // Area chart
chart.set_stepped(SteppedMode::After);  // Stepped line
chart.set_show_points(true);  // Show data points
chart.set_progressive_animation(true);  // Draw left-to-right
chart.set_gradient(true);  // Enable area gradient
```

### Pie / Doughnut Chart
```rust
use makepad_charts::chart::pie_chart::PieChart;

chart.set_doughnut(true);  // Doughnut mode
chart.set_radial_gradient(true);  // Enable gradient
```

### Scatter Chart
```rust
use makepad_charts::chart::scatter_chart::ScatterChart;

let data = ChartData::new()
    .add_dataset(
        Dataset::new("Points")
            .with_xy_data(vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)])
    );

chart.set_gradient(true);  // Radial gradient on points
```

### Bubble Chart
```rust
use makepad_charts::chart::bubble_chart::BubbleChart;

let data = ChartData::new()
    .add_dataset(
        Dataset::new("Bubbles")
            .with_bubble_data(vec![
                (x, y, radius),  // Each point has x, y, and radius
            ])
    );

chart.set_gradient(true);  // Radial gradient on bubbles
```

### Radar Chart
```rust
use makepad_charts::chart::radar_chart::RadarChart;

chart.set_fill(true);
chart.set_gradient(true);  // Radial gradient fill
```

### Polar Area Chart
```rust
use makepad_charts::chart::polar_area_chart::PolarAreaChart;
// Equal-angle segments with radius based on value
```

### Combo Chart (Bar + Line)
```rust
use makepad_charts::chart::combo_chart::{ComboChart, DatasetType};

chart.set_dataset_types(vec![DatasetType::Bar, DatasetType::Line]);
```

### Horizontal Bar Chart
```rust
use makepad_charts::chart::horizontal_bar_chart::HorizontalBarChart;
// Horizontal bars with Y-axis categories
```

### Chord Diagram
```rust
use makepad_charts::chart::chord_chart::{ChordChart, ChordData};

// Matrix-based relationship data
let data = ChordData::new()
    .with_labels(vec!["A", "B", "C", "D"])
    .with_matrix(vec![
        vec![0.0, 50.0, 30.0, 10.0],  // Flow from A to B, C, D
        vec![20.0, 0.0, 40.0, 15.0],  // Flow from B to A, C, D
        vec![10.0, 25.0, 0.0, 35.0],  // Flow from C to A, B, D
        vec![5.0, 10.0, 20.0, 0.0],   // Flow from D to A, B, C
    ]);

chart.set_data(data);
chart.set_gradient(true);      // Gradient ribbons
chart.set_directed(true);      // Arrow-like directed ribbons
chart.set_arc_gradient(true);  // Gradient on outer arcs
chart.set_gap_angle(0.05);     // Gap between groups
chart.set_arc_thickness(0.08); // Outer arc thickness
```

## Animation

### Basic Animation
All charts animate on load by default. Configure via `ChartOptions`:

```rust
chart.set_options(
    ChartOptions::new()
        .with_animation_duration(500.0)  // ms
        .with_animation_easing(EasingType::EaseOutQuart)
);
```

### Delay Animation (Staggered)
Bar charts support Chart.js-style delay animation:

```rust
chart.set_delay_animation(true);
chart.set_delay_timing(80.0, 40.0);  // per_index_ms, per_dataset_ms
```

### Replay Animation
```rust
chart.replay_animation(cx);
```

### Progressive Animation (Dense Data)
For time-series or stock market charts with 1000+ points:

```rust
// Enable progressive mode - draws points left-to-right
chart.set_progressive_animation(true);
chart.set_tension(0.1);  // Slight smoothing for dense data
chart.set_show_points(false);  // Hide points for performance

chart.set_options(ChartOptions::new()
    .with_animation_duration(3000.0)
    .with_animation_easing(EasingType::EaseOutCubic));
```

## Chart Overlays

Layer multiple chart types using absolute positioning:

```rust
live_design! {
    ChartOverlay = <View> {
        width: Fill, height: 500,

        // Base chart (renders first, behind)
        bubble_chart = <BubbleChart> { width: Fill, height: Fill }

        // Overlay chart (renders on top)
        scatter_chart = <ScatterChart> {
            width: Fill, height: Fill,
            abs_pos: vec2(0.0, 0.0)
        }
    }
}
```

Both charts should use the same `ChartOptions` for axis alignment.

## Gradients

All chart types support GPU-accelerated gradients:

### Bar Chart - Vertical Gradient
```rust
bar_chart.set_gradient(true);  // Bottom to top gradient
```

### Line Chart - Area Gradient
```rust
line_chart.set_fill(true);
line_chart.set_gradient(true);  // Top to bottom gradient on fill
```

### Scatter/Bubble - Radial Gradient
```rust
scatter_chart.set_gradient(true);  // Center to edge gradient
bubble_chart.set_gradient(true);   // Center to edge gradient
```

### Pie/Doughnut Gradients
```rust
pie_chart.set_radial_gradient(true);   // Inner to outer
pie_chart.set_angular_gradient(true);  // Along the arc
```

### Radar/Polar Area Gradients
```rust
radar_chart.set_gradient(true);       // Center to edges
polar_area_chart.set_gradient(true);  // Center to edges
```

## Data Structures

### ChartData
```rust
let data = ChartData::new()
    .with_labels(vec!["A", "B", "C"])  // X-axis labels
    .add_dataset(dataset1)
    .add_dataset(dataset2);
```

### Dataset
```rust
// Simple Y values
Dataset::new("Label").with_data(vec![1.0, 2.0, 3.0])

// X/Y pairs (scatter)
Dataset::new("Label").with_xy_data(vec![(1.0, 2.0), (3.0, 4.0)])

// Bubble data (x, y, radius)
Dataset::new("Label").with_bubble_data(vec![(1.0, 2.0, 5.0)])

// Floating bars (min, max)
Dataset::new("Label").with_floating_data(vec![(-5.0, 10.0), (0.0, 15.0)])

// Custom color
Dataset::new("Label")
    .with_data(vec![1.0, 2.0])
    .with_color(vec4(0.3, 0.5, 0.9, 1.0))
```

### Multi-Dataset Color Coding
```rust
let data = ChartData::new()
    .with_labels(labels)
    .add_dataset(Dataset::new("MKPD").with_data(stock1))  // Default color
    .add_dataset(Dataset::new("TECH").with_data(stock2)
        .with_color(vec4(0.95, 0.5, 0.3, 1.0)))  // Orange
    .add_dataset(Dataset::new("INDEX").with_data(stock3)
        .with_color(vec4(0.4, 0.8, 0.5, 1.0)));  // Green
```

### ChartOptions
```rust
ChartOptions::new()
    .with_begin_at_zero(true)
    .with_animation_duration(400.0)
    .with_animation_easing(EasingType::EaseOutQuart)
```
