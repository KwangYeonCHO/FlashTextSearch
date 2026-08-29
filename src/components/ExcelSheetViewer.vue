<template>
  <div class="h-full w-full flex flex-col bg-[#0b101b] overflow-hidden text-xs">
    <!-- 工作表选项卡栏 (Sheet Tabs) -->
    <div class="flex items-center gap-1 px-3 py-1.5 bg-slate-900 border-b border-white/10 shrink-0 overflow-x-auto">
      <button
        v-for="sheet in workbook.sheets"
        :key="sheet.sheetName"
        class="px-3 py-1 rounded-t-lg font-medium transition cursor-pointer flex items-center gap-1.5 shrink-0 border-t border-x"
        :class="
          activeSheetName === sheet.sheetName
            ? 'bg-slate-800 text-emerald-300 border-emerald-500/50 shadow'
            : 'bg-slate-900/60 text-slate-400 border-transparent hover:text-slate-200 hover:bg-slate-800/40'
        "
        @click="activeSheetName = sheet.sheetName"
      >
        <SheetIcon class="w-3.5 h-3.5" />
        <span>{{ sheet.sheetName }}</span>
        <span class="text-[10px] text-slate-500">({{ sheet.totalRows }} {{ t.sheetRows }})</span>
      </button>
    </div>

    <!-- 表格滚动视图 -->
    <div ref="tableContainer" class="flex-1 overflow-auto bg-slate-950/80">
      <table v-if="currentSheet" class="border-collapse w-full table-fixed text-slate-300 font-mono select-text">
        <!-- 列头 (A, B, C, D...) -->
        <thead class="sticky top-0 z-10 bg-slate-900 shadow-sm">
          <tr>
            <th class="w-12 py-1.5 px-2 bg-slate-900 border border-slate-800 text-slate-500 text-center font-normal sticky left-0 z-20">
              #
            </th>
            <th
              v-for="colIdx in currentSheet.maxCols"
              :key="colIdx"
              class="w-32 py-1.5 px-2 bg-slate-900 border border-slate-800 text-slate-400 text-center font-medium truncate"
            >
              {{ getColumnLetter(colIdx - 1) }}
            </th>
          </tr>
        </thead>

        <!-- 表格数据行 -->
        <tbody>
          <tr
            v-for="(row, rowIdx) in currentSheet.rows"
            :id="`excel-row-${rowIdx + 1}`"
            :key="rowIdx"
            class="hover:bg-slate-800/30 transition-colors"
          >
            <!-- 行号列 (1, 2, 3...) -->
            <td
              class="py-1 px-2 bg-slate-900/90 border border-slate-800 text-slate-500 text-center font-normal sticky left-0 z-10 select-none"
              :class="{
                'bg-amber-500/20 text-amber-300 font-bold': targetRow === rowIdx + 1
              }"
            >
              {{ rowIdx + 1 }}
            </td>

            <!-- 各单元格 -->
            <td
              v-for="colIdx in currentSheet.maxCols"
              :key="colIdx"
              class="py-1.5 px-2 border border-slate-800/80 truncate transition-all max-w-[200px]"
              :class="{
                'table-active-cell': isTargetCell(rowIdx + 1, colIdx)
              }"
              :title="row[colIdx - 1] || ''"
            >
              {{ row[colIdx - 1] || "" }}
            </td>
          </tr>
        </tbody>
      </table>

      <!-- 空表格提示 -->
      <div v-else class="h-full flex items-center justify-center text-slate-500">
        {{ t.noSheetData }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from "vue";
import { Table as SheetIcon } from "@lucide/vue";
import { t } from "../i18n";
import type { ExcelWorkbookContent } from "../types/search";

const props = defineProps<{
  workbook: ExcelWorkbookContent;
  targetSheet?: string;
  targetRow?: number;
  targetCol?: number;
}>();

const activeSheetName = ref<string>("");
const tableContainer = ref<HTMLElement | null>(null);

// 当前选中的工作表数据
const currentSheet = computed(() => {
  if (!props.workbook.sheets.length) return null;
  return (
    props.workbook.sheets.find((s) => s.sheetName === activeSheetName.value) ||
    props.workbook.sheets[0]
  );
});

// 列索引转英文字母 (0 -> "A", 1 -> "B", 26 -> "AA")
const getColumnLetter = (colIdx: number): string => {
  let result = "";
  let temp = colIdx;
  while (true) {
    const rem = temp % 26;
    result = String.fromCharCode(65 + rem) + result;
    if (temp < 26) break;
    temp = Math.floor(temp / 26) - 1;
  }
  return result;
};

// 判断是否为当前高亮目标单元格
const isTargetCell = (rowNum: number, colNum: number): boolean => {
  if (!props.targetRow) return false;
  const isRowMatch = props.targetRow === rowNum;
  if (!props.targetCol) return isRowMatch;
  return isRowMatch && props.targetCol === colNum;
};

// 滚动定位到目标行
const scrollToTargetRow = async (rowNum: number) => {
  await nextTick();
  const rowEl = document.getElementById(`excel-row-${rowNum}`);
  if (rowEl && tableContainer.value) {
    rowEl.scrollIntoView({ behavior: "smooth", block: "center" });
  }
};

// 监听目标 Sheet 与行号变化
watch(
  () => [props.workbook, props.targetSheet, props.targetRow],
  ([_newWb, newSheet, newRow]) => {
    if (newSheet && typeof newSheet === "string") {
      activeSheetName.value = newSheet;
    } else if (props.workbook.sheets.length && !activeSheetName.value) {
      activeSheetName.value = props.workbook.sheets[0].sheetName;
    }

    if (newRow && typeof newRow === "number") {
      scrollToTargetRow(newRow);
    }
  },
  { immediate: true }
);
</script>
