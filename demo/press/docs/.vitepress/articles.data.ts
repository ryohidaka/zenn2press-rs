import { createContentLoader } from "vitepress";

export default createContentLoader("articles/*.md", { includeSrc: true });
