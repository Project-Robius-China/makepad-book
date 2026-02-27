# 模型选择组件

## ModelSelector (`widgets/model_selector.rs`)
- 模型选择入口（按钮/下拉），与 `ChatController` 同步。
- API：`set_chat_controller(Option<Arc<Mutex<ChatController>>>)`；`set_grouping(Option<GroupingFn>)` 自定义分组。
- 示例：
  ```rust
  let controller = Arc::new(Mutex::new(ChatController::new(...)));
  selector.borrow_mut().unwrap().set_chat_controller(Some(controller.clone()));
  ```

## ModelSelectorList (`widgets/model_selector_list.rs`)
- 模型列表 + 搜索/分组。
- API：`get_height()`；`set_search_filter(cx, filter)`；`clear_search_filter(cx)`；`set_chat_controller(...)`；`set_grouping(...)`；`set_selected_bot_id(...)`。
- 示例：
  ```rust
  list.borrow_mut().unwrap().set_search_filter(cx, "gpt");
  list.borrow_mut().unwrap().set_selected_bot_id(Some(bot_id.clone()));
  ```

## ModelSelectorItem (`widgets/model_selector_item.rs`)
- 单个模型项。
- API：`set_bot(bot)`；`set_selected_bot_id(...)` 控制选中态。
- 示例：
  ```rust
  item.borrow_mut().unwrap().set_bot(bot.clone());
  item.borrow_mut().unwrap().set_selected_bot_id(Some(bot.id.clone()));
  ```

> 典型：`PromptInput` 已包含 `ModelSelector`。若独立使用，创建后设 controller；如需定制分组/搜索，直接操作 `ModelSelectorList`。
