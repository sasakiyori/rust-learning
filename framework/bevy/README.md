# Bevy

## rust相关

切换rustup toolchain:

```shell
rustup default nightly
rustup default stable
```

## 开发过程中遇到的一些问题

1.startup system按顺序做？  
<https://github.com/bevyengine/bevy/discussions/8335>
<https://docs.rs/bevy/latest/bevy/app/enum.StartupSet.html>
<https://www.reddit.com/r/bevy/comments/xv1hog/startup_systems_not_working_as_expected/>

2.为什么单实例despawn后再spawn，对应的texture不显示了？  
<https://github.com/bevyengine/bevy/discussions/8288>

3.按最快速度通过entity查找到对应实体，而不是通过大query for循环？  
<https://github.com/bevyengine/bevy/discussions/1205>

## 参考

- cheat book(0.9.0): <https://bevy-cheatbook.github.io/introduction.html>
- cheat book中文版(0.6.0): <https://yiviv.com/bevy-cheatbook/introduction.html>
- bevy-ball-game: <https://github.com/frederickjjoubert/bevy-ball-game>
