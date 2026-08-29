<template>
  <div class="h-full flex flex-col glass-panel overflow-hidden" style="background-color: var(--bg-editor)">
    <!-- 预览区顶部工具栏 -->
    <div
      v-if="selectedFile"
      class="px-4 py-2.5 border-b shrink-0 flex flex-wrap items-center justify-between gap-2 shadow-xs"
      style="background-color: var(--bg-surface); border-color: var(--border-subtle)"
    >
      <!-- 文件名与详细信息 -->
      <div class="flex items-center gap-2.5 overflow-hidden flex-1 min-w-[200px]">
        <component
          :is="getFileIcon(selectedFile.extension)"
          class="w-4 h-4 shrink-0 text-sky-500"
        />
        <div class="overflow-hidden">
          <div class="flex items-center gap-2">
            <span class="text-xs font-bold truncate" style="color: var(--text-title)" :title="selectedFile.fileName">
              {{ selectedFile.fileName }}
            </span>
            <span
              v-if="textContent?.encoding"
              class="text-[10px] px-1.5 py-0.2 rounded font-mono shrink-0 border"
              style="background-color: var(--bg-card); color: var(--text-muted); border-color: var(--border-subtle)"
            >
              {{ textContent.encoding }}
            </span>
            <span
              v-if="textContent?.isTruncated"
              class="text-[10px] px-1.5 py-0.2 rounded font-medium shrink-0 border"
              style="background-color: rgba(245, 158, 11, 0.15); color: #d97706; border-color: rgba(245, 158, 11, 0.3)"
            >
              {{ t.truncatedNotice }}
            </span>
          </div>
          <div class="text-[10px] truncate" style="color: var(--text-muted)" :title="selectedFile.filePath">
            {{ selectedFile.filePath }}
          </div>
        </div>
      </div>

      <!-- 多匹配项快速跳转导航条 (Next / Prev 按键) -->
      <div
        v-if="selectedFile.matches.length > 0"
        class="flex items-center gap-1.5 px-2.5 py-1 rounded-xl border shadow-inner"
        style="background-color: var(--bg-card); border-color: var(--border-subtle)"
      >
        <span class="text-[11px] font-medium" style="color: var(--text-muted)">
          {{ t.matchIndex }}
          <span class="font-bold font-mono text-amber-500">
            {{ activeMatchIndex }}
          </span>
          / {{ selectedFile.matches.length }}
        </span>

        <!-- 上一个匹配 (Shift+F3) -->
        <button
          class="p-1 rounded disabled:opacity-30 disabled:cursor-not-allowed transition cursor-pointer hover:bg-black/10 dark:hover:bg-white/10"
          style="color: var(--text-title)"
          :disabled="selectedFile.matches.length <= 1"
          :title="t.prevMatchTip"
          @click="jumpPrevMatch"
        >
          <ChevronUp class="w-4 h-4" />
        </button>

        <!-- 下一个匹配 (F3) -->
        <button
          class="p-1 rounded disabled:opacity-30 disabled:cursor-not-allowed transition cursor-pointer hover:bg-black/10 dark:hover:bg-white/10"
          style="color: var(--text-title)"
          :disabled="selectedFile.matches.length <= 1"
          :title="t.nextMatchTip"
          @click="jumpNextMatch"
        >
          <ChevronDown class="w-4 h-4" />
        </button>
      </div>

      <!-- 外部原生操作快捷按钮 -->
      <div class="flex items-center gap-1.5 shrink-0">
        <button
          class="px-2.5 py-1 text-xs font-medium rounded-lg transition flex items-center gap-1 cursor-pointer border shadow-2xs"
          style="background-color: var(--bg-card); color: var(--text-title); border-color: var(--border-subtle)"
          :title="t.locateInExplorer"
          @click="openInExplorer"
        >
          <FolderSearch class="w-3.5 h-3.5 text-sky-500" />
          <span>{{ t.locateInExplorer }}</span>
        </button>

        <button
          class="px-2.5 py-1 text-xs font-medium rounded-lg transition flex items-center gap-1 cursor-pointer border shadow-2xs"
          style="background-color: var(--bg-card); color: var(--text-title); border-color: var(--border-subtle)"
          :title="t.openWithApp"
          @click="openWithSystemApp"
        >
          <ExternalLink class="w-3.5 h-3.5 text-emerald-500" />
          <span>{{ t.openWithApp }}</span>
        </button>
      </div>
    </div>

    <!-- 预览内容主体 -->
    <div class="flex-1 relative overflow-hidden">
      <!-- 加载中动画 -->
      <div
        v-if="isLoading"
        class="absolute inset-0 bg-[#090d16]/80 backdrop-blur-sm z-30 flex flex-col items-center justify-center text-slate-400"
      >
        <Loader2 class="w-8 h-8 animate-spin text-sky-400 mb-2" />
        <span class="text-xs font-medium">{{ t.loadingDocument }}</span>
      </div>

      <!-- 错误提示 -->
      <div
        v-else-if="errorMessage"
        class="h-full flex flex-col items-center justify-center p-8 text-center text-rose-400"
      >
        <AlertCircle class="w-10 h-10 mb-2 text-rose-500" />
        <p class="text-sm font-semibold">{{ errorMessage }}</p>
        <button
          class="mt-4 px-3 py-1 bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs rounded-lg border border-slate-600 transition"
          @click="openWithSystemApp"
        >
          {{ t.openDefaultApp }}
        </button>
      </div>

      <!-- 未选择任何文档时的占位提示 -->
      <div
        v-else-if="!selectedFile"
        class="h-full flex flex-col items-center justify-center text-center p-8 text-slate-500"
      >
        <Eye class="w-14 h-14 text-slate-700 mb-3" />
        <p class="text-sm font-medium text-slate-400">{{ t.selectFileTip }}</p>
        <p class="text-xs mt-1 text-slate-600">
          {{ t.selectFileSubTip }}
        </p>
      </div>

      <!-- Excel 表格预览器 -->
      <ExcelSheetViewer
        v-else-if="isExcelFile && excelContent"
        :workbook="excelContent"
        :target-sheet="currentMatch?.sheetName"
        :target-row="currentMatch?.lineNumber"
        :target-col="currentMatch?.columnNumber"
      />

      <!-- 纯文本与代码 Monaco 编辑器预览器 -->
      <MonacoEditorViewer
        v-else-if="textContent"
        :content="textContent.content"
        :file-path="selectedFile.filePath"
        :target-line="currentMatch?.lineNumber"
        :match-start="currentMatch?.matchStart"
        :match-end="currentMatch?.matchEnd"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  FileText,
  FileSpreadsheet,
  FileCode,
  File,
  FolderSearch,
  ExternalLink,
  ChevronUp,
  ChevronDown,
  Loader2,
  AlertCircle,
  Eye,
} from "@lucide/vue";
import { t } from "../i18n";
import MonacoEditorViewer from "./MonacoEditorViewer.vue";
import ExcelSheetViewer from "./ExcelSheetViewer.vue";
import type {
  FileMatchResult,
  MatchItem,
  TextDocumentContent,
  ExcelWorkbookContent,
} from "../types/search";

const props = defineProps<{
  selectedFile?: FileMatchResult | null;
  selectedMatch?: MatchItem | null;
}>();

const emit = defineEmits<{
  (e: "changeMatch", match: MatchItem): void;
}>();

const isLoading = ref(false);
const errorMessage = ref<string | null>(null);
const textContent = ref<TextDocumentContent | null>(null);
const excelContent = ref<ExcelWorkbookContent | null>(null);
const currentMatch = ref<MatchItem | null>(null);

// 判断是否为 Excel 表格文件
const isExcelFile = computed(() => {
  if (!props.selectedFile) return false;
  const ext = props.selectedFile.extension.toLowerCase();
  return ["xlsx", "xls", "ods", "xlsb"].includes(ext);
});

// 当前选中的匹配索引
const activeMatchIndex = computed(() => {
  if (!currentMatch.value) return 1;
  return currentMatch.value.matchIndex;
});

// 图标判断
const getFileIcon = (ext: string) => {
  if (["xlsx", "xls", "ods", "csv"].includes(ext)) return FileSpreadsheet;
  if (["rs", "py", "js", "ts", "vue", "c", "cpp", "h", "cs", "java", "go", "php", "sql", "html", "css"].includes(ext)) {
    return FileCode;
  }
  if (["txt", "log", "md", "json", "xml", "yaml", "yml", "toml"].includes(ext)) {
    return FileText;
  }
  return File;
};

// 加载文件内容
const loadDocument = async (filePath: string) => {
  isLoading.value = true;
  errorMessage.value = null;

  try {
    if (isExcelFile.value) {
      const data = await invoke<ExcelWorkbookContent>("read_excel_file", { path: filePath });
      excelContent.value = data;
      textContent.value = null;
    } else {
      const data = await invoke<TextDocumentContent>("read_text_file", { path: filePath });
      textContent.value = data;
      excelContent.value = null;
    }
  } catch (err: any) {
    console.error("加载文档失败:", err);
    errorMessage.value = typeof err === "string" ? err : err.message || "无法读取该文档内容";
  } finally {
    isLoading.value = false;
  }
};

// 下一个匹配项 (Next)
const jumpNextMatch = () => {
  if (!props.selectedFile || !props.selectedFile.matches.length) return;
  const matches = props.selectedFile.matches;
  const currentIndex = currentMatch.value ? currentMatch.value.matchIndex - 1 : 0;
  const nextIndex = (currentIndex + 1) % matches.length;
  currentMatch.value = matches[nextIndex];
  emit("changeMatch", matches[nextIndex]);
};

// 上一个匹配项 (Prev)
const jumpPrevMatch = () => {
  if (!props.selectedFile || !props.selectedFile.matches.length) return;
  const matches = props.selectedFile.matches;
  const currentIndex = currentMatch.value ? currentMatch.value.matchIndex - 1 : 0;
  const prevIndex = (currentIndex - 1 + matches.length) % matches.length;
  currentMatch.value = matches[prevIndex];
  emit("changeMatch", matches[prevIndex]);
};

// 在资源管理器中定位
const openInExplorer = async () => {
  if (!props.selectedFile) return;
  try {
    await invoke("open_in_file_manager", { path: props.selectedFile.filePath });
  } catch (err) {
    console.error("定位文件失败:", err);
  }
};

// 使用默认应用打开
const openWithSystemApp = async () => {
  if (!props.selectedFile) return;
  try {
    await invoke("open_with_system_app", { path: props.selectedFile.filePath });
  } catch (err) {
    console.error("打开文件失败:", err);
  }
};

// 键盘快捷键监听：F3 (Next) 与 Shift+F3 (Prev)
const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === "F3") {
    e.preventDefault();
    if (e.shiftKey) {
      jumpPrevMatch();
    } else {
      jumpNextMatch();
    }
  }
};

onMounted(() => {
  window.addEventListener("keydown", handleKeydown);
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", handleKeydown);
});

// 监听选中的文件和匹配项
watch(
  () => props.selectedFile,
  (newFile) => {
    if (newFile) {
      loadDocument(newFile.filePath);
    } else {
      textContent.value = null;
      excelContent.value = null;
      currentMatch.value = null;
    }
  }
);

watch(
  () => props.selectedMatch,
  (newMatch) => {
    if (newMatch) {
      currentMatch.value = newMatch;
    }
  },
  { immediate: true }
);
</script>
