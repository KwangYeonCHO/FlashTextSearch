<template>
  <div class="h-full flex flex-col glass-panel overflow-hidden border-r" style="border-color: var(--border-subtle)">
    <!-- 结果栏顶部状态 -->
    <div
      class="px-4 py-3 border-b flex items-center justify-between shrink-0"
      style="background-color: var(--bg-surface); border-color: var(--border-subtle)"
    >
      <div class="flex items-center gap-2 text-xs font-semibold" style="color: var(--text-title)">
        <ListFilter class="w-3.5 h-3.5 text-sky-500" />
        <span>{{ t.searchResults }} ({{ results.length }} {{ t.files }}, {{ totalMatchCount }} {{ t.matches }})</span>
      </div>
      <div class="flex items-center gap-1.5">
        <button
          class="text-[11px] px-2 py-0.5 rounded transition cursor-pointer border shadow-2xs font-medium"
          style="background-color: var(--bg-card); color: var(--text-muted); border-color: var(--border-subtle)"
          :title="t.expandAll"
          @click="expandAll"
        >
          {{ t.expandAll }}
        </button>
        <button
          class="text-[11px] px-2 py-0.5 rounded transition cursor-pointer border shadow-2xs font-medium"
          style="background-color: var(--bg-card); color: var(--text-muted); border-color: var(--border-subtle)"
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
          <div class="h-4 rounded w-2/3 mb-2" style="background-color: var(--bg-surface-hover)"></div>
          <div class="h-3 rounded w-full mb-1" style="background-color: var(--bg-surface-hover)"></div>
          <div class="h-3 rounded w-4/5" style="background-color: var(--bg-surface-hover)"></div>
        </div>
      </div>

      <!-- 搜索完毕且无结果提示 -->
      <div
        v-else-if="!isSearching && results.length === 0"
        class="h-full flex flex-col items-center justify-center text-center p-6"
        style="color: var(--text-muted)"
      >
        <SearchX class="w-12 h-12 mb-3 opacity-60" style="color: var(--text-dim)" />
        <p class="text-sm font-semibold" style="color: var(--text-title)">{{ t.noResults }}</p>
        <p class="text-xs mt-1" style="color: var(--text-muted)">{{ t.noResultsTip }}</p>
      </div>

      <!-- 文件结果卡片列表 -->
      <div
        v-for="fileResult in results"
        :key="fileResult.filePath"
        class="glass-card rounded-xl overflow-hidden transition-all duration-150"
        :style="
          selectedFilePath === fileResult.filePath
            ? 'border-color: var(--accent-primary); box-shadow: 0 0 0 2px rgba(2, 132, 199, 0.2);'
            : ''
        "
      >
        <!-- 文件头：文件名、格式图标、匹配总数、展开/折叠 -->
        <div
          class="px-3 py-2.5 flex items-center justify-between cursor-pointer transition select-none"
          style="background-color: var(--bg-card)"
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
                <span class="text-xs font-bold truncate" style="color: var(--text-title)" :title="fileResult.fileName">
                  {{ fileResult.fileName }}
                </span>
                <span
                  class="text-[10px] px-1.5 py-0.2 rounded-full font-bold shrink-0"
                  :style="
                    fileResult.extension === 'xlsx' || fileResult.extension === 'xls'
                      ? 'background-color: rgba(16, 185, 129, 0.15); color: #059669; border: 1px solid rgba(16, 185, 129, 0.3);'
                      : 'background-color: var(--match-badge-bg); color: var(--match-badge-text); border: 1px solid var(--match-badge-border);'
                  "
                >
                  {{ fileResult.matches.length }} {{ t.hitsCount }}
                </span>
              </div>
              <div class="text-[10px] truncate" style="color: var(--text-muted)" :title="fileResult.filePath">
                {{ fileResult.filePath }}
              </div>
            </div>
          </div>

          <!-- 折叠箭头 -->
          <div class="shrink-0" style="color: var(--text-muted)">
            <ChevronDown
              class="w-4 h-4 transition-transform duration-200"
              :class="{ '-rotate-90': isCollapsed(fileResult.filePath) }"
            />
          </div>
        </div>

        <!-- 匹配行明细列表 -->
        <div
          v-show="!isCollapsed(fileResult.filePath)"
          class="border-t divide-y"
          style="border-color: var(--border-subtle); background-color: var(--bg-match-row)"
        >
          <div
            v-for="match in fileResult.matches"
            :key="`${fileResult.filePath}-${match.matchIndex}`"
            class="px-3 py-2 cursor-pointer transition flex items-start gap-2.5 text-xs group"
            :style="
              selectedFilePath === fileResult.filePath && selectedMatchIndex === match.matchIndex
                ? 'background-color: var(--match-badge-bg); border-left: 3px solid var(--accent-primary);'
                : 'border-left: 3px solid transparent;'
            "
            @click="handleSelectMatch(fileResult, match)"
          >
            <!-- 行号或单元格坐标标签 -->
            <span
              class="px-1.5 py-0.5 rounded font-mono text-[10px] shrink-0 font-bold mt-0.5 border"
              :style="
                match.cellCoord
                  ? 'background-color: rgba(16, 185, 129, 0.15); color: #059669; border-color: rgba(16, 185, 129, 0.3);'
                  : 'background-color: var(--bg-surface); color: var(--text-muted); border-color: var(--border-subtle);'
              "
            >
              {{ match.cellCoord ? `${match.sheetName}!${match.cellCoord}` : `L${match.lineNumber}` }}
            </span>

            <!-- 匹配文本片段（高亮关键词） -->
            <div class="flex-1 font-mono text-[11px] break-all leading-relaxed" style="color: var(--text-body)">
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
