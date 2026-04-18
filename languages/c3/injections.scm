([
  (line_comment)
  (block_comment)
] @injection.content
  (#set! injection.language "comment"))

((doc_comment_text) @injection.content
  (#set! injection.language "markdown"))
