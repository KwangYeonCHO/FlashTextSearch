<template>
  <div class="relative bg-[#080c14] border-t border-white/10 shrink-0 select-none shadow-inner flex flex-col">
    <!-- 顶部极光微光进度条 -->
    <div v-if="isSearching || progress.progressPercent > 0" class="w-full h-1 bg-slate-800/80 overflow-hidden relative">
      <div
        class="h-full bg-gradient-to-r from-sky-400 via-indigo-500 to-amber-400 transition-all duration-150 ease-out relative"
        :style="{ width: `${progress.progressPercent}%` }"
      >
        <!-- 进度条光效流动 -->
        <div
          v-if="isSearching"
          class="absolute inset-0 bg-gradient-to-r from-transparent via-white/30 to-transparent animate-[shimmer_1.5s_infinite]"
        ></div>
      </div>
    </div>

    <!-- 状态栏主体内容 -->
    <div class="px-4 py-1.5 flex items-center justify-between text-xs text-slate-400">
      <!-- 左侧：实时运行状态与进度百分比 -->
      <div class="flex items-center gap-4">
        <!-- 运行状态指示点与百分比徽章 -->
        <div class="flex items-center gap-2">
          <span
            class="w-2 h-2 rounded-full shrink-0"
            :class="{
              'bg-emerald-400 shadow-sm shadow-emerald-400/50': !isSearching && progress.isFinished,
              'bg-amber-400 animate-ping': isSearching,
              'bg-slate-500': !isSearching && !progress.isFinished
            }"
          ></span>
          <span class="font-medium text-slate-300">
            {{ getStatusText() }}
          </span>

          <!-- 百分比指示徽章 -->
          <span
            v-if="progress.totalFiles > 0"
            class="px-1.5 py-0.2 rounded-md font-mono text-[11px] font-bold"
            :class="
              isSearching
                ? 'bg-amber-500/20 text-amber-300 border border-amber-500/40 animate-pulse'
                : 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/30'
            "
          >
            {{ progress.progressPercent.toFixed(1) }}%
          </span>
        </div>

        <!-- 扫描文件数比率 (如 1,450 / 2,130 文件) -->
        <div v-if="progress.totalFiles > 0" class="flex items-center gap-1">
          <Files class="w-3.5 h-3.5 text-sky-400" />
          <span>{{ t.scanned }}</span>
          <span class="font-mono font-bold text-slate-200">
            {{ formatNumber(progress.filesScanned) }}
          </span>
          <span class="text-slate-500 font-mono">/ {{ formatNumber(progress.totalFiles) }}</span>
        </div>

        <!-- 当前扫描文件名 (仅在扫描中显示) -->
        <div
          v-if="isSearching && progress.currentFile"
          class="hidden lg:flex items-center gap-1 text-[11px] text-slate-500 max-w-[240px] truncate"
        >
          <span>{{ t.scanningCurrent }}</span>
          <span class="text-slate-400 font-mono truncate" :title="progress.currentFile">
            {{ progress.currentFile }}
          </span>
        </div>

        <!-- 命中文件数 -->
        <div v-if="progress.filesMatched > 0" class="flex items-center gap-1">
          <FileCheck2 class="w-3.5 h-3.5 text-emerald-400" />
          <span>{{ t.matchedFiles }}</span>
          <span class="font-mono font-bold text-emerald-300">{{ formatNumber(progress.filesMatched) }}</span>
        </div>

        <!-- 累计命中匹配数 -->
        <div v-if="progress.totalMatches > 0" class="flex items-center gap-1">
          <Target class="w-3.5 h-3.5 text-amber-400" />
          <span>{{ t.totalHits }}</span>
          <span class="font-mono font-bold text-amber-300">{{ formatNumber(progress.totalMatches) }}</span>
        </div>
      </div>

      <!-- 右侧：耗时、扫描速度与快捷键提示 -->
      <div class="flex items-center gap-4">
        <!-- 搜索耗时 -->
        <div v-if="progress.elapsedMs > 0" class="flex items-center gap-1 font-mono">
          <Timer class="w-3.5 h-3.5 text-indigo-400" />
          <span>{{ t.elapsedTime }}</span>
          <span class="text-indigo-300 font-semibold">
            {{ progress.elapsedMs < 1000 ? `${progress.elapsedMs} ms` : `${(progress.elapsedMs / 1000).toFixed(2)} s` }}
          </span>
        </div>

        <!-- 扫描吞吐速度 -->
        <div
          v-if="progress.filesScanned > 0 && progress.elapsedMs > 0"
          class="hidden sm:flex items-center gap-1 text-[11px] text-slate-500 font-mono"
        >
          <span>{{ t.speed }}</span>
          <span class="text-slate-300">
            {{ formatNumber(Math.round((progress.filesScanned / (progress.elapsedMs / 1000)))) }} {{ t.filesPerSec }}
          </span>
        </div>

        <!-- 快捷键提示 -->
        <div class="hidden md:flex items-center gap-2 text-[11px] text-slate-500">
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

