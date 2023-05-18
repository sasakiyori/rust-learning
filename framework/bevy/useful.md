# useful bevy functions

## edit schedule settings (log level)

```rust
fn main() {
    App::new()
        .edit_schedule(Main, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Info,
                ..default()
            });
        })
        .add_plugins(DefaultPlugins)
        .run();
}
```
