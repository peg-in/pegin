# Using the PEGIN knowledge base with AI

The wiki is **AI-ready** when you use the markdown topic files plus the artifacts in `docs/ai/`, not the HTML wiki alone.

## Artifacts

| File | Purpose |
|------|---------|
| [`llms.txt`](../llms.txt) | Standard index for LLM tools ([llmstxt.org](https://llmstxt.org/)) |
| [`manifest.json`](manifest.json) | Machine-readable catalog (paths, summaries, categories) |
| [`chunks.jsonl`](chunks.jsonl) | Section-level chunks for RAG / vector DB ingestion |
| [`CONTEXT.md`](CONTEXT.md) | Compact system prompt (~1–2k tokens) |
| [`../README.md`](../README.md) | Human + AI navigation index |

Regenerate after doc changes:

```bash
python3 scripts/generate-ai-knowledge-base.py
```

## Cursor / Claude Code

1. **Always-on context:** `.cursor/rules/pegin-knowledge-base.mdc` points agents at canonical docs.  
2. **@ mention files:** e.g. `@docs/08-developer/specs/tech-stack.md` for implementation work.  
3. **Compact boot:** `@docs/ai/CONTEXT.md` when you need project facts without loading the full corpus.  
4. **Repo guide:** root [`AGENTS.md`](../../AGENTS.md) links here.

## RAG pipelines (LangChain, LlamaIndex, etc.)

1. Load `docs/ai/chunks.jsonl` — one JSON object per line with `text`, `path`, `section`, `category`.  
2. Embed the `text` field; store `id`, `path`, `document_id`, `section` as metadata.  
3. Filter by `category` at query time if needed (`technical`, `business`, …).  
4. Do **not** embed `wiki/PEGIN_Wiki.md` or `_archive/` (excluded by design).

Example (Python):

```python
import json

chunks = []
with open("docs/ai/chunks.jsonl") as f:
    for line in f:
        chunks.append(json.loads(line))

# Each chunk: id, path, title, section, category, text, token_estimate
```

## OpenAI / custom GPT / Assistant

- Upload topic markdown from `docs/01-vision/` … `docs/07-penguin-products/` (not the whole `PEGIN_Wiki.md` in one file — duplicates confuse retrieval).  
- Or upload `CONTEXT.md` + selected files per domain (business vs engineering).  
- Paste `llms.txt` into **Instructions** as a document map.

## Claude Projects

- Add `docs/ai/CONTEXT.md` as project knowledge.  
- Add folder(s) by role: e.g. `docs/08-developer/` for engineers, `docs/05-business/` for GTM.  
- Use `manifest.json` to see summaries before attaching files.

## API / MCP

Expose `GET` manifest or stream `chunks.jsonl` from your doc server; filter `category == "technical"` for coding agents.

## What is not AI-ready

| Asset | Why |
|-------|-----|
| `PEGIN_Wiki_Knowledge_Base.html` | HTML/JS shell; poor chunking, no stable section IDs |
| `wiki/PEGIN_Wiki.md` | 1,500+ line duplicate of all topic docs |
| `wiki/_archive/` | Superseded navigation only |

## Chunking policy

- Split on `##` headings; sub-split large sections on `###`.  
- Max ~12k characters per chunk; tiny sections merged.  
- Navigation-only files (`quick-start`, `wiki-setup`) excluded from chunks.