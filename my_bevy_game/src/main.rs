use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // リソースの初期化
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));

        // システムの登録
        app.add_systems(Startup, (setup_hello, add_people));

        // .chain() を使って実行順序を保証しつつ、タイマー更新を独立させています
        app.add_systems(Update, (tick_timer, (update_people, greet_people).chain()));
    }
}

// 起動時に一度だけ実行
fn setup_hello() {
    println!("--- ゲームを開始します ---");
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Dr.STORN".to_string())));
    commands.spawn((Person, Name("Ranma 1/2".to_string())));
    commands.spawn((Person, Name("SAO".to_string())));
}

// タイマーを進める専用のシステム
fn tick_timer(time: Res<Time>, mut timer: ResMut<GreetTimer>) {
    timer.0.tick(time.delta());
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "SAO" {
            name.0 = "Sword Art Online".to_string();
            // 見つかったらループを抜ける（効率化）
            break;
        }
    }
}

fn greet_people(timer: Res<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
