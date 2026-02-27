# 核心对话组件

> 先决条件：入口调用 `moly_kit::widgets::live_design(cx);` 以加载主题与 Live DSL。`Chat` 需启用 `async-rt` 特性，实时语音需 `realtime` 特性（非 wasm）。

## Chat (`widgets/chat.rs`)
- 定位：一站式聊天容器，内含消息区、输入框、实时语音模态。
- 用法：Live DSL 放置 `<Chat>`；运行时 `set_chat_controller(cx, Some(Arc<Mutex<ChatController>>))` 绑定 controller。
- API：`set_chat_controller(...)`，`chat_controller()`，`prompt_input_ref()`，`messages_ref()`，`is_streaming()`；`ChatRef::read()/write()`。
- 交互：处理提交/停止、工具审批/拒绝、能力变更、实时语音升级（`Upgrade::Realtime`）。
- 示例：
  ```rust
  // 入口初始化
  moly_kit::widgets::live_design(cx);

  // 创建并绑定 ChatController
  let controller = Arc::new(Mutex::new(ChatController::new(/* ... */)));
  let mut chat = view.chat(ids!(chat));
  chat.write().set_chat_controller(cx, Some(controller.clone()));
  ```

## Messages (`widgets/messages.rs`)
- 定位：PortalList 消息列表，支持复制/删除/编辑/重生成、工具审批/拒绝、自动滚动。
- 用法：`messages.write().chat_controller = Some(controller.clone());`
- API：`is_at_bottom()`，`instant_scroll_to_bottom(cx)`，`animated_scroll_to_bottom(cx)`；`set_message_editor_visibility(idx, visible)`；`current_editor_text()/index()`；`register_custom_content<T: CustomContent>`；`MessagesRef::read()/write()`。
- 事件：`MessagesAction::{Copy,Delete,EditSave,EditRegenerate,ToolApprove,ToolDeny}`。
- 示例：处理列表动作
  ```rust
  if let Event::Actions(actions) = event {
      for action in actions {
          if let Some(msg_action) = action.as_widget_action().and_then(|wa| wa.cast::<MessagesAction>()) {
              match msg_action {
                  MessagesAction::Copy(idx) => { /* copy to clipboard */ }
                  MessagesAction::Delete(idx) => controller.lock().unwrap()
                      .dispatch_mutation(VecMutation::<Message>::RemoveOne(idx)),
                  MessagesAction::EditRegenerate(_) => { /* 自行实现 */ }
                  _ => {}
              }
          }
      }
  }
  ```

## PromptInput (`widgets/prompt_input.rs`)
- 定位：输入框 + 附件 + 模型选择器 + 发送/停止切换。
- 用法：Live DSL `<PromptInput>`；运行时 `set_chat_controller(...)` 以驱动模型选择与能力；根据 bot 能力自动显示附件/音频按钮。
- API：`submitted(actions)`，`call_pressed(actions)`；`has_send_task()/has_stop_task()`，`set_send()/set_stop()`；`enable()/disable()`；`reset(cx)`；`set_bot_capabilities(cx, Option<BotCapabilities>)`；`PromptInputRef::read()/write()`。
- 示例：响应提交
  ```rust
  let mut input = view.prompt_input(ids!(prompt));
  if input.read().submitted(event.actions()) && input.read().has_send_task() {
      let text = input.text();
      controller.lock().unwrap().dispatch_mutation(VecMutation::Push(Message {
          from: EntityId::User,
          content: MessageContent { text, ..Default::default() },
          ..Default::default()
      }));
      input.write().reset(cx);
      controller.lock().unwrap().dispatch_task(ChatTask::Send);
  }
  ```
