<template>
  <div class="h-full flex flex-col glass-panel overflow-hidden border-r border-white/10">
    <!-- 结果栏顶部状态 -->
    <div class="px-4 py-3 border-b border-white/5 flex items-center justify-between shrink-0 bg-slate-900/60">
      <div class="flex items-center gap-2 text-xs text-slate-300 font-medium">
        <ListFilter class="w-3.5 h-3.5 text-sky-400" />
        <span>{{ t.searchResults }} ({{ results.length }} {{ t.files }}, {{ totalMatchCount }} {{ t.matches }})</span>
      </div>
      <div class="flex items-center gap-1.5">
        <button
          class="text-[11px] px-2 py-0.5 rounded text-slate-400 hover:text-slate-200 hover:bg-slate-800 transition cursor-pointer"
          :title="t.expandAll"
          @click="expandAll"
        >
          {{ t.expandAll }}
        </button>
        <button
          class="text-[11px] px-2 py-0.5 rounded text-slate-400 hover:text-slate-200 hover:bg-slate-800 transition cursor-pointer"
          :title="t.collapseAll"
          @click="collapseAll"
        >
          {{ t.collapseAll }}
        </button>
      </div>
    </div>

    <!-- 搜索结果列表容器 -->
    <div class="flex-1 overflow-y-auto p-3 space-y-2.5">
      <!-- 搜索中但尚无结果时的骨架占位 -->
      <div v-if="isSearching && results.length === 0" class="space-y-3 p-2">
        <div v-for="i in 4" :key="i" class="glass-card p-3 rounded-xl animate-pulse">
          <div class="h-4 bg-slate-700/60 rounded w-2/3 mb-2"></div>
          <div class="h-3 bg-slate-800/60 rounded w-full mb-1"></div>
          <div class="h-3 bg-slate-800/60 rounded w-4/5"></div>
        </div>
      </div>

      <!-- 搜索完毕且无结果提示 -->
      <div
        v-else-if="!isSearching && results.length === 0"
        class="h-full flex flex-col items-center justify-center text-center p-6 text-slate-500"
      >
        <SearchX class="w-12 h-12 text-slate-600 mb-3" />
        <p class="text-sm font-medium text-slate-400">{{ t.noResults }}</p>
        <p class="text-xs mt-1 text-slate-600">{{ t.noResultsTip }}</p>
      </div>

      <!-- 文件结果卡片列表 -->
      <div
        v-for="fileResult in results"
        :key="fileResult.filePath"
        class="glass-card rounded-xl overflow-hidden transition-all duration-150"
        :class="{
          'border-sky-500/40 bg-slate-850/80 shadow-md': selectedFilePath === fileResult.filePath
        }"
      >
        <!-- 文件头：文件名、格式图标、匹配总数、展开/折叠 -->
        <div
          class="px-3 py-2.5 flex items-center justify-between cursor-pointer hover:bg-slate-700/40 transition select-none"
          @click="toggleFileCollapse(fileResult.filePath)"
        >
          <div class="flex items-center gap-2 overflow-hidden flex-1 mr-2">
            <!-- 格式图标 -->
            <component
              :is="getFileIcon(fileResult.extension)"
              class="w-4 h-4 shrink-0"
              :class="getFileIconColor(fileResult.extension)"
            />

            <!-- 文件名与相对路径 -->
            <div class="overflow-hidden flex-1">
              <div class="flex items-center gap-2">
                <span class="text-xs font-semibold text-slate-200 truncate" :title="fileResult.fileName">
                  {{ fileResult.fileName }}
                </span>
                <span
                  class="text-[10px] px-1.5 py-0.2 rounded-full font-medium shrink-0"
                  :class="
                    fileResult.extension === 'xlsx' || fileResult.extension === 'xls'
                      ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
                      : 'bg-sky-500/20 text-sky-300 border border-sky-500/30'
                  "
                >
                  {{ fileResult.matches.length }} {{ t.hitsCount }}
                </span>
              </div>
              <div class="text-[10px] text-slate-500 truncate" :title="fileResult.filePath">
                {{ fileResult.filePath }}
              </div>
            </div>
          </div>

          <!-- 折叠箭头 -->
          <div class="text-slate-400 hover:text-slate-200 shrink-0">
            <ChevronDown
              class="w-4 h-4 transition-transform duration-200"
              :class="{ '-rotate-90': isCollapsed(fileResult.filePath) }"
            />
          </div>
        </div>

        <!-- 匹配行明细列表 -->
        <div
          v-show="!isCollapsed(fileResult.filePath)"
          class="border-t border-white/5 divide-y divide-white/5 bg-slate-950/40"
        >
          <div
            v-for="match in fileResult.matches"
            :key="`${fileResult.filePath}-${match.matchIndex}`"
            class="px-3 py-2 hover:bg-sky-500/10 cursor-pointer transition flex items-start gap-2.5 text-xs group"
            :class="{
              'bg-sky-500/15 border-l-2 border-sky-400 pl-2.5':
                selectedFilePath === fileResult.filePath && selectedMatchIndex === match.matchIndex
            }"
            @click="handleSelectMatch(fileResult, match)"
          >
            <!-- 行号或单元格坐标标签 -->
            <span
              class="px-1.5 py-0.5 rounded font-mono text-[10px] shrink-0 font-medium mt-0.5"
              :class="
                match.cellCoord
                  ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
                  : 'bg-slate-800 text-slate-400 group-hover:text-slate-200 border border-slate-700/60'
              "
            >
              {{ match.cellCoord ? `${match.sheetName}!${match.cellCoord}` : `L${match.lineNumber}` }}
            </span>

            <!-- 匹配文本片段（高亮关键词） -->
            <div class="flex-1 font-mono text-[11px] text-slate-300 break-all leading-relaxed">
              <span v-html="renderHighlightedSnippet(match)"></span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import {
  ListFilter,
  SearchX,
  FileText,
  FileSpreadsheet,
  FileCode,
  File,
  ChevronDown,
} from "@lucide/vue";
import { t } from "../i18n";
import type { FileMatchResult, MatchItem } from "../types/search";

const props = defineProps<{
  results: FileMatchResult[];
  isSearching: boolean;
  selectedFilePath?: string;
  selectedMatchIndex?: number;
}>();

const emit = defineEmits<{
  (e: "selectMatch", payload: { fileResult: FileMatchResult; match: MatchItem }): void;
}>();

// 折叠状态映射 (filePath -> boolean)
const collapsedMap = ref<Record<string, boolean>>({});

const isCollapsed = (filePath: string) => !!collapsedMap.value[filePath];

const toggleFileCollapse = (filePath: string) => {
  collapsedMap.value[filePath] = !collapsedMap.value[filePath];
};

const expandAll = () => {
  collapsedMap.value = {};
};

const collapseAll = () => {
  const map: Record<string, boolean> = {};
  props.results.forEach((r) => {
    map[r.filePath] = true;
  });
  collapsedMap.value = map;
};

// 统计总命中数
const totalMatchCount = computed(() => {
  return props.results.reduce((acc, cur) => acc + cur.matches.length, 0);
});

// 选择单处匹配
const handleSelectMatch = (fileResult: FileMatchResult, match: MatchItem) => {
  emit("selectMatch", { fileResult, match });
};

// 格式图标判断
const getFileIcon = (ext: string) => {
  if (ext === "xlsx" || ext === "xls" || ext === "ods" || ext === "csv") {
    return FileSpreadsheet;
  }
  if (["rs", "py", "js", "ts", "vue", "c", "cpp", "h", "cs", "java", "go", "php", "sql", "html", "css"].includes(ext)) {
    return FileCode;
  }
  if (["txt", "log", "md", "json", "xml", "yaml", "yml", "toml"].includes(ext)) {
    return FileText;
  }
  return File;
};

const getFileIconColor = (ext: string) => {
  if (ext === "xlsx" || ext === "xls" || ext === "ods") return "text-emerald-400";
  if (ext === "csv" || ext === "tsv") return "text-teal-400";
  if (["rs", "c", "cpp"].includes(ext)) return "text-orange-400";
  if (["js", "ts", "vue"].includes(ext)) return "text-amber-400";
  if (["py"].includes(ext)) return "text-blue-400";
  if (["md", "txt", "log"].includes(ext)) return "text-sky-400";
  return "text-slate-400";
};

// 安全转义 HTML
const escapeHtml = (text: string) => {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
};

// 高亮渲染单行中的命中关键词
const renderHighlightedSnippet = (match: MatchItem) => {
  const line = match.previewLine;
  const start = match.matchStart;
  const end = match.matchEnd;

  if (start >= end || start >= line.length) {
    return escapeHtml(line);
  }

  const before = escapeHtml(line.slice(0, start));
  const keyword = escapeHtml(line.slice(start, end));
  const after = escapeHtml(line.slice(end));

  return `${before}<mark class="bg-amber-500/30 text-amber-200 font-semibold px-0.5 py-0.2 rounded border border-amber-500/40">${keyword}</mark>${after}`;
};
</script>
