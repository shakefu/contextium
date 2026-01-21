# Adversarial-Spec

> Multi-model specification refinement through structured debate

## Overview

Adversarial-spec is a Claude Code plugin that iteratively refines product specifications by orchestrating debates between multiple LLMs until all models reach consensus. The core insight is that multiple AI models debating a specification will identify gaps and challenge assumptions more effectively than any single model working alone.

## Purpose

Specifications often have blind spots that single-model generation misses. Adversarial-spec solves this by:

- **Multi-model debate**: Different models critique from different perspectives
- **Consensus requirement**: Refinement continues until agreement
- **Claude participation**: Claude actively debates, not just orchestrates
- **Structured output**: PRD or technical specification formats

## Key Features

| Feature | Description |
|---------|-------------|
| **Debate mechanism** | Parallel critiques, synthesis, revision cycles |
| **Interview mode** | In-depth requirement gathering before debates |
| **Early agreement verification** | Skeptical review of quick consensus |
| **User review period** | Accept, modify, or continue iteration |
| **Session management** | Save, resume, and checkpoint debates |
| **Cost tracking** | Token usage and cost estimates per round |

## Document Types

### PRD (Product Requirements Document)
For stakeholders and product managers:
- Executive summary
- Problem statements
- User personas
- Functional requirements
- Success metrics

### Technical Specification
For developers and architects:
- System architecture
- API design with schemas
- Data models
- Infrastructure
- Security considerations
- Deployment strategies

## Installation

```bash
# Install plugin
claude plugin marketplace add zscole/adversarial-spec
claude plugin install adversarial-spec

# Configure API key
export OPENAI_API_KEY="sk-..."

# Or use OpenRouter for multi-provider access
export OPENROUTER_API_KEY="sk-or-..."
```

## Usage

### Basic Usage
```bash
/adversarial-spec "Build a rate limiter service with Redis backend"
```

### With Interview Mode
Conducts in-depth requirement gathering before debates:
- Problem context
- Stakeholder needs
- Functional requirements
- Technical constraints
- Success criteria

## Debate Workflow

1. **Initial spec creation** - Claude generates first draft
2. **Parallel model critiques** - Multiple models identify issues
3. **Claude synthesis** - Independent analysis + opponent evaluation
4. **Revisions** - Address identified concerns
5. **Repeat until consensus**

## Supported Models

Integrates with 10+ providers:

| Provider | Models |
|----------|--------|
| OpenAI | GPT-4o, o1 |
| Anthropic | Claude models |
| Google | Gemini |
| xAI | Grok |
| Mistral | Mistral models |
| Groq | Fast inference |
| OpenRouter | All providers via single API |
| AWS Bedrock | Enterprise gateway |

## Advanced Configuration

### Critique Focus Modes
Direct model attention to specific concerns:
- Security
- Scalability
- Performance
- UX
- Reliability
- Cost

### Model Personas
Have models adopt professional perspectives:
- Security engineer
- On-call engineer
- Junior developer
- QA engineer
- SRE
- Custom personas

### Context Injection
Include existing documents for reference:
- API specifications
- Database schemas
- Compliance requirements

### Preserve Intent Mode
Prevents unnecessary homogenization by requiring justification for removing unusual but functional design choices.

## Session Management

```bash
# Start named session
python3 debate.py critique --models gpt-4o --session my-feature-spec

# Resume session
python3 debate.py critique --resume my-feature-spec

# List sessions
python3 debate.py sessions
```

Checkpoints saved to `.adversarial-spec-checkpoints/` for rollback.

## Profiles

Save frequently-used configurations:
```bash
# Save profile
python3 debate.py save-profile strict-security \
  --models gpt-4o,gemini/gemini-2.0-flash \
  --focus security

# Use profile
python3 debate.py critique --profile strict-security < spec.md
```

## Additional Features

| Feature | Description |
|---------|-------------|
| **Diff viewer** | Compare spec versions between rounds |
| **Task extraction** | Export actionable tasks as structured JSON |
| **Telegram integration** | Remote notifications and feedback |
| **Local LLM support** | OpenAI-compatible endpoints |

## Integration with Contextium

Adversarial-spec provides specification refinement:

- Multi-model validation of project requirements
- Consensus-based feature specifications
- Structured debate for architectural decisions
- Quality assurance through model diversity

## Requirements

- Python 3.10+
- litellm package
- At least one LLM provider API key

## Technical Details

- **License**: MIT
- **Repository**: [zscole/adversarial-spec](https://github.com/zscole/adversarial-spec)

## Links

- [GitHub Repository](https://github.com/zscole/adversarial-spec)
