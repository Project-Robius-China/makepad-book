use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::styles::*;

    SLEEP_IMG = dep("crate://self/resources/img/sleep.png")

    MOON = dep("crate://self/resources/img/moon.png")
    BOTTLE = dep("crate://self/resources/img/bottle.png")
    CARROT = dep("crate://self/resources/img/carrot.png")
    DUMBELL = dep("crate://self/resources/img/dumbell.png")
    FIRE = dep("crate://self/resources/img/fire.png")
    FIRE_DARK = dep("crate://self/resources/img/fire_dark.png")
    SNEAKER = dep("crate://self/resources/img/sneaker.png")

    pub HomeHeader = <View> {
        flow: Down
        width: Fill,
        height: Fit
        <Label> { 
            padding: {left: 8}
            text: "Good morning,", 
            draw_text: {
                // color: #9,
                text_style: <TextBold> {
                    font_size: 15.0
                }
            } 
        }
        <Label> { 
            padding: {left: 8}
            text: "Julian", 
            draw_text: {
                color: #f,
                text_style: <TextBold> {
                    font_size: 15.0
                }
            } 
        }
    }

    pub Box = <RoundedView> {
        cursor: Hand
        draw_bg: {
            radius: 5
        }
        padding: 20
        flow: Down
        width: Fill

        header = <SectionRight> {
            spacing: 10
            icon = <Image> {
                width: 20, height: 20
            }
            title = <Label> {
                draw_text: {
                    color: #2,
                    text_style: {
                        font_size: 13.0
                    }
                }
            }
        }
    }

    pub Pill = <RoundedView> {
        width: 80, height: 30
        align: {x: 0.5, y: 0.5}
        draw_bg: {
            radius: 7.0
            border_width: 1.0
            border_color: #3
        }
        lbl = <Label> {
            draw_text: {
                text_style: <TextBold> {}
                // color: #x7
            }
        }
    }

    pub SleepBox = <Box> {
        draw_bg: {
            color: #c9a0ff
        }
        align: {x: 0.0, y: 0.0}
        padding: 20
        height: 120 
        flow: Down
        header = {
            icon = { source: (MOON) }
            title = { text: "Sleep" }
        }

        <SectionDown> {
            width: Fill, height: Fit
            align: {x: 1.0}
            <Label> { 
                text: "Time in Bed"
                draw_text: {
                    color: #4a,
                    text_style: {
                        font_size: 12.0
                    }
                }
            }
            <Label> { 
                text: "7hr 18min"
                draw_text: {
                    color: #3a,
                    text_style: {
                        font_size: 20.0
                    }
                }
            }
        }
    }

    pub NutritionBox = <Box> {
        draw_bg: {
            color: #fede67
        }
        height: 150
        header = {
            icon = { source: (CARROT) }
            title = { text: "Nutrition" }
        }
    }

    pub StepsBox = <Box> {
        draw_bg: {
            color: #ff9a62
        }
        height: 100
        header = {
            icon = { source: (SNEAKER) }
            title = { text: "Steps" }
        }

        <SectionRight> {
            show_bg: true
            spacing: 0
            align: {x: 0.0, y: 1.0}
            <Label> {
                text: "7.500 / "
                draw_text: {
                    color: #2a,
                    text_style: {
                        font_size: 15.0
                    }
                }
            }
            <Label> {
                text: "15k goal"
                draw_text: {
                    color: #2a,
                    text_style: {
                        font_size: 10.0
                    }
                }
            }
        }
    }

    pub HabitsBox = <Box> {
        draw_bg: {
            color: #94dbfb
        }
        height: 100
        header = {
            icon = { source: (BOTTLE) }
            title = { text: "Habit tracker" }
        }
    }

    pub WorkoutBox = <Box> {
        draw_bg: {
            color: #b6f36a
        }
        height: 150
        header = {
            icon = { source: (DUMBELL) }
            title = { text: "Workout" }
        }

        <SectionRight> {
            spacing: 5
            height: Fill
            daily_workout = <RoundedView> {
                cursor: Hand
                width: Fit, height: Fill
                align: {x: 0.5, y: 0.5}
                padding: {left: 10, right: 10, top: 5, bottom: 5}
                show_bg: true
                draw_bg: {
                    border_width: 1.0
                    border_color: #2a
                }
                <Label> {
                    text: "Daily\nWorkout"
                    draw_text: {
                        color: #2a
                        text_style: <TextBold> {
                            font_size: 9.0
                        }
                    }
                }
            }

            abs_routine = <RoundedView> {
                cursor: Hand
                width: Fit, height: Fill
                align: {x: 0.5, y: 0.5}
                padding: {left: 10, right: 10, top: 5, bottom: 5}
                show_bg: true
                draw_bg: {
                    color: (COLOR_BG)
                }
                <Label> {
                    text: "Abs\nRoutine"
                    draw_text: {
                        color: #f
                        text_style: <TextBold> {
                            font_size: 9.0
                        }
                    }
                }
            }
        }
    }

    pub Store = <SectionDown> {
        margin: {top: 30}
        spacing: 10
        width: Fill, height: Fit

        <Label> {
            text: "LATEST IN OUR STORE", 
            draw_text: {
                color: #d
                text_style: {
                    font_size: 9.0
                }
            } 
        }
    
        store_section = <RoundedView> {
            cursor: Hand
            draw_bg: { 
                color: #333645 
                radius: 5
            }
            height: 100
            align: {x: 0.0, y: 0.5}
            padding: {left: 20, right: 20}
            flow: Right, spacing: 15
            img = <Image> {
                source: (SLEEP_IMG)
                width: 70, height: 70
            }
            description = <SectionDown> {
                height: Fit
                align: {x: 0.0, y: 0.5}
                spacing: 4
                <Label> {
                    text: "Improve your sleep"
                    draw_text: {
                        color: #f
                        text_style: <TextBold> {
                            font_size: 12.0
                        }
                    }
                }
    
                subtitle = <Label> {
                    text: "Melatonin tablets now available"
                    draw_text: {
                        text_style: {
                            font_size: 9.0
                        }
                    }
                }
            }
        }
    }

    pub Home = <SectionDown> {
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                // Gradient to make it slightly darker at the very bottom
                // Use smoothstep for a smoother gradient transition
                let result = smoothstep(0.8, 1.0, self.pos.y);
                
                // Mix the colors based on the interpolated value 'result'
                return mix(COLOR_BG, COLOR_BG_DARKER, result);
            }
        }
        
        padding: {top: 35, left: 20, right: 20, bottom: 25}
        flow: Down
        spacing: 5.0

        <HomeHeader> {}

        pills = <SectionRight> {
            height: Fit, width: Fill
            align: {x: 0.5, y: 0.5}
            padding: 10
            spacing: 20
            <Pill> { lbl = { text: "Daily", draw_text: { color: #f } }, draw_bg: { color: #759cff } }
            <Pill> { lbl = { text: "Weekly" } }
            <Pill> { lbl = { text: "Monthly" } }
            <Pill> { lbl = { text: "Yearly" } }
        }

        feed = <SectionDown> {
            padding: 10
            spacing: 8.
            margin: {top: 10}
            height: Fit
            show_bg: true

            <SleepBox> {}
            <SectionRight> {
                spacing: 8.
                height: Fit, width: Fill
                <SectionDown> {
                    width: Fill, height: Fit
                    spacing: 8.
                    <NutritionBox> {}
                    <StepsBox> {}
                }

                <SectionDown> {
                    width: Fill, height: Fit
                    spacing: 8.
                    <HabitsBox> {}
                    workout_box = <WorkoutBox> {}
                }
            }
        }

        <Store> {}
    }
}
