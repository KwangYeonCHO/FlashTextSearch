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
        <div class="flex items-center justify-between mb-1.5">
          <h4 class="text-xs font-semibold truncate max-w-[220px]" style="color: var(--text-body)">
            {{ displayReleaseTitle }}
          </h4>

          <!-- 多语言日志切换标签 -->
          <div v-if="hasMultiLangNotes" class="flex items-center gap-1">
            <button
              v-for="l in availableLangs"
              :key="l.key"
              class="px-1.5 py-0.5 rounded text-[10px] font-medium transition cursor-pointer"
              :class="activeNoteLang === l.key ? 'bg-sky-500/20 text-sky-400 font-bold border border-sky-500/30' : 'text-slate-400 hover:text-slate-200'"
              @click="activeNoteLang = l.key"
            >
              {{ l.icon }}
            </button>
          </div>
        </div>

        <div
          class="max-h-48 overflow-y-auto rounded-xl p-3 text-xs font-mono border whitespace-pre-wrap leading-relaxed select-text"
          style="background-color: var(--bg-surface); border-color: var(--border-subtle); color: var(--text-muted)"
        >
          {{ displayReleaseNotes }}
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
import { ref, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Sparkles, Download, X, Loader2, AlertCircle } from "@lucide/vue";
import { t, currentLang, type LanguageKey } from "../i18n";
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

// 当前更新日志显示的语言，默认跟随软件界面语言
const activeNoteLang = ref<LanguageKey>(currentLang.value);

watch(
  () => currentLang.value,
  (newLang) => {
    activeNoteLang.value = newLang;
  }
);

watch(
  () => props.isOpen,
  (open) => {
    if (open) {
      activeNoteLang.value = currentLang.value;
      errorMessage.value = null;
      isInstalling.value = false;
    }
  }
);

const availableLangs = [
  { key: "zh" as LanguageKey, icon: "🇨🇳 中文" },
  { key: "ko" as LanguageKey, icon: "🇰🇷 한국어" },
  { key: "en" as LanguageKey, icon: "🇺🇸 English" },
];

/**
 * 智能解析多语言 Release 内容
 */
const parseMultilingualContent = (raw: string | undefined, targetLang: LanguageKey): string => {
  if (!raw) return "";

  // 1. 优先提取 <!-- lang:ko --> ... <!-- /lang:ko --> 标签内容
  const tagRegex = new RegExp(`<!--\\s*lang:${targetLang}\\s*-->([\\s\\S]*?)<!--\\s*/lang:${targetLang}\\s*-->`, "i");
  const tagMatch = raw.match(tagRegex);
  if (tagMatch && tagMatch[1].trim()) {
    return tagMatch[1].trim();
  }

  // 2. 匹配 Markdown 分段标题，如 ## 🇰🇷 或 ## 한국어
  const sections = splitMarkdownByLangHeaders(raw);
  if (sections[targetLang]) {
    return sections[targetLang].trim();
  }

  // 3. 兜底提取：如果没有该语言的独立段落，直接返回全文
  return raw.trim();
};

/**
 * 判断 Release Notes 是否包含多语言标记
 */
const hasMultiLangNotes = computed(() => {
  const raw = props.updateInfo?.releaseNotes || "";
  return (
    raw.includes("<!-- lang:") ||
    raw.includes("🇰🇷") ||
    raw.includes("🇨🇳") ||
    raw.includes("🇺🇸") ||
    raw.includes("[KO]") ||
    raw.includes("[ZH]") ||
    raw.includes("[EN]")
  );
});

/**
 * 分解 Markdown 中的多语言段落
 */
const splitMarkdownByLangHeaders = (text: string): Record<string, string> => {
  const result: Record<string, string> = {};
  const lines = text.split("\n");
  let currentLangKey: string | null = null;
  let currentBuffer: string[] = [];

  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed.includes("🇰🇷") || trimmed.toLowerCase().includes("[ko]") || trimmed.includes("한국어")) {
      if (currentLangKey && currentBuffer.length) {
        result[currentLangKey] = currentBuffer.join("\n");
      }
      currentLangKey = "ko";
      currentBuffer = [line];
    } else if (trimmed.includes("🇨🇳") || trimmed.toLowerCase().includes("[zh]") || trimmed.includes("中文")) {
      if (currentLangKey && currentBuffer.length) {
        result[currentLangKey] = currentBuffer.join("\n");
      }
      currentLangKey = "zh";
      currentBuffer = [line];
    } else if (trimmed.includes("🇺🇸") || trimmed.toLowerCase().includes("[en]") || trimmed.includes("English")) {
      if (currentLangKey && currentBuffer.length) {
        result[currentLangKey] = currentBuffer.join("\n");
      }
      currentLangKey = "en";
      currentBuffer = [line];
    } else if (currentLangKey) {
      currentBuffer.push(line);
    }
  }

  if (currentLangKey && currentBuffer.length) {
    result[currentLangKey] = currentBuffer.join("\n");
  }

  return result;
};

// 当前展示的 Release 标题
const displayReleaseTitle = computed(() => {
  const raw = props.updateInfo?.releaseTitle || "";
  const parsed = parseMultilingualContent(raw, activeNoteLang.value);
  return parsed || raw || `${props.updateInfo?.latestVersion || ""} Update`;
});

// 当前展示的 Release 更新说明
const displayReleaseNotes = computed(() => {
  const raw = props.updateInfo?.releaseNotes || "";
  const parsed = parseMultilingualContent(raw, activeNoteLang.value);
  return parsed || raw || "包含常规性能优化与问题修复。";
});

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
