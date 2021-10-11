use druid::im::Vector;
use druid::lens;
use druid::widget::{
    Button, Container, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Scroll, TextBox,
};
use druid::Env;
use druid::EventCtx;
use druid::LensExt;
use druid::{AppLauncher, Color, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};

#[derive(Debug, Clone, Data, PartialEq)]
enum TaskState {
    InProgress { duration: usize },
    Completed,
    Discarded,
    Stopped,
}

#[derive(Debug, Clone, Data, Lens, PartialEq)]
struct Task {
    description: String,
    duration: usize,
    state: TaskState,
}

impl Task {
    pub fn new(description: &str, duration: usize) -> Self {
        Self {
            duration,
            state: TaskState::Stopped,
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, Lens, Data)]
struct AppState {
    tasks: Vector<Task>,
    task_description: String,
    task_duration: String,
    selected_task: usize,
}

impl AppState {
    pub fn add_task(&mut self, task: &Task) -> &mut Self {
        self.tasks.push_back(task.clone());
        self
    }
    pub fn remove_task(&mut self, task: &Task) -> &mut Self {
        self.tasks.retain(|t| t != task);
        self
    }
}

fn build_ui() -> impl Widget<AppState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(build_controls_ui())
        .with_flex_child(
            Container::new(Scroll::new(List::new(build_task_ui).with_spacing(5.0))).lens(
                lens::Identity.map(
                    |d: &AppState| (d.clone(), d.tasks.clone()),
                    |d: &mut AppState, x: (AppState, Vector<Task>)| {
                        d.tasks = x.0.tasks;
                    },
                ),
            ),
            1.0,
        )
}

fn build_controls_ui() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Flex::row()
                .with_child(Label::new("Description: "))
                .with_child(TextBox::new().lens(AppState::task_description)),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("Duration: "))
                .with_child(TextBox::new().lens(AppState::task_duration)),
        )
        .with_child(Button::new("Add task").on_click(add_task_handler))
}

fn build_task_ui() -> impl Widget<(AppState, Task)> {
    Container::new(
        Flex::row()
            .must_fill_main_axis(true)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
            .with_child(Label::new(|(_state, task): &(AppState, Task), _env: &_| {
                task.description.clone()
            }))
            .with_child(Label::new(|(_state, task): &(AppState, Task), _env: &_| {
                task.duration.to_string()
            }))
            .with_child(Button::new("Start"))
            .with_child(Button::new("Delete").on_click(
                |_, (state, task): &mut (AppState, Task), _| {
                    state.remove_task(task);
                },
            ))
            .padding(10.0),
    )
    .background(Color::rgb(0.3, 0.3, 0.3))
}

fn add_task_handler(_ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
    data.add_task(&Task::new(
        &data.task_description,
        data.task_duration.parse().unwrap(),
    ));
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(AppState::default())?;
    Ok(())
}
