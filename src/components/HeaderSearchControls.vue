<template>
  <div class="glass-panel px-5 py-3.5 border-b shadow-sm relative z-30">
    <!-- 最顶部品牌与全局配置行 (Logo、标题、多语言、主题切换) -->
    <div class="flex items-center justify-between pb-3 mb-3 border-b" style="border-color: var(--border-subtle)">
      <!-- 品牌与 Logo -->
      <div class="flex items-center gap-2.5">
        <img src="/logo.png" alt="Logo" class="w-7 h-7 rounded-lg shadow-sm border shrink-0" style="border-color: var(--border-subtle)" />
        <div class="flex items-baseline gap-2">
          <span class="text-sm font-black tracking-wide bg-gradient-to-r from-sky-500 via-indigo-400 to-amber-500 bg-clip-text text-transparent">
            {{ t.appTitle }}
          </span>
          <span class="text-[11px] hidden sm:inline-block font-medium" style="color: var(--text-muted)">
            {{ t.appSubtitle }}
          </span>
        </div>
      </div>

      <!-- 右侧：多语言与多主题切换器 -->
      <div class="flex items-center gap-2">
        <!-- 语言选择器 (中/韩/英) -->
        <div class="relative flex items-center theme-input-box rounded-xl px-2.5 py-1 text-xs">
          <Languages class="w-3.5 h-3.5 text-sky-500 mr-1.5 shrink-0" />
          <select
            :value="currentLang"
            class="bg-transparent border-none outline-none text-xs cursor-pointer font-medium"
            style="color: var(--text-title)"
            @change="handleLangChange($event)"
          >
            <option value="zh" class="text-slate-900 bg-white dark:bg-slate-900 dark:text-slate-200">🇨🇳 简体中文</option>
            <option value="ko" class="text-slate-900 bg-white dark:bg-slate-900 dark:text-slate-200">🇰🇷 한국어</option>
            <option value="en" class="text-slate-900 bg-white dark:bg-slate-900 dark:text-slate-200">🇺🇸 English</option>
          </select>
        </div>

        <!-- 多主题选择器 -->
        <div class="relative flex items-center theme-input-box rounded-xl px-2.5 py-1 text-xs">
          <Palette class="w-3.5 h-3.5 text-amber-500 mr-1.5 shrink-0" />
          <select
            :value="currentTheme"
            class="bg-transparent border-none outline-none text-xs cursor-pointer font-medium"
            style="color: var(--text-title)"
            @change="handleThemeChange($event)"
          >
            <option v-for="theme in themeOptions" :key="theme.id" :value="theme.id" class="text-slate-900 bg-white dark:bg-slate-900 dark:text-slate-200">
              {{ theme.icon }} {{ t[theme.labelKey] }}
            </option>
          </select>
        </div>
      </div>
    </div>

    <!-- 搜索输入主控制行：路径与搜索关键词 -->
    <div class="grid grid-cols-1 md:grid-cols-12 gap-3 items-center">
      <!-- 搜索目录输入与历史下拉 -->
      <div class="md:col-span-5 relative">
        <div class="flex items-center gap-2 theme-input-box rounded-xl px-3 py-2 shadow-sm">
          <FolderOpen class="w-4 h-4 text-sky-500 shrink-0" />
          <input
            v-model="rootPath"
            type="text"
            :placeholder="t.folderPlaceholder"
            class="bg-transparent border-none outline-none text-sm w-full"
            style="color: var(--text-title)"
            @focus="showPathHistory = true"
            @keydown.enter="handleSearch"
          />
          <button
            v-if="pathHistory.length > 0"
            class="transition p-1 rounded cursor-pointer hover:opacity-80"
            style="color: var(--text-muted)"
            :title="t.folderHistory"
            @click.stop="showPathHistory = !showPathHistory"
          >
            <History class="w-3.5 h-3.5" />
          </button>
          <button
            class="px-2.5 py-1 text-xs font-medium rounded-lg shrink-0 transition flex items-center gap-1 cursor-pointer border shadow-xs"
            style="background-color: var(--bg-surface-hover); color: var(--text-title); border-color: var(--border-subtle)"
            :title="t.browse"
            @click="browseFolder"
          >
            <span>{{ t.browse }}</span>
          </button>
        </div>

        <!-- 目录历史下拉弹出框 -->
        <div
          v-if="showPathHistory && pathHistory.length > 0"
          v-click-outside="() => (showPathHistory = false)"
          class="absolute left-0 right-0 top-full mt-1.5 rounded-xl border shadow-xl z-50 overflow-hidden text-xs divide-y"
          style="background-color: var(--bg-card); border-color: var(--border-subtle); backdrop-filter: blur(16px);"
        >
          <div class="px-3 py-1.5 flex items-center justify-between font-semibold" style="background-color: var(--bg-surface); color: var(--text-muted)">
            <span class="flex items-center gap-1.5">
              <History class="w-3 h-3 text-sky-500" />
              {{ t.folderHistory }}
            </span>
            <button
              class="text-[10px] text-rose-500 hover:text-rose-600 transition cursor-pointer font-medium"
              @click.stop="clearAllPathHistory"
            >
              {{ t.clearAll }}
            </button>
          </div>
          <div class="max-h-48 overflow-y-auto divide-y" style="border-color: var(--border-subtle)">
            <div
              v-for="item in pathHistory"
              :key="item"
              class="px-3 py-2 flex items-center justify-between hover:bg-sky-500/10 cursor-pointer transition group"
              style="color: var(--text-body)"
              @click="selectPathHistory(item)"
            >
              <span class="truncate mr-2 font-mono" :title="item">{{ item }}</span>
              <button
                class="opacity-0 group-hover:opacity-100 p-0.5 rounded text-rose-400 hover:text-rose-600 hover:bg-rose-500/10 transition cursor-pointer shrink-0"
                :title="t.deleteItem"
                @click.stop="deletePathHistory(item)"
              >
                <X class="w-3.5 h-3.5" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 搜索关键词输入与历史下拉 -->
      <div class="md:col-span-5 relative">
        <div class="flex items-center gap-2 theme-input-box rounded-xl px-3 py-2 shadow-sm">
          <Search class="w-4 h-4 text-amber-500 shrink-0" />
          <input
            v-model="keyword"
            type="text"
            :placeholder="t.keywordPlaceholder"
            class="bg-transparent border-none outline-none text-sm w-full"
            style="color: var(--text-title)"
            @focus="showKeywordHistory = true"
            @keydown.enter="handleSearch"
          />
          <button
            v-if="keywordHistory.length > 0"
            class="transition p-1 rounded cursor-pointer hover:opacity-80"
            style="color: var(--text-muted)"
            :title="t.searchHistory"
            @click.stop="showKeywordHistory = !showKeywordHistory"
          >
            <History class="w-3.5 h-3.5" />
          </button>
          <button
            v-if="keyword"
            class="transition p-0.5 rounded cursor-pointer"
            style="color: var(--text-muted)"
            title="Clear"
            @click="keyword = ''"
          >
            <X class="w-3.5 h-3.5" />
          </button>
        </div>

        <!-- 关键词历史下拉弹出框 (支持单个删除与快速填入) -->
        <div
          v-if="showKeywordHistory && keywordHistory.length > 0"
          v-click-outside="() => (showKeywordHistory = false)"
          class="absolute left-0 right-0 top-full mt-1.5 rounded-xl border shadow-xl z-50 overflow-hidden text-xs divide-y"
          style="background-color: var(--bg-card); border-color: var(--border-subtle); backdrop-filter: blur(16px);"
        >
          <div class="px-3 py-1.5 flex items-center justify-between font-semibold" style="background-color: var(--bg-surface); color: var(--text-muted)">
            <span class="flex items-center gap-1.5">
              <History class="w-3 h-3 text-amber-500" />
              {{ t.searchHistory }}
            </span>
            <button
              class="text-[10px] text-rose-500 hover:text-rose-600 transition cursor-pointer font-medium"
              @click.stop="clearAllKeywordHistory"
            >
              {{ t.clearAll }}
            </button>
          </div>
          <div class="max-h-48 overflow-y-auto divide-y" style="border-color: var(--border-subtle)">
            <div
              v-for="item in keywordHistory"
              :key="item"
              class="px-3 py-2 flex items-center justify-between hover:bg-amber-500/10 cursor-pointer transition group"
              style="color: var(--text-body)"
              @click="selectKeywordHistory(item)"
            >
              <span class="truncate mr-2 font-mono font-medium" :title="item">{{ item }}</span>
              <button
                class="opacity-0 group-hover:opacity-100 p-0.5 rounded text-rose-400 hover:text-rose-600 hover:bg-rose-500/10 transition cursor-pointer shrink-0"
                :title="t.deleteItem"
                @click.stop="deleteKeywordHistory(item)"
              >
                <X class="w-3.5 h-3.5" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 搜索/停止操作主按钮 -->
      <div class="md:col-span-2 flex items-center gap-2">
        <button
          v-if="!isSearching"
          class="w-full py-2 px-4 rounded-xl text-sm font-semibold text-white bg-gradient-to-r from-sky-500 to-indigo-600 hover:from-sky-400 hover:to-indigo-500 shadow-md shadow-sky-500/25 active:scale-[0.98] transition-all flex items-center justify-center gap-2 cursor-pointer"
          :disabled="!keyword.trim() || !rootPath.trim()"
          :class="{ 'opacity-50 cursor-not-allowed': !keyword.trim() || !rootPath.trim() }"
          @click="handleSearch"
        >
          <Zap class="w-4 h-4 fill-amber-300 text-amber-300" />
          <span>{{ t.startSearch }}</span>
        </button>

        <button
          v-else
          class="w-full py-2 px-4 rounded-xl text-sm font-semibold text-white bg-gradient-to-r from-rose-600 to-red-500 hover:from-rose-500 hover:to-red-400 shadow-md shadow-rose-500/25 active:scale-[0.98] transition-all flex items-center justify-center gap-2 cursor-pointer animate-pulse"
          @click="handleCancel"
        >
          <Square class="w-4 h-4 fill-current" />
          <span>{{ t.stopSearch }}</span>
        </button>
      </div>
    </div>

    <!-- 底部辅助行：扩展名预设过滤与搜索高级选项 -->
    <div class="mt-3 flex flex-wrap items-center justify-between gap-3 text-xs border-t pt-2.5" style="border-color: var(--border-subtle)">
      <!-- 扩展名选择器与自定义输入 -->
      <div class="flex flex-wrap items-center gap-1.5">
        <span class="font-medium mr-1 flex items-center gap-1" style="color: var(--text-muted)">
          <Filter class="w-3.5 h-3.5" />
          {{ t.formatFilter }}
        </span>
        <button
          v-for="preset in getExtensionPresets()"
          :key="preset.id"
          class="px-2.5 py-1 rounded-lg border transition cursor-pointer font-medium"
          :style="
            activePreset === preset.id
              ? 'background-color: var(--match-badge-bg); border-color: var(--match-badge-border); color: var(--match-badge-text); font-weight: 600;'
              : 'background-color: var(--bg-surface); border-color: var(--border-subtle); color: var(--text-muted);'
          "
          @click="selectPreset(preset.id)"
        >
          {{ preset.label }}
        </button>

        <!-- 自定义后缀输入框 -->
        <div
          v-if="activePreset === 'custom'"
          class="flex items-center gap-1 theme-input-box rounded-lg px-2 py-0.5"
        >
          <input
            v-model="customExtensions"
            type="text"
            :placeholder="t.customPlaceholder"
            class="bg-transparent border-none outline-none text-xs w-32"
            style="color: var(--accent-primary)"
          />
        </div>
      </div>

      <!-- 搜索高级条件开关 -->
      <div class="flex flex-wrap items-center gap-2">
        <label
          class="flex items-center gap-1 px-2.5 py-1 rounded-lg border transition cursor-pointer select-none font-medium text-[11px]"
          :style="
            caseSensitive
              ? 'background-color: rgba(99, 102, 241, 0.15); border-color: rgba(99, 102, 241, 0.4); color: #6366f1; font-weight: 600;'
              : 'background-color: var(--bg-surface); border-color: var(--border-subtle); color: var(--text-muted);'
          "
          :title="t.caseSensitiveTip"
        >
          <input v-model="caseSensitive" type="checkbox" class="hidden" />
          <span class="font-mono font-bold text-[11px]">Aa</span>
          <span>{{ t.caseSensitive }}</span>
        </label>

        <label
          class="flex items-center gap-1 px-2.5 py-1 rounded-lg border transition cursor-pointer select-none font-medium text-[11px]"
          :style="
            isRegex
              ? 'background-color: rgba(245, 158, 11, 0.15); border-color: rgba(245, 158, 11, 0.4); color: #d97706; font-weight: 600;'
              : 'background-color: var(--bg-surface); border-color: var(--border-subtle); color: var(--text-muted);'
          "
          :title="t.regexTip"
        >
          <input v-model="isRegex" type="checkbox" class="hidden" />
          <span class="font-mono font-bold text-[11px]">.*</span>
          <span>{{ t.regex }}</span>
        </label>

        <label
          class="flex items-center gap-1 px-2.5 py-1 rounded-lg border transition cursor-pointer select-none font-medium text-[11px]"
          :style="
            wholeWord
              ? 'background-color: rgba(13, 148, 136, 0.15); border-color: rgba(13, 148, 136, 0.4); color: #0d9488; font-weight: 600;'
              : 'background-color: var(--bg-surface); border-color: var(--border-subtle); color: var(--text-muted);'
          "
          :title="t.wholeWordTip"
        >
          <input v-model="wholeWord" type="checkbox" class="hidden" />
          <span class="font-mono font-bold text-[11px]">\b</span>
          <span>{{ t.wholeWord }}</span>
        </label>

        <label
          class="flex items-center gap-1 px-2.5 py-1 rounded-lg border transition cursor-pointer select-none font-medium text-[11px]"
          :style="
            includeSubdirectories
              ? 'background-color: var(--match-badge-bg); border-color: var(--match-badge-border); color: var(--match-badge-text); font-weight: 600;'
              : 'background-color: var(--bg-surface); border-color: var(--border-subtle); color: var(--text-muted);'
          "
          :title="t.subdirectoriesTip"
        >
          <input v-model="includeSubdirectories" type="checkbox" class="hidden" />
          <FolderGit2 class="w-3.5 h-3.5" />
          <span>{{ t.subdirectories }}</span>
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
  Languages,
  Palette,
  History,
} from "@lucide/vue";
import { t, currentLang, setLanguage, type LanguageKey } from "../i18n";
import { currentTheme, applyTheme, themeOptions, type ThemeKey } from "../theme";
import type { SearchQuery } from "../types/search";

const emit = defineEmits<{
  (e: "search", query: SearchQuery): void;
  (e: "cancel"): void;
}>();

defineProps<{
  isSearching: boolean;
}>();

// 自定义点击外部指令
const vClickOutside = {
  mounted(el: any, binding: any) {
    el.clickOutsideEvent = (event: Event) => {
      if (!(el === event.target || el.contains(event.target))) {
        binding.value(event);
      }
    };
    document.addEventListener("click", el.clickOutsideEvent);
  },
  unmounted(el: any) {
    document.removeEventListener("click", el.clickOutsideEvent);
  },
};

// 搜索条件与持久化状态
const rootPath = ref(localStorage.getItem("flashtext_last_path") || "");
const keyword = ref(localStorage.getItem("flashtext_last_keyword") || "");
const caseSensitive = ref(false);
const isRegex = ref(false);
const wholeWord = ref(false);
const includeSubdirectories = ref(true);
const ignoreHidden = ref(true);

// 历史记录状态
const pathHistory = ref<string[]>(
  JSON.parse(localStorage.getItem("flashtext_path_history") || "[]")
);
const keywordHistory = ref<string[]>(
  JSON.parse(localStorage.getItem("flashtext_keyword_history") || "[]")
);

const showPathHistory = ref(false);
const showKeywordHistory = ref(false);

// 保存目录历史
const savePathHistory = (path: string) => {
  if (!path.trim()) return;
  const filtered = pathHistory.value.filter((p) => p !== path.trim());
  filtered.unshift(path.trim());
  pathHistory.value = filtered.slice(0, 15);
  localStorage.setItem("flashtext_path_history", JSON.stringify(pathHistory.value));
  localStorage.setItem("flashtext_last_path", path.trim());
};

// 单个删除目录历史
const deletePathHistory = (path: string) => {
  pathHistory.value = pathHistory.value.filter((p) => p !== path);
  localStorage.setItem("flashtext_path_history", JSON.stringify(pathHistory.value));
};

// 清空所有目录历史
const clearAllPathHistory = () => {
  pathHistory.value = [];
  localStorage.removeItem("flashtext_path_history");
  showPathHistory.value = false;
};

// 选择目录历史
const selectPathHistory = (path: string) => {
  rootPath.value = path;
  showPathHistory.value = false;
};

// 保存关键词历史
const saveKeywordHistory = (kw: string) => {
  if (!kw.trim()) return;
  const filtered = keywordHistory.value.filter((k) => k !== kw.trim());
  filtered.unshift(kw.trim());
  keywordHistory.value = filtered.slice(0, 20);
  localStorage.setItem("flashtext_keyword_history", JSON.stringify(keywordHistory.value));
  localStorage.setItem("flashtext_last_keyword", kw.trim());
};

// 单个删除关键词历史
const deleteKeywordHistory = (kw: string) => {
  keywordHistory.value = keywordHistory.value.filter((k) => k !== kw);
  localStorage.setItem("flashtext_keyword_history", JSON.stringify(keywordHistory.value));
};

// 清空所有关键词历史
const clearAllKeywordHistory = () => {
  keywordHistory.value = [];
  localStorage.removeItem("flashtext_keyword_history");
  showKeywordHistory.value = false;
};

// 选择关键词历史
const selectKeywordHistory = (kw: string) => {
  keyword.value = kw;
  showKeywordHistory.value = false;
  handleSearch();
};

// 扩展名过滤预设
const activePreset = ref("all_text");
const customExtensions = ref("txt, xlsx, log");

const getExtensionPresets = () => [
  { id: "all_text", label: t.value.presetAll, exts: [] },
  { id: "pure_text", label: t.value.presetText, exts: ["txt", "log", "md", "ini", "conf"] },
  { id: "excel", label: t.value.presetExcel, exts: ["xlsx", "xls", "ods", "csv"] },
  { id: "code", label: t.value.presetCode, exts: ["rs", "py", "js", "ts", "vue", "c", "cpp", "h", "cs", "java", "go", "php", "sql", "html", "css"] },
  { id: "custom", label: t.value.presetCustom, exts: [] },
];

const handleLangChange = (event: Event) => {
  const target = event.target as HTMLSelectElement;
  setLanguage(target.value as LanguageKey);
};

const handleThemeChange = (event: Event) => {
  const target = event.target as HTMLSelectElement;
  applyTheme(target.value as ThemeKey);
};

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
      savePathHistory(selected);
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
  const presets = getExtensionPresets();
  const found = presets.find((p) => p.id === activePreset.value);
  return found ? found.exts : [];
};

// 发起搜索
const handleSearch = () => {
  if (!rootPath.value.trim() || !keyword.value.trim()) return;

  // 记录并持久化路径与关键词历史
  savePathHistory(rootPath.value);
  saveKeywordHistory(keyword.value);
  showPathHistory.value = false;
  showKeywordHistory.value = false;

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


