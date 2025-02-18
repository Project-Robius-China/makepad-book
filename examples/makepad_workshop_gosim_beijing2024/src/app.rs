use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::styles::*;
    use crate::home::*;
    use crate::daily_workout_screen::*;
    use crate::abs_routine_screen::*;
    use crate::product_screen::*;

    pub App = {{App}} {
        ui: <Window> {
            window: {position: vec2(970, 80), inner_size: vec2(440, 800)},

            body = {
                nav = <StackNavigation> {
                    root_view = <Home> {}
                    product_view = <StackNavigationView> {
                        header = {
                            // padding: {left: 20.}
                            content = {
                                title_container = {
                                    title = {
                                        text: "Store"
                                    }
                                }
                            }
                        }
                        body = <ProductScreen> {}
                    }
                    daily_workout_view = <StackNavigationView> {
                        header = {
                            content = {
                                title_container = {
                                    title = {
                                        text: "Exercise"
                                    }
                                }
                            }
                        }
                        body = <DailyWorkoutScreen> {}
                    }
                    abs_routine_view = <StackNavigationView> {
                        header = {
                            content = {
                                title_container = {
                                    title = {
                                        text: "Timer"
                                    }
                                }
                            }
                        }
                        body = <AbsRoutineScreen> {}
                    }
                }
            }
        }
    }
}

// Registers App as the main module for this application
app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

/// Registers all the live components for this application
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        // Order matters here, as some widget definitions depend on others.
        // `makepad_widgets` must be registered first,
        // then `shared`` widgets (in which styles are defined),
        // then other modules widgets.
        makepad_widgets::live_design(cx);
        crate::styles::live_design(cx);
        crate::home::live_design(cx);
        crate::abs_routine_screen::live_design(cx);
        crate::daily_workout_screen::live_design(cx);
        crate::product_screen::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let navigation = self.ui.stack_navigation(id!(nav));
        navigation.handle_stack_view_actions(cx, &actions);

        if let Some(_) = self.ui.view(id!(store_section)).finger_down(&actions) {
            cx.widget_action(
                self.ui.widget_uid(),
                &Scope::default().path,
                StackNavigationAction::NavigateTo(live_id!(product_view)),
            );
        }

        if let Some(_) = self.ui.view(id!(daily_workout)).finger_down(&actions) {
            cx.widget_action(
                self.ui.widget_uid(),
                &Scope::default().path,
                StackNavigationAction::NavigateTo(live_id!(daily_workout_view)),
            );
        }

        if let Some(_) = self.ui.view(id!(abs_routine)).finger_down(&actions) {
            cx.widget_action(
                self.ui.widget_uid(),
                &Scope::default().path,
                StackNavigationAction::NavigateTo(live_id!(abs_routine_view)),
            );
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        self.match_event(cx, event);
    }
}
