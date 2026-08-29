import { ref } from "vue";

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

export const applyTheme = (theme: ThemeKey) => {
  currentTheme.value = theme;
  localStorage.setItem("flashtext_theme", theme);
  document.documentElement.setAttribute("data-theme", theme);
};

// 初始化主题
applyTheme(savedTheme);
