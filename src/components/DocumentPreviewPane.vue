<template>
  <div class="h-full flex flex-col glass-panel overflow-hidden bg-[#090d16]">
    <!-- 预览区顶部工具栏 -->
    <div
      v-if="selectedFile"
      class="px-4 py-2.5 bg-slate-900/90 border-b border-white/10 shrink-0 flex flex-wrap items-center justify-between gap-2 shadow-sm"
    >
      <!-- 文件名与详细信息 -->
      <div class="flex items-center gap-2.5 overflow-hidden flex-1 min-w-[200px]">
        <component
          :is="getFileIcon(selectedFile.extension)"
          class="w-4 h-4 shrink-0 text-sky-400"
        />
        <div class="overflow-hidden">
          <div class="flex items-center gap-2">
            <span class="text-xs font-bold text-slate-100 truncate" :title="selectedFile.fileName">
              {{ selectedFile.fileName }}
            </span>
            <span
              v-if="textContent?.encoding"
              class="text-[10px] px-1.5 py-0.2 bg-slate-800 text-slate-400 border border-slate-700 rounded font-mono shrink-0"
            >
              {{ textContent.encoding }}
            </span>
            <span
              v-if="textContent?.isTruncated"
              class="text-[10px] px-1.5 py-0.2 bg-amber-500/20 text-amber-300 border border-amber-500/40 rounded font-medium shrink-0"
            >
              前 20,000 行预览
            </span>
          </div>
          <div class="text-[10px] text-slate-500 truncate" :title="selectedFile.filePath">
            {{ selectedFile.filePath }}
          </div>
        </div>
      </div>

      <!-- 多匹配项快速跳转导航条 (Next / Prev 按键) -->
      <div
        v-if="selectedFile.matches.length > 0"
        class="flex items-center gap-1.5 bg-slate-950/80 px-2.5 py-1 rounded-xl border border-slate-700/60 shadow-inner"
      >
        <span class="text-[11px] text-slate-400 font-medium">
          匹配项:
          <span class="text-amber-400 font-bold font-mono">
            {{ activeMatchIndex }}
          </span>
          / {{ selectedFile.matches.length }}
        </span>

        <!-- 上一个匹配 (Shift+F3) -->
        <button
          class="p-1 rounded text-slate-300 hover:text-white hover:bg-slate-800 disabled:opacity-30 disabled:cursor-not-allowed transition cursor-pointer"
          :disabled="selectedFile.matches.length <= 1"
          title="跳转到上一个匹配项 (Shift + F3)"
          @click="jumpPrevMatch"
        >
          <ChevronUp class="w-4 h-4" />
        </button>

        <!-- 下一个匹配 (F3) -->
        <button
          class="p-1 rounded text-slate-300 hover:text-white hover:bg-slate-800 disabled:opacity-30 disabled:cursor-not-allowed transition cursor-pointer"
          :disabled="selectedFile.matches.length <= 1"
          title="跳转到下一个匹配项 (F3)"
          @click="jumpNextMatch"
        >
          <ChevronDown class="w-4 h-4" />
        </button>
      </div>

      <!-- 外部原生操作快捷按钮 -->
      <div class="flex items-center gap-1.5 shrink-0">
        <button
          class="px-2.5 py-1 text-xs font-medium text-slate-300 hover:text-white bg-slate-800 hover:bg-slate-700 border border-slate-700 rounded-lg transition flex items-center gap-1 cursor-pointer"
          title="在 Windows 资源管理器中高亮定位"
          @click="openInExplorer"
        >
          <FolderSearch class="w-3.5 h-3.5 text-sky-400" />
          <span>定位</span>
        </button>

        <button
          class="px-2.5 py-1 text-xs font-medium text-slate-300 hover:text-white bg-slate-800 hover:bg-slate-700 border border-slate-700 rounded-lg transition flex items-center gap-1 cursor-pointer"
          title="使用系统默认程序打开该文件"
          @click="openWithSystemApp"
        >
          <ExternalLink class="w-3.5 h-3.5 text-emerald-400" />
          <span>打开</span>
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
        <span class="text-xs font-medium">正在极速加载文档内容...</span>
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
          使用系统默认应用打开该文档
        </button>
      </div>

      <!-- 未选择任何文档时的占位提示 -->
      <div
        v-else-if="!selectedFile"
        class="h-full flex flex-col items-center justify-center text-center p-8 text-slate-500"
      >
        <Eye class="w-14 h-14 text-slate-700 mb-3" />
        <p class="text-sm font-medium text-slate-400">选择左侧搜索结果查看文档预览</p>
        <p class="text-xs mt-1 text-slate-600">
          支持实时平滑跳转到指定命中行、多匹配项导航与 Excel 工作表单元格高亮
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
