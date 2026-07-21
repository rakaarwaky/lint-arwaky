---
name: authoring-skills
description: Guides creation of effective SKILL.md files following Claude agent skill best practices. Use when the user asks to create, write, structure, or improve a Skill, SKILL.md file, or agent skill instructions.
---
# Authoring Skills

## Quick start

Create a SKILL.md with this structure:

```yaml
---
name: your-skill-name
description: What it does and when to use it. Third person. Include trigger terms.
---
```

Then add concise markdown body with instructions.

## Core rules

1. **Be concise** — Claude is already smart. Only add what it doesn't know. Challenge every token.
2. **Set degrees of freedom** — Match specificity to task fragility:
   - High freedom (text steps): multiple valid approaches, context-dependent
   - Medium freedom (parameterized scripts): preferred pattern exists, some variation OK
   - Low freedom (exact commands): fragile operations, strict sequence required
3. **Progressive disclosure** — SKILL.md is a table of contents. Link to detail files. Keep references ONE level deep.
4. **Consistent terminology** — Pick one term per concept. Never mix synonyms.

## Naming

- Gerund form preferred: `processing-pdfs`, `analyzing-data`, `managing-deployments`
- Lowercase, hyphens, numbers only. Max 64 chars.
- Avoid: `helper`, `utils`, `tools`, `data`, `files`

## Description field

- Third person: "Extracts text from PDFs" (not "I can help" or "You can use")
- Include WHAT it does + WHEN to use it + key trigger terms
- Max 1,024 chars. Be specific. Avoid "Helps with documents" or "Processes data"

## Structure patterns

### Simple skill (single file)

```
my-skill/
└── SKILL.md
```

### Complex skill (progressive disclosure)

```
my-skill/
├── SKILL.md              # Overview + links (under 500 lines)
├── reference.md          # API/method details
├── examples.md           # Input/output pairs
└── scripts/
    └── validate.py       # Executed, not loaded into context
```

### Domain-organized skill

```
my-skill/
├── SKILL.md
└── reference/
    ├── domain-a.md
    ├── domain-b.md
    └── domain-c.md
```

## Workflow pattern

For multi-step tasks, provide a checklist:

```
Task Progress:
- [ ] Step 1: [action]
- [ ] Step 2: [action]
- [ ] Step 3: [validate]
- [ ] Step 4: [execute]
- [ ] Step 5: [verify]
```

Add feedback loops: run validator → fix errors → repeat. Only proceed when validation passes.

## Content guidelines

- **No time-sensitive info** — Use "Current method" + collapsed "Old patterns" section
- **Provide defaults, not menus** — "Use pdfplumber" not "You can use pypdf, or pdfplumber, or PyMuPDF..."
- **Use forward slashes** in all paths: `scripts/helper.py` not `scripts\helper.py`
- **Examples over descriptions** — Show input/output pairs for style-dependent output
- **Conditional branching** — Guide decision points: "Creating? → Workflow A. Editing? → Workflow B"

## Scripts and code

- Handle errors explicitly in scripts (don't defer to Claude)
- Justify all constants: `TIMEOUT = 30  # HTTP requests complete within 30s`
- State intent clearly: "Run script.py" (execute) vs "See script.py" (read as reference)
- List dependencies explicitly: `pip install pypdf` before usage
- Use fully qualified MCP tool names: `ServerName:tool_name`

## Evaluation-driven development

1. Run task WITHOUT the skill → document failures
2. Build 3+ evaluation scenarios testing those gaps
3. Write minimal instructions to pass evaluations
4. Test across Haiku, Sonnet, Opus
5. Iterate based on real usage observations

## Pre-publish checklist

- [ ] Description: specific, third person, includes triggers
- [ ] Body under 500 lines; overflow in linked files
- [ ] References one level deep only
- [ ] Consistent terminology throughout
- [ ] Concrete examples (not abstract)
- [ ] Workflows have clear sequential steps
- [ ] Feedback loops for quality-critical operations
- [ ] Scripts handle errors; no magic numbers
- [ ] Dependencies listed; paths use forward slashes
- [ ] No time-sensitive content in main body
- [ ] Tested with target models and real scenarios

```

This SKILL.md is self-referential — it follows the very practices it teaches: concise, gerund-form name, specific third-person description with triggers, progressive structure, checklists, consistent terminology, and under 500 lines. Drop it into a folder named `authoring-skills/` and it's ready to use.
```
