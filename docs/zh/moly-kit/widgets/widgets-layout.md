# 布局/模态/主题

## MolyModal (`widgets/moly_modal.rs`)
- 统一模态容器，支持 dialog/popup，遮罩可关闭。
- API（Widget/Ref 同名）：`open()`，`open_as_dialog()`，`open_as_popup(cx, pos)`，`close()`；`dismissed(actions)`；`is_open()`。
- 示例：
  ```rust
  let modal = view.moly_modal(ids!(settings_modal));
  modal.open_as_dialog(cx);
  if modal.dismissed(event.actions()) { /* 关闭后逻辑 */ }
  ```

## Slot (`widgets/slot.rs`)
- 占位/替换容器。
- API（Widget/Ref 同名）：`replace(widget)`，`restore()`，`current()`，`default()`。
- 示例：
  ```rust
  let mut slot = item.slot(ids!(content));
  slot.replace(custom_widget);
  // 恢复默认
  slot.restore();
  ```

## Theme (`widgets/theme_moly_kit_light.rs`)
- 默认浅色主题，随 `widgets::live_design(cx)` 自动链接，无需手动引入。
