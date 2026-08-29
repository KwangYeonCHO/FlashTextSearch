<template>
  <div class="px-4 py-1.5 bg-[#080c14] border-t border-white/10 shrink-0 flex items-center justify-between text-xs text-slate-400 select-none shadow-inner">
    <!-- 左侧：实时统计指标 -->
    <div class="flex items-center gap-4">
      <!-- 运行状态指示点 -->
      <div class="flex items-center gap-1.5">
        <span
          class="w-2 h-2 rounded-full"
          :class="{
            'bg-emerald-400 shadow-sm shadow-emerald-400/50': !isSearching && progress.isFinished,
            'bg-amber-400 animate-ping': isSearching,
            'bg-slate-500': !isSearching && !progress.isFinished
          }"
        ></span>
        <span class="font-medium text-slate-300">
          {{ getStatusText() }}
        </span>
      </div>

      <!-- 扫描文件数 -->
      <div v-if="progress.filesScanned > 0" class="flex items-center gap-1">
        <Files class="w-3.5 h-3.5 text-sky-400" />
        <span>已扫描:</span>
        <span class="font-mono font-bold text-slate-200">{{ formatNumber(progress.filesScanned) }}</span>
      </div>

      <!-- 命中文件数 -->
      <div v-if="progress.filesMatched > 0" class="flex items-center gap-1">
        <FileCheck2 class="w-3.5 h-3.5 text-emerald-400" />
        <span>匹配文件:</span>
        <span class="font-mono font-bold text-emerald-300">{{ formatNumber(progress.filesMatched) }}</span>
      </div>

      <!-- 累计命中匹配数 -->
      <div v-if="progress.totalMatches > 0" class="flex items-center gap-1">
        <Target class="w-3.5 h-3.5 text-amber-400" />
        <span>总匹配项:</span>
        <span class="font-mono font-bold text-amber-300">{{ formatNumber(progress.totalMatches) }}</span>
      </div>
    </div>

    <!-- 右侧：耗时、扫描速度与快捷键提示 -->
    <div class="flex items-center gap-4">
      <!-- 搜索耗时 -->
      <div v-if="progress.elapsedMs > 0" class="flex items-center gap-1 font-mono">
        <Timer class="w-3.5 h-3.5 text-indigo-400" />
        <span>耗时:</span>
        <span class="text-indigo-300 font-semibold">
          {{ progress.elapsedMs < 1000 ? `${progress.elapsedMs} ms` : `${(progress.elapsedMs / 1000).toFixed(2)} s` }}
        </span>
      </div>

      <!-- 扫描吞吐速度 -->
      <div
        v-if="progress.filesScanned > 0 && progress.elapsedMs > 0"
        class="hidden sm:flex items-center gap-1 text-[11px] text-slate-500 font-mono"
      >
        <span>速度:</span>
        <span class="text-slate-300">
          {{ formatNumber(Math.round((progress.filesScanned / (progress.elapsedMs / 1000)))) }} 文件/秒
        </span>
      </div>

      <!-- 快捷键提示 -->
      <div class="hidden md:flex items-center gap-2 text-[11px] text-slate-500">
        <span>F3 下一个匹配</span>
        <span>•</span>
        <span>Shift+F3 上一个匹配</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Files, FileCheck2, Target, Timer } from "@lucide/vue";
import type { SearchProgress } from "../types/search";

const props = defineProps<{
  isSearching: boolean;
  progress: SearchProgress;
}>();

const getStatusText = () => {
  if (props.isSearching) return "极速扫描中...";
  if (props.progress.isCancelled) return "已取消";
  if (props.progress.isFinished) return "搜索完成";
  return "就绪";
};

const formatNumber = (num: number) => {
  return num.toLocaleString();
};
</script>
