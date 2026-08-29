import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export type ThemeKey = "dark-slate" | "midnight-blue" | "obsidian-gold" | "light-crisp";

export interface ThemeOption {
  id: ThemeKey;
  labelKey: "themeDarkSlate" | "themeMidnightBlue" | "themeObsidianGold" | "themeLightCrisp";
  icon: string;
  dotColor: string;
}

export const themeOptions: ThemeOption[] = [
  { id: "dark-slate", labelKey: "themeDarkSlate", icon: "🌌", dotColor: "#38bdf8" },
  { id: "midnight-blue", labelKey: "themeMidnightBlue", icon: "🌊", dotColor: "#6366f1" },
  { id: "obsidian-gold", labelKey: "themeObsidianGold", icon: "🪙", dotColor: "#f59e0b" },
  { id: "light-crisp", labelKey: "themeLightCrisp", icon: "☀️", dotColor: "#0284c7" },
];

const savedTheme = (localStorage.getItem("flashtext_theme") as ThemeKey) || "dark-slate";
export const currentTheme = ref<ThemeKey>(savedTheme);

export const syncWindowAppearance = (theme?: ThemeKey, lang?: string) => {
  const t = theme || currentTheme.value;
  const l = lang || localStorage.getItem("flashtext_lang") || "zh";

  const subtitles: Record<string, string> = {
    zh: "极速文本与文档搜索",
    ko: "초고속 텍스트 & 문서 검색",
    en: "Ultra-Fast Text & Document Search",
  };

  const title = `FlashText Search v0.6.1 - ${subtitles[l] || subtitles.zh}`;
  const themeMode = t === "light-crisp" ? "light" : "dark";

  invoke("update_window_theme_and_title", { theme: themeMode, title }).catch((err: any) => {
    console.warn("更新窗口主题与标题失败:", err);
  });
};

export const applyTheme = (theme: ThemeKey) => {
  currentTheme.value = theme;
  localStorage.setItem("flashtext_theme", theme);
  document.documentElement.setAttribute("data-theme", theme);
  syncWindowAppearance(theme);
};

// 初始化主题
applyTheme(savedTheme);
