# 附件与媒体组件

## AttachmentList / DenseAttachmentList (`widgets/attachment_list.rs`)
- 展示并管理附件集合（文件/图片）。
- 用法：`PromptInput` 已内置；自定义时放置 `<AttachmentList>`。
- API：`AttachmentListRef::read()/write()` 访问内部 `attachments: Vec<Attachment>`。
- 示例：添加附件
  ```rust
  let mut list = view.attachment_list(ids!(attachments));
  list.write().attachments.push(Attachment::from_text("note.txt", "hi").unwrap());
  ```

## AttachmentView (`widgets/attachment_view.rs`)
- 单附件卡片。
- API：`set_attachment(cx, Attachment)`；`get_attachment()`；`get_texture()`；工具 `can_preview(&Attachment)`。
- 示例：显示卡片
  ```rust
  attachment_view.set_attachment(cx, attachment.clone());
  if AttachmentView::can_preview(&attachment) {
      // 显示“预览”按钮
  }
  ```

## AttachmentViewerModal (`widgets/attachment_viewer_modal.rs`)
- 模态预览附件。
- API：`open(cx, Attachment)`；`close(cx)`。
- 示例：
  ```rust
  if AttachmentView::can_preview(&attachment) {
      viewer_modal.open(cx, attachment);
  }
  ```

## ImageView (`widgets/image_view.rs`)
- 图片加载/呈现。
- API：`load_png(cx, &[u8])`；`load_jpeg(cx, &[u8])`；`load_with_contet_type(cx, &[u8], content_type)`；`set_texture(cx, Option<Texture>)`；`get_texture()`；工具 `can_load(content_type)`。
- 示例：从字节加载
  ```rust
  if ImageView::can_load("image/png") {
      image_view.load_png(cx, &png_bytes)?;
  }
  ```

> 小贴士：用 `AttachmentView::can_preview` 判断是否可预览，可预览则配合 `AttachmentViewerModal` 打开。
