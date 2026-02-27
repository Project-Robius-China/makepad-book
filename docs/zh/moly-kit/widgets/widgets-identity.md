# 身份与头像

## Avatar (`widgets/avatar.rs`)
- 在消息行展示用户/机器人/工具头像。
- 数据：`avatar: Option<EntityAvatar>`（`Text(grapheme)` 或 `Image(path)`）。
- 行为：根据类型自动切换字母块或图片；直接设置字段后重绘即可。
- 示例：
  ```rust
  let mut avatar = widget.avatar(ids!(avatar)).borrow_mut().unwrap();
  avatar.avatar = Some(EntityAvatar::Text("B".into()));
  ```
- 另一示例：使用图片路径
  ```rust
  avatar.avatar = Some(EntityAvatar::Image("res/avatar.png".into()));
  ```
