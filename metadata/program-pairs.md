# Program Pairs

## Finding Program Pairs

- Initial Discovery: Manually identify several high-quality program pairs and create their metadata entries.
  - A significant use case for Rust is rewriting and improving existing CLI tools. I searched for Rust rewrites of existing C command line tools.
- AI-Assisted Generation: Use the manually curated examples as prompts for LLMs (like ChatGPT) to suggest additional program pairs.
  - First, manually format the program pairs into the format of the metadata schema.
  - Then, paste it into an LLM with the following prompt: "I am compiling a list of C to Rust program pairs, help me add to my list"
- Manual Verification: Review and validate all AI-generated suggestions to ensure:
  - The programs actually exist and are maintained
  - The Rust version is genuinely a rewrite/improvement of the C version
  - The functionality and scope are comparable
  - Metadata accuracy and completeness

### Determining C Source Files

- Check the Makefile for the `c` files needed for the executable.
- Check each `c` file for the header files it uses.

## Rejected Program Pairs

(Accepted program pairs appear in the metadata files in this repository.)
