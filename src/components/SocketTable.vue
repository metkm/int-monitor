<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

interface Payload {
  table: Table
}

interface Table {
  row_count: number,
  rows: TableRow[]
}

interface TableRow {
  local_addr: string,
  local_port: number,
  remote_addr: string,
  remote_port: number
}

const listener = ref<UnlistenFn | null>(null);
const table = ref<Table | null>();

onMounted(async () => {
  listener.value = await listen("table-update", (event) => {
    let payload = event.payload as Payload;
    table.value = payload.table;
  })

  invoke("init_process");
})

onUnmounted(() => listener.value?.())
</script>

<template>
  <div class="p-2">
    <table v-if="table" class="w-full rounded-lg overflow-hidden">
      <thead class="bg-neutral-200 dark:bg-neutral-800 border dark:border-neutral-800">
        <tr>
          <th class="p-2">Local Address</th>
          <th class="p-2">Local Port</th>
          <th class="p-2">Remote Address</th>
          <th class="p-2">Remote Port</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in table.rows" class="text-center border dark:border-neutral-800">
          <td class="p-2">{{ row.local_addr }}</td>
          <td class="p-2">{{ row.local_port }}</td>
          <td class="p-2">{{ row.remote_addr }}</td>
          <td class="p-2">{{ row.remote_port }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
