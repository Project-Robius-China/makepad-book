use makepad_widgets::*; // Import Makepad Widgets package

// Define live_design macro for declaring UI components and layout
live_design! {
    // Import base components and desktop theme
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Define App component
    App = {{App}} {
        // Define the UI tree root node
        ui: <Root>{
            // Define main window
            main_window = <Window>{
                // Show background
                show_bg: true
                width: Fill,
                height: Fill


                // Define custom background drawing
                draw_bg: {
                    fn pixel(self) -> vec4 {
                         // Get geometric position
                        let st = vec2(
                            self.geom_pos.x,
                            self.geom_pos.y
                        );

                         // Calculate color based on x and y position and time
                        let color = vec3(st.x, st.y, abs(sin(self.time)));
                        return vec4(color, 1.0);
                    }
								}
                // Define window body
                body = <ScrollXYView>{
                    // Vertical layout direction
                    flow: Down,
                    // 10-unit spacing between children
                    spacing:10,
                    // Alignment
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    // Button component
                    button1 = <Button> {
                        text: "Hello world"
                        draw_text:{color:#f00}  // Red text color
                    }


                    // Text input component
                    label1 = <Label> {
                        draw_text: {
                            color: #f // White text color
                        },
                        text: "Click to count "
                    }

                    // Text input component
                    input1 = <TextInput> {
                        width: 100, height: 30
                        text: "Counter: 0 "
                    }
                }
            }
        }
    }
}

// Define application entry point
app_main!(App);

// Define App struct containing UI and counter
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef, // UI component reference
    #[rust]
    counter: usize, // Counter
}

// Implement LiveRegister trait for registering live design
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        // Register Makepad Widgets' live design
        makepad_widgets::live_design(cx);
    }
}

// Implement AppMain trait for handling events
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Match and handle events
        self.match_event(cx, event);
        // Handle UI events
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

// Implement MatchEvent trait for handling events
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Check if button was clicked
        // We can directly use the button1 instance through `id!()` to get the clicked Button
        // `clicked` is a method of the Button component
        if self.ui.button(id!(button1)).clicked(&actions) {
            // Increment counter
            log!("BUTTON jk {}", self.counter);
            self.counter += 1;

            // Update label text
            // Similarly, get the input1 text input instance through `id!()`
            let input = self.ui.text_input(id!(input1));
            // Use the text input component's built-in `set_text_and_redraw`
            // to update the text input content with the new counter value and trigger immediate redraw
            input.set_text(cx, &format!("Counter: {}", self.counter));
        }
    }
}