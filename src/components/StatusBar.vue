<template>
  <div class="relative border-t shrink-0 select-none shadow-xs flex flex-col" style="background-color: var(--bg-statusbar); border-color: var(--border-subtle)">
    <!-- 顶部极光微光进度条 -->
    <div v-if="isSearching || progress.progressPercent > 0" class="w-full h-1 overflow-hidden relative" style="background-color: var(--bg-surface)">
      <div
        class="h-full bg-gradient-to-r from-sky-400 via-indigo-500 to-amber-400 transition-all duration-150 ease-out relative"
        :style="{ width: `${progress.progressPercent}%` }"
      >
        <!-- 进度条光效流动 -->
        <div
          v-if="isSearching"
          class="absolute inset-0 bg-gradient-to-r from-transparent via-white/40 to-transparent animate-[shimmer_1.5s_infinite]"
        ></div>
      </div>
    </div>

    <!-- 状态栏主体内容 -->
    <div class="px-4 py-1.5 flex items-center justify-between text-xs" style="color: var(--text-muted)">
      <!-- 左侧：实时运行状态与进度百分比 -->
      <div class="flex items-center gap-4">
        <!-- 运行状态指示点与百分比徽章 -->
        <div class="flex items-center gap-2">
          <span
            class="w-2 h-2 rounded-full shrink-0"
            :class="{
              'bg-emerald-500 shadow-xs shadow-emerald-500/50': !isSearching && progress.isFinished,
              'bg-amber-500 animate-ping': isSearching,
              'bg-slate-400': !isSearching && !progress.isFinished
            }"
          ></span>
          <span class="font-bold" style="color: var(--text-title)">
            {{ getStatusText() }}
          </span>

          <!-- 百分比指示徽章 -->
          <span
            v-if="progress.totalFiles > 0"
            class="px-1.5 py-0.2 rounded-md font-mono text-[11px] font-bold border"
            :style="
              isSearching
                ? 'background-color: rgba(245, 158, 11, 0.15); color: #d97706; border-color: rgba(245, 158, 11, 0.35);'
                : 'background-color: rgba(16, 185, 129, 0.15); color: #059669; border-color: rgba(16, 185, 129, 0.3);'
            "
          >
            {{ progress.progressPercent.toFixed(1) }}%
          </span>
        </div>

        <!-- 扫描文件数比率 (如 1,450 / 2,130 文件) -->
        <div v-if="progress.totalFiles > 0" class="flex items-center gap-1">
          <Files class="w-3.5 h-3.5 text-sky-500" />
          <span>{{ t.scanned }}</span>
          <span class="font-mono font-bold" style="color: var(--text-title)">
            {{ formatNumber(progress.filesScanned) }}
          </span>
          <span class="font-mono opacity-70">/ {{ formatNumber(progress.totalFiles) }}</span>
        </div>

        <!-- 当前扫描文件名 (仅在扫描中显示) -->
        <div
          v-if="isSearching && progress.currentFile"
          class="hidden lg:flex items-center gap-1 text-[11px] max-w-[240px] truncate"
          style="color: var(--text-dim)"
        >
          <span>{{ t.scanningCurrent }}</span>
          <span class="font-mono truncate font-medium" style="color: var(--text-muted)" :title="progress.currentFile">
            {{ progress.currentFile }}
          </span>
        </div>

        <!-- 命中文件数 -->
        <div v-if="progress.filesMatched > 0" class="flex items-center gap-1">
          <FileCheck2 class="w-3.5 h-3.5 text-emerald-500" />
          <span>{{ t.matchedFiles }}</span>
          <span class="font-mono font-bold text-emerald-600 dark:text-emerald-300">{{ formatNumber(progress.filesMatched) }}</span>
        </div>

        <!-- 累计命中匹配数 -->
        <div v-if="progress.totalMatches > 0" class="flex items-center gap-1">
          <Target class="w-3.5 h-3.5 text-amber-500" />
          <span>{{ t.totalHits }}</span>
          <span class="font-mono font-bold text-amber-600 dark:text-amber-300">{{ formatNumber(progress.totalMatches) }}</span>
        </div>
      </div>

      <!-- 右侧：耗时、扫描速度与快捷键提示 -->
      <div class="flex items-center gap-4">
        <!-- 搜索耗时 -->
        <div v-if="progress.elapsedMs > 0" class="flex items-center gap-1 font-mono">
          <Timer class="w-3.5 h-3.5 text-indigo-500" />
          <span>{{ t.elapsedTime }}</span>
          <span class="font-semibold text-indigo-600 dark:text-indigo-300">
            {{ progress.elapsedMs < 1000 ? `${progress.elapsedMs} ms` : `${(progress.elapsedMs / 1000).toFixed(2)} s` }}
          </span>
        </div>

        <!-- 扫描吞吐速度 -->
        <div
          v-if="progress.filesScanned > 0 && progress.elapsedMs > 0"
          class="hidden sm:flex items-center gap-1 text-[11px] font-mono"
          style="color: var(--text-dim)"
        >
          <span>{{ t.speed }}</span>
          <span class="font-medium" style="color: var(--text-body)">
            {{ formatNumber(Math.round((progress.filesScanned / (progress.elapsedMs / 1000)))) }} {{ t.filesPerSec }}
          </span>
        </div>

        <!-- 快捷键提示 -->
        <div class="hidden md:flex items-center gap-2 text-[11px]" style="color: var(--text-dim)">
          <span>{{ t.f3Next }}</span>
          <span>•</span>
          <span>{{ t.shiftF3Prev }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Files, FileCheck2, Target, Timer } from "@lucide/vue";
import { t } from "../i18n";
import type { SearchProgress } from "../types/search";

const props = defineProps<{
  isSearching: boolean;
  progress: SearchProgress;
}>();

const getStatusText = () => {
  if (props.isSearching) return t.value.scanning;
  if (props.progress.isCancelled) return t.value.cancelled;
  if (props.progress.isFinished) return t.value.finished;
  return t.value.ready;
};

const formatNumber = (num: number) => {
  return num.toLocaleString();
};
</script>

