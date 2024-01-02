<script lang="ts">
  import type Tab from "types/tab";

  import { open, save } from "@tauri-apps/api/dialog";
  import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";

  import FloatingFocus from "./components/FloatingFocus.svelte";
  import TopBar from "./components/TopBar.svelte";
  import TabBar from "./components/TabBar.svelte";
  import WindowControls from "./components/WindowControls.svelte";
  import Editor from "./components/Editor.svelte";
  import Markdown from "./components/Markdown.svelte";
  import Divider from "./components/Divider.svelte";

  import createDefaultTab from "./utils/createDefaultTab";
  import getFilename from "./utils/getFilename";
  import removeIndexElement from "./utils/removeIndexElement";
  import { getTabs, saveTabs } from "./lib/tabs";

  import welcomeText from "./assets/welcome";

  import "@fontsource/roboto";
  import "./styles/global.css";
  import { onDestroy, onMount } from "svelte";

  const savedTabs = getTabs();

  let tabs: Tab[] = savedTabs.length ? savedTabs : [createDefaultTab(welcomeText, "welcome")];
  let selectedTab: Tab = tabs[0];
  let selectedTabIndex = 0;

  let isGrabbing: boolean;
  let leftWidth: number = 50;
  let container: HTMLElement;

  $: saveTabs(tabs);

  $: selectedTab = tabs[selectedTabIndex];

  const selectLastTab = () => {
    selectedTabIndex = tabs.length - 1;
  };

  const handleNewFile = () => {
    tabs = [...tabs, createDefaultTab()];
    selectLastTab();
  };

  const handleSelectFile = async () => {
    const path = (await open({
      title: "Select file",
      multiple: false,
    })) as string;

    const alreadyOpenFileIndex = tabs.findIndex((tab) => tab.path === path);

    if (alreadyOpenFileIndex > 0) {
      selectedTabIndex = alreadyOpenFileIndex;
      return;
    }

    const value = await readTextFile(path);

    tabs = [...tabs, { name: getFilename(path)!, path, value }];
    selectLastTab();
  };

  const saveFile = async (isAutoSave = false) => {
    const { path, value } = selectedTab;

    if (path) return writeTextFile(path!, value!);

    if (isAutoSave) return;

    const newPath = await save({
      title: "Save file",
      defaultPath: "Untitled.md",
      filters: [{ name: "Markdown", extensions: ["md"] }],
    });

    if (!newPath) return;

    await writeTextFile(newPath!, value!);

    tabs[selectedTabIndex] = {
      ...tabs[selectedTabIndex],
      path: newPath,
      name: getFilename(newPath)!,
    };
  };

  const handleSaveFile = () => saveFile();

  const handleSelectTab = (event: CustomEvent<{ index: number }>) => {
    selectedTabIndex = event.detail.index;
  };

  const handleCloseTab = (event: CustomEvent<{ index: number }>) => {
    if (tabs.length == 1) {
      tabs = [createDefaultTab(welcomeText, "welcome")];
      return;
    }

    tabs = removeIndexElement(tabs, event.detail.index);
    selectLastTab();
  };

  let interval: number;

  onMount(async () => {
    interval = setInterval(() => saveFile(true), 0);
  });

  onDestroy(() => {
    clearInterval(interval);
  });
</script>

<FloatingFocus />

<header class="header">
  <TopBar
    on:newFile={handleNewFile}
    on:selectFile={handleSelectFile}
    on:saveFile={handleSaveFile}
  />
  <WindowControls />
</header>

<main class="main">
  <TabBar
    bind:tabs
    bind:selectedTabIndex
    on:selectTab={handleSelectTab}
    on:closeTab={handleCloseTab}
    on:newFile={handleNewFile}
  />

  <div
    class="editor"
    bind:this={container}
    style={`--left-width: ${leftWidth}%; --right-width: ${100 - leftWidth}%`}
  >
    <Editor bind:value={selectedTab.value} />
    <Divider bind:isGrabbing bind:leftWidth {container} />
    <div class="output">
      <Markdown value={selectedTab.value || ""} />
    </div>
  </div>
</main>

<style lang="scss">
  :global(:root) {
    --header-height: 2.75rem;
  }

  .header {
    display: grid;
    grid-template-columns: 1fr auto;
    align-items: center;
    background-color: var(--cuaternary-color);
    height: var(--header-height);
    padding-left: 1rem;
    padding-right: 1rem;
  }

  .editor {
    display: grid;
    grid-template-columns: var(--left-width, 50%) 0.35rem calc(
        var(--right-width, 50%) - 0.35rem
      );
    grid-template-rows: calc(
      100vh - 0.75rem - var(--tab-bar-height) - var(--header-height)
    );
    padding-top: 0.75rem;
  }

  .output {
    overflow: auto;
    height: 100%;

    &::-webkit-scrollbar {
      width: 0.5rem;
      height: 0.5rem;
      background-color: var(--primary-color);
    }

    &::-webkit-scrollbar-thumb {
      background-color: var(--secondary-color);
      border-radius: 0.5rem;
    }

    &:active::-webkit-scrollbar-thumb {
      background-color: var(--tertiary-color);
    }
  }
</style>
