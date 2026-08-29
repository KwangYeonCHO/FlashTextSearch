<template>
  <div class="h-full w-full relative flex flex-col bg-[#0b101b] overflow-hidden">
    <!-- Monaco 容器 -->
    <div ref="editorContainer" class="h-full w-full"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from "vue";
import * as monaco from "monaco-editor";

const props = defineProps<{
  content: string;
  filePath: string;
  targetLine?: number;
  matchStart?: number;
  matchEnd?: number;
}>();

const editorContainer = ref<HTMLElement | null>(null);
let editorInstance: monaco.editor.IStandaloneCodeEditor | null = null;
let currentDecorations: string[] = [];

// 根据后缀映射 Monaco 语言标识
const detectLanguage = (path: string): string => {
  const ext = path.split(".").pop()?.toLowerCase() || "";
  const map: Record<string, string> = {
    txt: "plaintext",
    log: "plaintext",
    md: "markdown",
    markdown: "markdown",
    json: "json",
    xml: "xml",
    yaml: "yaml",
    yml: "yaml",
    toml: "toml",
    ini: "ini",
    csv: "plaintext",
    rs: "rust",
    py: "python",
    js: "javascript",
    ts: "typescript",
    jsx: "javascript",
    tsx: "typescript",
    vue: "html",
    html: "html",
    htm: "html",
    css: "css",
    scss: "scss",
    less: "less",
    sql: "sql",
    c: "c",
    cpp: "cpp",
    h: "cpp",
    hpp: "cpp",
    cs: "csharp",
    java: "java",
    go: "go",
    php: "php",
    rb: "ruby",
    sh: "shell",
    bat: "bat",
    ps1: "powershell",
    lua: "lua",
  };
  return map[ext] || "plaintext";
};

// 初始化 Monaco
onMounted(() => {
  if (!editorContainer.value) return;

  editorInstance = monaco.editor.create(editorContainer.value, {
    value: props.content,
    language: detectLanguage(props.filePath),
    theme: "vs-dark",
    readOnly: true,
    automaticLayout: true,
    fontSize: 13,
    fontFamily: "Fira Code, Consolas, Monaco, monospace",
    lineNumbers: "on",
    minimap: { enabled: true },
    scrollBeyondLastLine: false,
    renderLineHighlight: "all",
    contextmenu: true,
    wordWrap: "off",
    smoothScrolling: true,
    cursorBlinking: "smooth",
  });

  // 如果初始就有目标行，执行跳转高亮
  if (props.targetLine) {
    jumpToTarget(props.targetLine, props.matchStart, props.matchEnd);
  }
});

// 跳转到目标行并高亮
const jumpToTarget = (line: number, startChar?: number, endChar?: number) => {
  if (!editorInstance) return;

  const model = editorInstance.getModel();
  if (!model) return;

  const totalLines = model.getLineCount();
  const safeLine = Math.min(Math.max(1, line), totalLines);

  // 平滑居中滚动到目标行
  editorInstance.revealLineInCenter(safeLine, monaco.editor.ScrollType.Smooth);
  editorInstance.setPosition({ lineNumber: safeLine, column: (startChar || 0) + 1 });

  // 构建高亮装饰
  const newDecorations: monaco.editor.IModelDeltaDecoration[] = [
    // 整行高亮装饰
    {
      range: new monaco.Range(safeLine, 1, safeLine, 1),
      options: {
        isWholeLine: true,
        className: "monaco-active-line-decoration",
        linesDecorationsClassName: "bg-amber-500 w-1",
      },
    },
  ];

  // 如果有精确字符范围，添加关键字高亮装饰
  if (startChar !== undefined && endChar !== undefined && endChar > startChar) {
    newDecorations.push({
      range: new monaco.Range(safeLine, startChar + 1, safeLine, endChar + 1),
      options: {
        inlineClassName: "monaco-active-match-decoration",
      },
    });
  }

  // 更新 Monaco 装饰
  currentDecorations = editorInstance.deltaDecorations(currentDecorations, newDecorations);
};

// 监听文件内容与路径变化
watch(
  () => [props.content, props.filePath],
  ([newContent, newPath]) => {
    if (!editorInstance) return;

    const language = detectLanguage(newPath as string);
    const model = monaco.editor.createModel(newContent as string, language);
    editorInstance.setModel(model);

    if (props.targetLine) {
      setTimeout(() => {
        jumpToTarget(props.targetLine!, props.matchStart, props.matchEnd);
      }, 50);
    }
  }
);

// 监听行号或关键词范围变更（如点击 Next / Prev 按钮）
watch(
  () => [props.targetLine, props.matchStart, props.matchEnd],
  ([newLine, newStart, newEnd]) => {
    if (newLine) {
      jumpToTarget(newLine as number, newStart as number, newEnd as number);
    }
  }
);

onBeforeUnmount(() => {
  if (editorInstance) {
    editorInstance.dispose();
  }
});
</script>
