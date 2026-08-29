<template>
  <!-- 自动更新弹窗遮罩 -->
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md animate-[fadeIn_0.2s_ease-out]"
  >
    <div
      class="w-full max-w-md rounded-2xl border shadow-2xl p-6 relative overflow-hidden"
      style="background-color: var(--bg-card); border-color: var(--border-subtle); color: var(--text-title)"
    >
      <!-- 顶部发光微光线 -->
      <div class="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-sky-400 via-indigo-500 to-amber-400"></div>

      <!-- 弹窗头部 -->
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-2.5">
          <div class="p-2 rounded-xl bg-sky-500/15 text-sky-500 border border-sky-500/20">
            <Sparkles class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-bold text-base bg-gradient-to-r from-sky-500 to-indigo-500 bg-clip-text text-transparent">
              {{ t.newVersionAvailable }}
            </h3>
            <div class="flex items-center gap-2 text-xs mt-0.5" style="color: var(--text-muted)">
              <span>{{ t.currentVersion }} {{ updateInfo?.currentVersion }}</span>
              <span>→</span>
              <span class="font-bold text-emerald-500">{{ t.latestVersion }} {{ updateInfo?.latestVersion }}</span>
            </div>
          </div>
        </div>

        <button
          v-if="!isInstalling"
          class="p-1 rounded-lg transition hover:bg-slate-500/10 cursor-pointer"
          style="color: var(--text-muted)"
          @click="closeModal"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- 更新日志内容区域 -->
      <div class="my-4">
        <h4 class="text-xs font-semibold mb-1.5" style="color: var(--text-body)">
          {{ updateInfo?.releaseTitle }}
        </h4>
        <div
          class="max-h-48 overflow-y-auto rounded-xl p-3 text-xs font-mono border whitespace-pre-wrap leading-relaxed select-text"
          style="background-color: var(--bg-surface); border-color: var(--border-subtle); color: var(--text-muted)"
        >
          {{ updateInfo?.releaseNotes || '包含常规性能优化与问题修复。' }}
        </div>
      </div>

      <!-- 下载更新中的进度提示 -->
      <div v-if="isInstalling" class="my-4 p-3 rounded-xl border flex items-center gap-3 bg-amber-500/10 border-amber-500/30 text-amber-500 text-xs">
        <Loader2 class="w-4 h-4 animate-spin shrink-0" />
        <span class="font-medium animate-pulse">{{ t.updating }}</span>
      </div>

      <!-- 错误提示 -->
      <div v-if="errorMessage" class="my-2 p-2.5 rounded-xl border flex items-center gap-2 bg-rose-500/10 border-rose-500/30 text-rose-500 text-xs">
        <AlertCircle class="w-4 h-4 shrink-0" />
        <span>{{ errorMessage }}</span>
      </div>

      <!-- 操作按钮组 -->
      <div class="flex items-center justify-end gap-2.5 mt-5">
        <button
          v-if="!isInstalling"
          class="px-4 py-2 text-xs font-medium rounded-xl border transition cursor-pointer"
          style="background-color: var(--bg-surface); border-color: var(--border-subtle); color: var(--text-muted)"
          @click="closeModal"
        >
          {{ t.remindLater }}
        </button>

        <button
          class="px-5 py-2 text-xs font-bold text-white rounded-xl bg-gradient-to-r from-sky-500 to-indigo-600 hover:from-sky-400 hover:to-indigo-500 shadow-md shadow-sky-500/25 transition cursor-pointer flex items-center gap-1.5"
          :disabled="isInstalling"
          :class="{ 'opacity-60 cursor-not-allowed': isInstalling }"
          @click="handleInstallUpdate"
        >
          <Download v-if="!isInstalling" class="w-4 h-4" />
          <Loader2 v-else class="w-4 h-4 animate-spin" />
          <span>{{ isInstalling ? t.updating : t.updateNow }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Sparkles, Download, X, Loader2, AlertCircle } from "@lucide/vue";
import { t } from "../i18n";
import type { UpdateCheckResult } from "../types/search";

const props = defineProps<{
  updateInfo: UpdateCheckResult | null;
  isOpen: boolean;
}>();

const emit = defineEmits<{
  (e: "close"): void;
}>();

const isInstalling = ref(false);
const errorMessage = ref<string | null>(null);

const closeModal = () => {
  if (!isInstalling.value) {
    emit("close");
  }
};

const handleInstallUpdate = async () => {
  if (!props.updateInfo) return;

  isInstalling.value = true;
  errorMessage.value = null;

  try {
    await invoke("install_app_update", { tagName: props.updateInfo.latestVersion });
  } catch (err: any) {
    console.error("执行自动更新失败:", err);
    errorMessage.value = typeof err === "string" ? err : err.message || "更新失败，请重试";
    isInstalling.value = false;
  }
};
</script>
