# 引用与链接

## Citation (`widgets/citation.rs`)
- 单条引用，支持点击跳转。
- API：`set_url_once(cx, url: String)`（仅首次生效）。
- 示例：
  ```rust
  let mut citation = view.citation(ids!(c1));
  citation.borrow_mut().unwrap().set_url_once(cx, "https://example.com".into());
  citation.label(ids!(title)).set_text(cx, "Example");
  ```

## CitationList (`widgets/citation_list.rs`)
- 引用列表容器，点击时打开 URL（依赖 `robius_open`）。
- 常嵌在 `StandardMessageContent` 或自定义消息内容中，为每项设置 URL/标题文本。
- 示例：在内容区嵌入
  ```rust
  item.citation_list(ids!(citations)).with_items(|list| {
      // 假定内部提供接口设置条目，或通过 Live DSL 预定义
  });
  ```
