# Makepad Workshop

> Author: [Julián Montes de Oca](https://github.com/joulei)
> 
> Project From: [GOSIM Beijing 2024](https://china2024.gosim.org/)

## Cheat Sheet
### Widgets
#### View
```rust
<View> {
    width: Fit, height: Fill // or fixed, e.g. width: 250
    padding: {top: 10, right: 20, bottom: 10, left: 20}
    margin: 10

	show_bg: true // required to `draw_bg`, otherwise the background is transparent
	draw_bg: {
		color: #ffffff
	}
}
```

#### Label
```rust
<Label> {
	text: "Hello, World"
	draw_text: {
		color: #x0 // changes the text color
		text_style: {
			font_size
			font_family
		}
	}
}
```
#### Button
```rust
<Button> {
	text: "Click me"
	draw_text: { /*...*/ }
	draw_bg: { /*...*/ }
	draw_icon: { /*...*/ }
}
```
#### Inheritance
```rust
MyButton = <Button> {
	text: "Click me"
}

<View> {
	<MyButton> {}
	<MyButton> { text: "Clickity click" } // anything can be overwitten
	<MyButton> {}
}
```

### Implementing a Custom Widget
```rust
// dsl:
live_design! {
	// ...

	MyWidget = {{MyWidget}} {
		// use this like a normal view

		<Label> { /*...*/ }
		<Button> { /*...*/ }
	}
}

// rust:

#[derive(Widget, Live, LiveHook)]
struct MyWidget {
	#[deref]
	view: View

	#[rust]
	counter: i32
}

impl Widget for MyWidget {
	fn handle_event() {
		self.view.handle_event()
	}
	fn draw_walk() -> {
		self.view.draw_walk()
	}
}
```

#### Events

Signals from user interactions like clicks, key presses, or system changes like window resize, etc. Event usually come from the platform level.
#### Actions

Represent internal state changes of the application or operations that need to be executed. 
For example `Button` emits a `ButtonAction::Clicked` whenever the button is clicked.

Widgets usually implement helpers so you can "ask" a specific widget instance if an action happened:
```rust
impl MatchEvent for App {
	fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
		if self.button(id!(my_button_id)).clicked(&actions) {
			log!("button clicked!");
		}
	}
}
```

Actions can also be used to communicate state to widgets, you can dispatch a actions that will be observed by all visible widgets.
For example, `StackNavigation` handles `StackNavigationAction::NavigateTo` which tells the Widget to perform navigation.

#### StackNavigation

In the DSL, add your views inside a `StackNavigationView` within the `StackNavigation`.
```
nav = <StackNavigation> {
    root_view = <Home> {}
    program_view = <StackNavigationView> {
        header = {
            content = {
                title_container = {
                    title = {
                        text: "Program"
                    }
                }
            }
        }
        body = <MyView> {}
    }
}
```
In the rust code, make sure to handle the navigation actions:
```rust
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut navigation = self.ui.stack_navigation(id!(nav));
        navigation.handle_stack_view_actions(cx, &actions);
    }
}
```
You can navigate to a view by calling:
```rust
cx.widget_action(
	widget_uid,
	&Scope::default().path,
	StackNavigationAction::NavigateTo(live_id!(my_view)),
);
```
### Querying DSL

```rust
self.button(id!(my_button_id)) // this returns a ButtonRef

self.button(id!(my_button_id)).clicked(&actions)

self.view(id!(my_view)).apply_over(cx, live!{ draw_bg: { color: #f }})

self.view(id!(my_view)).visible(false)
```
#### Timer
You can use Makepad's Timer API to create a timer that will dispatch events at a given interval.

For example, to create a timer that will dispatch an event every second, you can do:
```rust
let timer = cx.start_interval(1.0);
```
Where `timer` is the timer id, which you can then use to check
if an event is a tick of the timer:
```rust
fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
    if self.timer.is_event(event).is_some() {
        // do something every tick
    }
}
```
You can also stop the timer by calling `cx.stop_timer(timer_id);`

### Animator
First, include the animator in your widget:
```rust
#[animator]
animator: Animator,
```

Make sure to handle the animator events:
```rust
impl Widget for MyWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.view.redraw(cx);
        }

        self.view.handle_event(cx, event, scope);
    }
}
```

Then, in your DSL you can use Animator to modify properties over time.
```rust
animator: {
    shake = {
        default: init, // the default state of the animator
        init = {
            from: {all: Snap} // Snap means the animation will jump to the end state immediately
            apply: { // what to apply over the DSL in this state
                image = {
                    draw_bg: { rotation: 0.0} // override the rotation of the image
                }
            }
        }

        on = {
            redraw: true, // redraw if this state is active
            from: {all: BounceLoop {duration: 0.5, end: 1.0}} // BounceLoop means the animation will bounce back and forth
            apply: {
                image = {
                    draw_bg: { rotation: 0.5} // override the rotation of the image
                }
            }
        }
    }
}
```
Finally, to start the animation, call:
```rust
if !self.animator_in_state(cx, id!(shake.on)) { // don't play if already in that state
    self.animator_play(cx, id!(shake.on));
}
```
