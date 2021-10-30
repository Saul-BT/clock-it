use druid::im::Vector;
use druid::lens;
use druid::widget::{
    Button, Container, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Scroll, TextBox,
};
use druid::Env;
use druid::EventCtx;
use druid::LensExt;
use druid::{AppLauncher, Color, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
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
    timestamp: u32,
    state: TaskState,
}

impl Task {
    pub fn new(description: &str, duration: usize) -> Self {
        let time_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The clock on your computer is broken.");

        Self {
            description: description.to_string(),
            duration,
            timestamp: time_since_epoch.as_millis() as u32,
            state: TaskState::Stopped,
        }
    }
}

#[derive(Debug, Clone, Default, Lens, Data)]
struct AppState {
    tasks: Vector<Task>,
    task_description: String,
    task_duration: String,
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
    pub fn start_task(&mut self, starting_task: &Task) -> &mut Self {
        for task in self.tasks.iter_mut() {
            if task == starting_task {
                task.state = TaskState::InProgress {
                    duration: task.duration,
                }
            }
        }
        self
    }
    pub fn pause_task(&mut self, stopping_task: &Task) -> &mut Self {
        for task in self.tasks.iter_mut() {
            if task == stopping_task {
                task.state = TaskState::Stopped
            }
        }
        self
    }

    // Add task
    fn add_task_handler(_ctx: &mut EventCtx, state: &mut AppState, _env: &Env) {
        state.add_task(&Task::new(
            &state.task_description,
            state.task_duration.parse().unwrap(),
        ));
    }

    // Handle the action of the button state change of a task
    fn action_task_handler(_ctx: &mut EventCtx, (state, task): &mut (AppState, Task), _env: &Env) {
        match task.state {
            TaskState::InProgress { duration: _ } => {
                state.pause_task(&task);
            }
            _ => {
                state.start_task(&task);
            }
        }
    }

    // Handle the state button label of a task
    fn button_label_task_handler((_state, task): &(AppState, Task), _env: &Env) -> String {
        match task.state {
            TaskState::InProgress { duration: _ } => "Pause".to_string(),
            TaskState::Completed => "Restart".to_string(),
            _ => "Start".to_string(),
        }
    }

    // Delete a task
    fn delete_task_handler(_ctx: &mut EventCtx, (state, task): &mut (AppState, Task), _env: &Env) {
        state.remove_task(task);
    }

    // Handle the duration labe of a task
    fn description_task_handler((_state, task): &(AppState, Task), _env: &Env) -> String {
        format!("[{}]", task.duration)
    }

    // Handle the description labe of a task
    fn duration_task_handler((_state, task): &(AppState, Task), _env: &Env) -> String {
        match task.state {
            TaskState::InProgress { duration: _ } => format!("-> {}", task.description),
            _ => format!("     {}", task.description),
        }
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
        .with_child(Button::new("Add task").on_click(AppState::add_task_handler))
}

fn build_task_ui() -> impl Widget<(AppState, Task)> {
    Container::new(
        Flex::row()
            .must_fill_main_axis(true)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
            .with_child(Label::new(AppState::duration_task_handler))
            .with_child(Label::new(AppState::description_task_handler))
            .with_child(
                Button::new(AppState::button_label_task_handler)
                    .on_click(AppState::action_task_handler),
            )
            .with_child(Button::new("Delete").on_click(AppState::delete_task_handler))
            .padding(10.0),
    )
    .background(Color::rgb(0.3, 0.3, 0.3))
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(AppState::default())?;
    Ok(())
}
