import type Tab from "types/tab";

const createDefaultTab = (value= "", name = "Untitled.md"): Tab => ({
  name,
  path: null,
  value,
});

export default createDefaultTab;
