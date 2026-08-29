<template>
  <div class="glass-panel px-5 py-4 border-b border-white/10 shadow-lg relative z-20">
    <!-- 顶部主控制行：路径与搜索关键词 -->
    <div class="grid grid-cols-1 md:grid-cols-12 gap-3 items-center">
      <!-- 搜索目录输入与选择 -->
      <div class="md:col-span-5 flex items-center gap-2 bg-slate-900/80 border border-slate-700/80 rounded-xl px-3 py-2 focus-within:border-sky-500/80 focus-within:ring-2 focus-within:ring-sky-500/20 transition-all shadow-inner">
        <FolderOpen class="w-4 h-4 text-sky-400 shrink-0" />
        <input
          v-model="rootPath"
          type="text"
          placeholder="选择或输入搜索根目录路径..."
          class="bg-transparent border-none outline-none text-sm text-slate-100 placeholder-slate-500 w-full"
          @keydown.enter="handleSearch"
        />
        <button
          class="px-2.5 py-1 text-xs font-medium text-slate-300 hover:text-white bg-slate-800 hover:bg-slate-700 border border-slate-600/60 rounded-lg shrink-0 transition flex items-center gap-1 cursor-pointer"
          title="浏览并选择文件夹"
          @click="browseFolder"
        >
          <span>浏览</span>
        </button>
      </div>

      <!-- 搜索关键词输入 -->
      <div class="md:col-span-5 flex items-center gap-2 bg-slate-900/80 border border-slate-700/80 rounded-xl px-3 py-2 focus-within:border-sky-500/80 focus-within:ring-2 focus-within:ring-sky-500/20 transition-all shadow-inner">
        <Search class="w-4 h-4 text-amber-400 shrink-0" />
        <input
          v-model="keyword"
          type="text"
          placeholder="输入要搜索的文本内容 (例如 test, function, 订单号)..."
          class="bg-transparent border-none outline-none text-sm text-slate-100 placeholder-slate-500 w-full"
          @keydown.enter="handleSearch"
        />
        <button
          v-if="keyword"
          class="text-slate-400 hover:text-slate-200 transition p-0.5 rounded cursor-pointer"
          title="清空关键词"
          @click="keyword = ''"
        >
          <X class="w-3.5 h-3.5" />
        </button>
      </div>

      <!-- 搜索/停止操作主按钮 -->
      <div class="md:col-span-2 flex items-center gap-2">
        <button
          v-if="!isSearching"
          class="w-full py-2 px-4 rounded-xl text-sm font-semibold text-white bg-gradient-to-r from-sky-500 to-indigo-600 hover:from-sky-400 hover:to-indigo-500 shadow-md shadow-sky-500/20 active:scale-[0.98] transition-all flex items-center justify-center gap-2 cursor-pointer"
          :disabled="!keyword.trim() || !rootPath.trim()"
          :class="{ 'opacity-50 cursor-not-allowed': !keyword.trim() || !rootPath.trim() }"
          @click="handleSearch"
        >
          <Zap class="w-4 h-4 fill-amber-300 text-amber-300" />
          <span>极速搜索</span>
        </button>

        <button
          v-else
          class="w-full py-2 px-4 rounded-xl text-sm font-semibold text-white bg-gradient-to-r from-rose-600 to-red-500 hover:from-rose-500 hover:to-red-400 shadow-md shadow-rose-500/20 active:scale-[0.98] transition-all flex items-center justify-center gap-2 cursor-pointer animate-pulse"
          @click="handleCancel"
        >
          <Square class="w-4 h-4 fill-current" />
          <span>停止搜索</span>
        </button>
      </div>
    </div>

    <!-- 底部辅助行：扩展名预设过滤与搜索高级选项 -->
    <div class="mt-3 flex flex-wrap items-center justify-between gap-3 text-xs border-t border-white/5 pt-2.5">
      <!-- 扩展名选择器与自定义输入 -->
      <div class="flex flex-wrap items-center gap-1.5">
        <span class="text-slate-400 font-medium mr-1 flex items-center gap-1">
          <Filter class="w-3.5 h-3.5 text-slate-500" />
          格式过滤:
        </span>
        <button
          v-for="preset in extensionPresets"
          :key="preset.id"
          class="px-2.5 py-1 rounded-lg border transition cursor-pointer"
          :class="
            activePreset === preset.id
              ? 'bg-sky-500/20 border-sky-500/60 text-sky-300 font-medium'
              : 'bg-slate-800/60 border-slate-700/60 text-slate-400 hover:text-slate-200 hover:bg-slate-700/60'
          "
          @click="selectPreset(preset.id)"
        >
          {{ preset.label }}
        </button>

        <!-- 自定义后缀输入框 -->
        <div
          v-if="activePreset === 'custom'"
          class="flex items-center gap-1 bg-slate-900/90 border border-sky-500/60 rounded-lg px-2 py-0.5"
        >
          <input
            v-model="customExtensions"
            type="text"
            placeholder="如 txt, xlsx, log"
            class="bg-transparent border-none outline-none text-xs text-sky-200 placeholder-slate-500 w-32"
          />
        </div>
      </div>

      <!-- 搜索高级条件开关 -->
      <div class="flex flex-wrap items-center gap-2">
        <label
          class="flex items-center gap-1 px-2 py-1 rounded-lg border transition cursor-pointer select-none"
          :class="caseSensitive ? 'bg-indigo-500/20 border-indigo-500/50 text-indigo-300' : 'bg-slate-800/40 border-slate-700/50 text-slate-400 hover:text-slate-300'"
          title="区分英文字母大小写"
        >
          <input v-model="caseSensitive" type="checkbox" class="hidden" />
          <span class="font-mono font-bold text-[11px]">Aa</span>
          <span>区分大小写</span>
        </label>

        <label
          class="flex items-center gap-1 px-2 py-1 rounded-lg border transition cursor-pointer select-none"
          :class="isRegex ? 'bg-amber-500/20 border-amber-500/50 text-amber-300' : 'bg-slate-800/40 border-slate-700/50 text-slate-400 hover:text-slate-300'"
          title="使用正则表达式检索"
        >
          <input v-model="isRegex" type="checkbox" class="hidden" />
          <span class="font-mono font-bold text-[11px]">.*</span>
          <span>正则表达式</span>
        </label>

        <label
          class="flex items-center gap-1 px-2 py-1 rounded-lg border transition cursor-pointer select-none"
          :class="wholeWord ? 'bg-teal-500/20 border-teal-500/50 text-teal-300' : 'bg-slate-800/40 border-slate-700/50 text-slate-400 hover:text-slate-300'"
          title="仅匹配完整独立的单词"
        >
          <input v-model="wholeWord" type="checkbox" class="hidden" />
          <span class="font-mono font-bold text-[11px]">\b</span>
          <span>全词匹配</span>
        </label>

        <label
          class="flex items-center gap-1 px-2 py-1 rounded-lg border transition cursor-pointer select-none"
          :class="includeSubdirectories ? 'bg-sky-500/20 border-sky-500/50 text-sky-300' : 'bg-slate-800/40 border-slate-700/50 text-slate-400 hover:text-slate-300'"
          title="递归搜索所有子文件夹"
        >
          <input v-model="includeSubdirectories" type="checkbox" class="hidden" />
          <FolderGit2 class="w-3.5 h-3.5" />
          <span>包含子目录</span>
        </label>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  FolderOpen,
  Search,
  Zap,
  Square,
  X,
  Filter,
  FolderGit2,
} from "@lucide/vue";
import type { SearchQuery } from "../types/search";

const emit = defineEmits<{
  (e: "search", query: SearchQuery): void;
  (e: "cancel"): void;
}>();

defineProps<{
  isSearching: boolean;
}>();

// 搜索条件状态
const rootPath = ref("");
const keyword = ref("");
const caseSensitive = ref(false);
const isRegex = ref(false);
const wholeWord = ref(false);
const includeSubdirectories = ref(true);
const ignoreHidden = ref(true);

// 扩展名过滤预设
const activePreset = ref("all_text");
const customExtensions = ref("txt, xlsx, log");

const extensionPresets = [
  { id: "all_text", label: "全部文本/代码", exts: [] },
  { id: "pure_text", label: "纯文本 (*.txt, *.log, *.md)", exts: ["txt", "log", "md", "ini", "conf"] },
  { id: "excel", label: "Excel 表格 (*.xlsx, *.xls)", exts: ["xlsx", "xls", "ods", "csv"] },
  { id: "code", label: "源代码文件", exts: ["rs", "py", "js", "ts", "vue", "c", "cpp", "h", "cs", "java", "go", "php", "sql", "html", "css"] },
  { id: "custom", label: "自定义后缀...", exts: [] },
];

// 选择预设
const selectPreset = (presetId: string) => {
  activePreset.value = presetId;
};

// 打开系统原生文件夹选择器
const browseFolder = async () => {
  try {
    const selected = await invoke<string | null>("select_folder");
    if (selected) {
      rootPath.value = selected;
    }
  } catch (err) {
    console.error("选择文件夹失败:", err);
  }
};

// 获取最终扩展名数组
const getResolvedExtensions = (): string[] => {
  if (activePreset.value === "custom") {
    return customExtensions.value
      .split(/[,;\s]+/)
      .map((s) => s.trim().replace(/^\*\./, "").replace(/^\./, ""))
      .filter(Boolean);
  }
  const found = extensionPresets.find((p) => p.id === activePreset.value);
  return found ? found.exts : [];
};

// 发起搜索
const handleSearch = () => {
  if (!rootPath.value.trim() || !keyword.value.trim()) return;

  const query: SearchQuery = {
    rootPath: rootPath.value.trim(),
    keyword: keyword.value.trim(),
    extensions: getResolvedExtensions(),
    caseSensitive: caseSensitive.value,
    isRegex: isRegex.value,
    wholeWord: wholeWord.value,
    includeSubdirectories: includeSubdirectories.value,
    ignoreHidden: ignoreHidden.value,
  };

  emit("search", query);
};

// 中止搜索
const handleCancel = () => {
  emit("cancel");
};
</script>
