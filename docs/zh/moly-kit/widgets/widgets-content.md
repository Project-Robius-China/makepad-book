# 消息内容与呈现组件

## MessageMarkdown (`widgets/message_markdown.rs`)
- Markdown 渲染，含代码高亮与复制按钮。
- 默认用于标准消息；可单独插入自定义 slot。
- 事件：代码复制按钮会发出 widget action，可监听 `event.actions().widget_action(ids!(copy_code_button))`。
- 示例：单独使用
  ```rust
  live_design! {
      import crate::widgets::message_markdown::*;
      Preview = <MessageMarkdown> { text: "Hello **Markdown**" }
  }
  ```

## StandardMessageContent (`widgets/standard_message_content.rs`)
- 组合文本/Markdown/附件/工具结果等的统一呈现。
- API：`set_content(cx, &MessageContent)`；`set_content_with_metadata(cx, &MessageContent, &MessageMetadata)`。
- 用法：在消息行 slot 中 `as_standard_message_content().set_content(cx, ...)`。
- 示例：设置内容
  ```rust
  let content = MessageContent {
      text: "Hi there".into(),
      attachments: vec![],
      ..Default::default()
  };
  item.slot(ids!(content))
      .current()
      .as_standard_message_content()
      .set_content(cx, &content);
  ```

## MessageLoading (`widgets/message_loading.rs`)
- “正在输入/加载” 动画。
- API：`animate(cx)`；`update_animation(cx)`（内部用）。
- 示例：绘制加载态
  ```rust
  let loading = list.item(cx, index, live_id!(LoadingLine));
  loading.message_loading(ids!(content_section.loading)).animate(cx);
  ```

## MessageThinkingBlock (`widgets/message_thinking_block.rs`)
- 展示“思考/分析中”。
- API：`set_content(cx, &MessageContent)`；`update_animation(cx)`。
- 示例：
  ```rust
  thinking_block.set_content(cx, &MessageContent {
      text: "Reasoning...".into(),
      ..Default::default()
  });
  thinking_block.update_animation(cx);
  ```

## ChatLine (`widgets/chat_line.rs`)
- 单条消息行布局，触发动作 `ChatLineAction::{Copy,Delete,Edit,EditRegenerate,ToolApprove,ToolDeny}`；被 `Messages` 复用。
- 示例：监听动作
  ```rust
  for action in item.filter_actions(event.actions()) {
      match action.cast() {
          ChatLineAction::Copy => { /* handle copy */ }
          ChatLineAction::Delete => { /* handle delete */ }
          _ => {}
      }
  }
  ```
