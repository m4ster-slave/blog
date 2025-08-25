# Backend Documentation

## Endpoints

### GET /posts

- return all the posts title with metadata and description

```json
[
  {
    "title": "This is the title of a post",
    "date": "2025-08-22T10:00:00Z",
    "summary": "This is the summary of a blog post with metadata",
    "slug": "this-is-the-title-of-a-post"
  },
  {
    "title": "Another post",
    "date": "2025-08-22T10:00:00Z",
    "summary": "Another post summary",
    "slug": "another-post"
  }
]
```

### GET /posts/:slug

- return metadata and post content of a single post

```json
{
  "title": "This is the title of a post",
  "date": "2025-08-22T10:00:00Z",
  "summary": "This is the summary of a blog post with metadata",
  "content": "# Title\n- hello"
}
```
