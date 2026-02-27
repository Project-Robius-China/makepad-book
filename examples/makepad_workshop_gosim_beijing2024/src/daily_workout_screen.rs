use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::styles::*;

    FIRE = dep("crate://self/resources/img/fire.png")

    pub Exercise = <RoundedView> {
        cursor: Hand
        show_bg: true
        draw_bg: {
            color: #333645
        }
        flow: Right
        align: {x: 0.5, y: 0.5}
        width: Fill, height: Fit
        padding: 10
        spacing: 10
        content = <SectionDown> {
            width: Fill, height: Fit
            spacing: 10
            header = <SectionRight> {
                spacing: 10
                width: Fill, height: Fit
                <RoundedView> {
                    height: 20, width: 8
                    show_bg: true
                    draw_bg: {
                        color: #b6f36a
                        radius: 2
                    }
                }
                title = <Label> {
                    text: "Burpees"
                    draw_text: {
                        color: #xf
                        text_style: <TextBold> {
                            font_size: 11
                        }
                    }
                }
                <View> {width: Fill}
                series =<Label> {
                    text: "15 x 3"
                    draw_text: {
                        color: #xf
                        text_style: <TextBold> {
                            font_size: 12
                        }
                    }
                }
            }
            // sub_header =<View> {
                // width: Fill, height: Fit
            description = <Label> {
                text: "Biceps, triceps and shoulders"
                draw_text: {
                    color: #c
                    text_style: {
                        font_size: 10
                    }
                }
            }
            // }
            // <SectionRight> {
            //     width: Fill, height: Fit
            //     // <SectionDown> {
            //     //     spacing: 0
            //     //     width: Fit, height: Fit
            //     //     set_1_chbox = <CheckBox> { text: "Set 1"}
            //     //     set_2_chbox = <CheckBox> { text: "Set 2"}
            //     //     set_3_chbox = <CheckBox> { text: "Set 3"}
            //     // }
            //     // <View> { width: 50 }
            //     log_button = <Button> {
            //         height: 50
            //         width: Fit
            //         padding: {left: 20, right: 20}
            //         text: "Done"
            //         draw_text: {
            //             color: #f
            //             text_style: <TextBold> {
            //                 font_size: 12
            //             }
            //         }
            //     }
            // }
        }
        log_button = <Button> {
            height: 50, width: Fit
            padding: {left: 15, right: 15}
            text: "Log"
            draw_text: {
                color: #f
                text_style: <TextBold> {
                    font_size: 12
                }
            }
        }
    }

    pub DailyWorkoutScreen = {{DailyWorkoutScreen}} {
        flow: Down, spacing: 10
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: (COLOR_BG)
        }

        padding: {top: 100, left: 25, right: 25}

        <Label> {
            text: "Today's Activity"
            draw_text: {
                color: #f
                text_style: <TextBold> {
                    font_size: 15
                }
            }
        }

        <Label> {
            text: "You have a full body workout today!"
            draw_text: {
                color: #f
                text_style: {
                    font_size: 12
                }
            }
        }

        // Calories bar
        <RoundedView> {
            margin: {top: 20}
            spacing: 10
            flow: Right
            width: Fill, height: 500

            calories = <RoundedView> {
                flow: Down
                align: {x: 0.5, y: 0.5}
                width: 100, height: 310
                show_bg: true
                draw_bg: {color: #c55367}

                <RoundedView> {
                    padding: 10
                    margin: {bottom: 30}
                    width: Fit, height: Fit
                    show_bg: true
                    draw_bg: { color: #a53b57 }
                    <Image> {
                        source: (FIRE)
                        width: 40, height: 40
                    }
                }

                calories_number = <Label> {
                    text: "0"
                    draw_text: {
                        color: #f
                        text_style: <TextBold> {
                            line_spacing: 1
                            font_size: 20
                        }
                    }
                }

                <Label> {
                    text: "Calories"
                    draw_text: {
                        color: #f
                        text_style: {
                            line_spacing: 1
                            font_size: 12
                        }
                    }
                }
            }
            items = <SectionDown> {
                width: Fill, height: Fit
                spacing: 10
                burpees = <Exercise> {}
                box_jumps = <Exercise> {
                    content = { 
                        header = { 
                            title = { text: "Box Jumps"}
                        }
                        description = {
                            text: "Legs, glutes and core"
                        }
                    }
                }
                rope_jumping = <Exercise> {
                    content = {
                        header = {
                            title = { text: "Jump Rope"}
                            series = { text: "5 min"}
                        }
                        description = {
                            text: "Full body"
                        }
                    }
                }
                jump_squats = <Exercise> {
                    content = {
                        header = {
                            title = { text: "Jump Squats"}
                            series = { text: "3 x 10"}
                        }
                        description = {
                            text: "Legs, glutes and core"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Widget, Live, LiveHook)]
pub struct DailyWorkoutScreen {
    #[deref]
    view: View,

    #[rust]
    calories: usize,
}

impl Widget for DailyWorkoutScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for DailyWorkoutScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let burpees_button = self.button(id!(burpees.log_button));
        let box_jumps_button = self.button(id!(box_jumps.log_button));
        let rope_jumping_button = self.button(id!(rope_jumping.log_button));
        let jump_squats_button = self.button(id!(jump_squats.log_button));

        // Burpees
        if burpees_button.clicked(actions) {
            self.calories += 180;
            self.label(id!(calories_number))
                .set_text(cx, &format!("{0}", self.calories));

            self.view(id!(burpees)).apply_over(cx, live! {
                draw_bg: {color: #x2a2c36 }
            });

            burpees_button.set_enabled(cx, false);
            burpees_button.set_text(cx, "Done");
        }

        // Box Jumps
        if box_jumps_button.clicked(actions) {
            self.calories += 60;
            self.label(id!(calories_number))
                .set_text(cx, &format!("{0}", self.calories));

            self.view(id!(box_jumps)).apply_over(cx, live! {
                draw_bg: {color: #x2a2c36 }
            });

            box_jumps_button.set_enabled(cx, false);
            box_jumps_button.set_text(cx, "Done");
        }

        // Jump Rope
        if rope_jumping_button.clicked(actions) {
            self.calories += 100;
            self.label(id!(calories_number))
                .set_text(cx, &format!("{0}", self.calories));

            self.view(id!(rope_jumping)).apply_over(cx, live! {
                draw_bg: {color: #x2a2c36 }
            });

            rope_jumping_button.set_enabled(cx, false);
            rope_jumping_button.set_text(cx, "Done");
        }

        // Jump Squats
        if jump_squats_button.clicked(actions) {
            self.calories += 70;
            self.label(id!(calories_number))
                .set_text(cx, &format!("{0}", self.calories));

            self.view(id!(jump_squats)).apply_over(cx, live! {
                draw_bg: {color: #x2a2c36 }
            });

            jump_squats_button.set_enabled(cx, false);
            jump_squats_button.set_text(cx, "Done");
        }
    }
}
