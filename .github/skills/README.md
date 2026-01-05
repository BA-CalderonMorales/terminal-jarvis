# Agent Skills for Terminal Jarvis

This directory contains reusable AI agent skills that enhance the capabilities of GitHub Copilot, Claude, and other AI coding assistants when working on Terminal Jarvis.

## Available Skills

| Skill | Description | When to Use |
|-------|-------------|-------------|
| [verification](verification/) | Quality verification feedback loop | Before commits, after changes |
| [deployment](deployment/) | Deployment workflows and CI/CD | Releasing new versions |
| [release-checklist](release-checklist/) | Pre-release automation and hardening | Before tagging versions |
| [versioning](versioning/) | Version management across platforms | Updating version numbers |
| [testing](testing/) | Test-driven development practices | Bug fixes, new features |
| [refactoring](refactoring/) | Code refactoring patterns | Large file extraction |
| [database](database/) | Database architecture patterns | Schema, queries, migrations |
| [tool-config](tool-config/) | AI tool configuration | Adding new tools |
| [homebrew](homebrew/) | Homebrew distribution | macOS/Linux releases |
| [npm](npm/) | NPM distribution | NPM publishing |
| [code-quality](code-quality/) | Code quality standards | All code changes |
| [git-workflow](git-workflow/) | Branching and merge strategy | All git operations |
| [token-budget](token-budget/) | Token efficiency patterns | Long AI sessions |

## How Skills Work

1. **AI agents load skills on-demand** - Only relevant skills are loaded into context
2. **Each skill is self-contained** - Contains all instructions needed for that domain
3. **Skills are version-controlled** - Changes are tracked with the repository
4. **Skills are portable** - Can be adapted for other projects

## Skill Structure

Each skill directory contains:
```
.github/skills/<skill-name>/
  SKILL.md           # Required: Skill definition and instructions
  examples/          # Optional: Example files or snippets
  scripts/           # Optional: Associated scripts
```

## Using Skills

AI agents automatically discover and use these skills when working on relevant tasks. You can also explicitly reference a skill:

```
"Use the deployment skill to release v0.0.71"
"Apply the testing skill for this bug fix"
```

## Adding New Skills

1. Create a new directory: `.github/skills/<skill-name>/`
2. Add a `SKILL.md` file with name, description, and instructions
3. Include examples or scripts if helpful
4. Update this README with the new skill
