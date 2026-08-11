/**
 * Minimal example module for the libraries-ai-starter TypeScript reference
 * path. Replace this with your actual project code once you've promoted
 * this example into src/ (see TEMPLATE_SETUP.md at the repository root).
 */
export function greet(name: string): string {
  if (!name) {
    throw new Error("name must not be empty");
  }
  return `Hello, ${name}!`;
}
