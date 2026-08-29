<template>
  <div class="h-screen w-screen flex flex-col bg-[#090d16] text-slate-100 overflow-hidden select-none">
    <!-- 顶部搜索控制台 -->
    <HeaderSearchControls
      :is-searching="isSearching"
      @search="handleStartSearch"
      @cancel="handleCancelSearch"
    />

    <!-- 中部双栏工作区 -->
    <div class="flex-1 flex overflow-hidden relative">
      <!-- 左侧搜索结果栏 -->
      <div class="w-[360px] md:w-[420px] lg:w-[460px] shrink-0 h-full flex flex-col">
        <SearchResultsList
          :results="searchResults"
          :is-searching="isSearching"
          :selected-file-path="selectedFile?.filePath"
          :selected-match-index="selectedMatch?.matchIndex"
          @select-match="handleSelectMatch"
        />
      </div>

      <!-- 右侧沉浸式文档预览区 -->
      <div class="flex-1 h-full min-w-0">
        <DocumentPreviewPane
          :selected-file="selectedFile"
          :selected-match="selectedMatch"
          @change-match="handleMatchChange"
        />
      </div>
    </div>

    <!-- 底部状态与性能指标栏 -->
    <StatusBar
      :is-searching="isSearching"
      :progress="progress"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import HeaderSearchControls from "./components/HeaderSearchControls.vue";
import SearchResultsList from "./components/SearchResultsList.vue";
import DocumentPreviewPane from "./components/DocumentPreviewPane.vue";
import StatusBar from "./components/StatusBar.vue";
import type {
  FileMatchResult,
  MatchItem,
  SearchProgress,
  SearchQuery,
} from "./types/search";

const isSearching = ref(false);
const searchResults = ref<FileMatchResult[]>([]);
const selectedFile = ref<FileMatchResult | null>(null);
const selectedMatch = ref<MatchItem | null>(null);

const progress = ref<SearchProgress>({
  filesScanned: 0,
  filesMatched: 0,
  totalMatches: 0,
  elapsedMs: 0,
  isFinished: false,
  isCancelled: false,
});

let unlistenBatch: UnlistenFn | null = null;
let unlistenProgress: UnlistenFn | null = null;
let unlistenFinished: UnlistenFn | null = null;

// 发起全文搜索
const handleStartSearch = async (query: SearchQuery) => {
  isSearching.value = true;
  searchResults.value = [];
  selectedFile.value = null;
  selectedMatch.value = null;

  progress.value = {
    filesScanned: 0,
    filesMatched: 0,
    totalMatches: 0,
    elapsedMs: 0,
    isFinished: false,
    isCancelled: false,
  };

  try {
    await invoke("start_search", { query });
  } catch (err) {
    console.error("启动搜索任务失败:", err);
    isSearching.value = false;
    alert(`启动搜索任务失败: ${err}`);
  }
};

// 手动取消搜索
const handleCancelSearch = async () => {
  try {
    await invoke("cancel_search");
    isSearching.value = false;
  } catch (err) {
    console.error("取消搜索失败:", err);
  }
};

// 点击左侧某处匹配项
const handleSelectMatch = (payload: { fileResult: FileMatchResult; match: MatchItem }) => {
  selectedFile.value = payload.fileResult;
  selectedMatch.value = payload.match;
};

// 右侧 Next / Prev 按钮切换匹配项
const handleMatchChange = (match: MatchItem) => {
  selectedMatch.value = match;
};

// 初始化 Tauri 事件监听通道
onMounted(async () => {
  // 1. 批量接收搜索结果流
  unlistenBatch = await listen<FileMatchResult[]>("search-result-batch", (event) => {
    const newItems = event.payload;
    if (!newItems || !newItems.length) return;

    searchResults.value.push(...newItems);

    // 如果当前尚未打开任何预览，默认自动选中第一个命中的文件与第一处匹配
    if (!selectedFile.value && searchResults.value.length > 0) {
      const firstFile = searchResults.value[0];
      selectedFile.value = firstFile;
      if (firstFile.matches.length > 0) {
        selectedMatch.value = firstFile.matches[0];
      }
    }
  });

  // 2. 接收实时搜索进度更新
  unlistenProgress = await listen<SearchProgress>("search-progress", (event) => {
    progress.value = event.payload;
  });

  // 3. 接收搜索完成事件
  unlistenFinished = await listen<SearchProgress>("search-finished", (event) => {
    isSearching.value = false;
    progress.value = event.payload;
  });
});

onBeforeUnmount(() => {
  if (unlistenBatch) unlistenBatch();
  if (unlistenProgress) unlistenProgress();
  if (unlistenFinished) unlistenFinished();
});
</script>