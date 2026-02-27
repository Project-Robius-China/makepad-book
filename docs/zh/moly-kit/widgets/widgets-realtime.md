# 实时语音组件

## Realtime (`widgets/realtime.rs`)
- 语音实时对话 UI，通常由 `Chat` 在收到 `Upgrade::Realtime` 时弹出。
- API：`set_realtime_channel(channel: RealtimeChannel)`；`set_bot_entity_id(cx, EntityId)`；`set_chat_controller(Option<Arc<Mutex<ChatController>>>)`；`connection_requested()`；`take_conversation_messages()`；`reset_state(cx)`。
- 流程：controller 升级 -> Chat 将 channel/实体传给 `Realtime` -> 通过 `MolyModal` 打开 -> 结束后用 `take_conversation_messages` 取回文本并 `reset_state`。
- 示例：在升级时处理
  ```rust
  match upgrade {
      Upgrade::Realtime(channel) => {
          let mut realtime = chat.realtime(ids!(realtime));
          realtime.set_bot_entity_id(cx, EntityId::Bot(bot_id.clone()));
          realtime.set_realtime_channel(channel.clone());
          chat.moly_modal(ids!(audio_modal)).open_as_dialog(cx);
          None
      }
      _ => Some(upgrade),
  }
  ```
