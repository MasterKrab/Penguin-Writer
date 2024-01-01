<script lang="ts">
  import { onMount, onDestroy, afterUpdate } from "svelte";
  import * as monaco from "monaco-editor";
  import editorWorker from "monaco-editor/esm/vs/editor/editor.worker.js?worker";
  import dracula from "../themes/dracula";

  //@ts-ignore
  self.MonacoEnvironment = {
    getWorker() {
      return new editorWorker();
    },
    globalAPI: false,
  };

  monaco.editor.defineTheme(
    "dracula",
    dracula as monaco.editor.IStandaloneThemeData
  );

  let editor: monaco.editor.IStandaloneCodeEditor | null = null;

  export let value = "";

  export let fontSize = 14;
  export let readOnly = false;

  let element: HTMLElement;

  const createConfig = () => ({
    value,
    readOnly,
    language: "markdown",
    theme: "dracula",
    lineNumbers: "off",
    automaticLayout: true,
    renderLineHighlight: "none",
    minimap: {
      enabled: false,
    },
  });

  onMount(() => {
    if (!element) return;

    // @ts-ignore
    editor = monaco.editor.create(element, createConfig());

    editor.onDidChangeModelContent(() => {
      value = editor!.getValue();
    });
  });

  const resize = () => editor!.layout();

  $: if (editor && editor.getValue() !== value) editor.setValue(value);

  $: editor?.updateOptions({ fontSize, readOnly });

  onDestroy(() => editor!.dispose());
</script>

<svelte:window on:resize={resize} />

<div bind:this={element} />
